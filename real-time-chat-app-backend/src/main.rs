mod presence;
mod websocket;
use crate::presence::PresenceActor;
use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, verify};
use chrono::{DateTime, Utc};
use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;
use websocket::chat_route;
use websocket::ChatServer;

// Define shared app state
struct AppState {
    db_pool: Pool<SqliteConnectionManager>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub status: String,
    pub last_seen: String, // Changed from DateTime<Utc> to String
}

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (user ID)
    exp: usize,  // Expiration time
}

// Request and Response structures
#[derive(Debug, Serialize, Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SigninRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    status: String,
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserResponse {
    id: i32,
    username: String,
    status: String,
    last_seen: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
struct ChatRoom {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserWithRooms {
    user: User,
    chat_rooms: Vec<ChatRoom>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoomRequest {
    pub name: String, // Make this field public
}

#[derive(Debug, Deserialize)]
struct JoinRoomRequest {
    user_id: i32,
    room_id: i32,
}

#[derive(Debug, Serialize, Deserialize)] // Add Serialize for JSON response
pub struct SigninResponse {
    pub user_id: i32, // Include user ID in the backend response
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoomRequest {
    pub name: String,
    pub user_id: i32,
}

async fn signup_user(
    state: web::Data<AppState>,
    payload: web::Json<SignupRequest>,
) -> impl Responder {
    let pool = &state.db_pool;
    let conn = pool.get().expect("Failed to get DB connection");

    // Hash the password
    let hashed_password = match hash(&payload.password, 12) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    // Insert user into the database
    match conn.execute(
        "INSERT INTO users (username, password_hash, status, last_seen, createdAt, updatedAt) VALUES (?, ?, ?, ?, ?, ?)",
        params![
            &payload.username,
            &hashed_password,
            "offline",             // status (offline)
            Utc::now().to_string(),
            Utc::now().to_string(),
            Utc::now().to_string()
        ],
    ) {
        Ok(_) => {
            // Retrieve the last inserted row ID
            let user_id = conn.last_insert_rowid();

            // Construct and return the structured response
            let response = ApiResponse {
                status: "success".to_string(),
                data: UserData {
                    user: UserResponse {
                        id: user_id as i32, // Use the actual user ID
                        username: payload.username.clone(),
                        status: "offline".to_string(),
                        last_seen: Utc::now().to_rfc3339(),
                    },
                },
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                HttpResponse::BadRequest().body("Username already exists")
            } else {
                HttpResponse::InternalServerError().body("Failed to register user")
            }
        }
    }
}

// Handler: Sign in a user
async fn signin_user(
    state: web::Data<AppState>,
    payload: web::Json<SigninRequest>,
) -> impl Responder {
    let pool = &state.db_pool;
    let conn = pool.get().expect("Failed to get DB connection");

    // Fetch the user by username
    let mut stmt =
        match conn.prepare("SELECT id, username, password_hash FROM users WHERE username = ?") {
            Ok(s) => s,
            Err(_) => return HttpResponse::InternalServerError().body("Failed to prepare query"),
        };

    let user = stmt.query_row(params![&payload.username], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    });

    match user {
        Ok((id, username, hashed_password)) => {
            if verify(&payload.password, &hashed_password).unwrap_or(false) {
                HttpResponse::Ok().json(SigninResponse { user_id: id })
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

// Handler: Get user info
async fn get_user_info(
    state: web::Data<AppState>,
    path: web::Path<i32>, // Extract user ID from the path
) -> impl Responder {
    let pool = &state.db_pool;
    let conn = pool.get().expect("Failed to get DB connection");
    let user_id = path.into_inner();
    // Fetch user details
    let user = match conn.query_row(
        "SELECT id, username, status, last_seen FROM users WHERE id = ?",
        params![&user_id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                status: row.get(2)?,
                last_seen: row.get::<_, String>(3)?, // Keep it as String
            })
        },
    ) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::NotFound().body("User not found");
        }
    };

    // Fetch associated chat rooms
    let mut stmt = conn
        .prepare(
            "SELECT c.id, c.name
         FROM chat_rooms c
         JOIN user_chat_room_membership ucrm ON c.id = ucrm.room_id
         WHERE ucrm.user_id = ?",
        )
        .unwrap();

    let chat_rooms = stmt
        .query_map(params![user_id], |row| {
            Ok(ChatRoom {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<ChatRoom>>();

    // Return the user and associated chat rooms
    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        data: UserWithRooms {
            user,
            chat_rooms,
        },
    })
}

// API to create a new chat room
async fn create_chat_room(
    state: web::Data<AppState>,
    payload: web::Json<CreateRoomRequest>, // Updated to include `user_id`
) -> impl Responder {
    let pool = &state.db_pool;
    let conn = pool.get().expect("Failed to get DB connection");

    // Step 1: Create the chat room
    let result = conn.execute(
        "INSERT INTO chat_rooms (name) VALUES (?)",
        params![&payload.name],
    );

    let room_id = match result {
        Ok(_) => conn.last_insert_rowid() as i32,
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                return HttpResponse::BadRequest().body("Chat room already exists");
            } else {
                return HttpResponse::InternalServerError().body("Failed to create chat room");
            }
        }
    };

    // Step 2: Add the user to the chat room membership
    match conn.execute(
        "INSERT INTO user_chat_room_membership (user_id, room_id) VALUES (?, ?)",
        params![payload.user_id, room_id],
    ) {
        Ok(_) => HttpResponse::Ok().json(ChatRoom {
            id: room_id,
            name: payload.name.clone(),
        }),
        Err(_) => HttpResponse::InternalServerError().body("Failed to add user to the chat room"),
    }
}

// API to join a chat room by ID
async fn join_chat_room(
    state: web::Data<AppState>,
    payload: web::Json<JoinRoomRequest>,
) -> impl Responder {
    let pool = &state.db_pool;
    let conn = pool.get().expect("Failed to get DB connection");

    match conn.execute(
        "INSERT INTO user_chat_room_membership (user_id, room_id) VALUES (?, ?)",
        params![payload.user_id, payload.room_id],
    ) {
        Ok(_) => HttpResponse::Ok().body("Joined chat room successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to join chat room"),
    }
}

// API to delete a chat room
async fn delete_chat_room(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool = &state.db_pool;
    let conn = pool.get().expect("Failed to get DB connection");

    match conn.execute(
        "DELETE FROM chat_rooms WHERE id = ?",
        params![path.into_inner()],
    ) {
        Ok(_) => HttpResponse::Ok().body("Chat room deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete chat room"),
    }
}


async fn get_all_chat_rooms(state: web::Data<AppState>) -> impl Responder {
    let pool = &state.db_pool;
    let conn = pool.get().expect("Failed to get DB connection");


    let mut stmt = conn
        .prepare("SELECT id, name FROM chat_rooms")
        .expect("Failed to prepare statement");

    let chat_rooms = stmt
        .query_map([], |row| {
            Ok(ChatRoom {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<ChatRoom>>();

    HttpResponse::Ok().json(chat_rooms)
}


fn initialize_database(db_path: &str, schema_path: &str) {
    if !std::path::Path::new(db_path).exists() {
        println!("Database not found, creating a new one...");
        let conn = Connection::open(db_path).expect("Failed to create database");
        let schema = fs::read_to_string(schema_path).expect("Failed to read schema file");
        conn.execute_batch(&schema)
            .expect("Failed to execute schema");
    } else {
        println!("Database already exists.");
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    initialize_database("chatapp.db", "./sql/init.sql");
    // Set up the database connection pool
    let manager = SqliteConnectionManager::file("chatapp.db");
    let pool = Pool::new(manager).expect("Failed to create DB pool");
    let server = ChatServer::new().start();
    let chat_server = ChatServer::new().start();
    let presence_actor: Addr<PresenceActor> = PresenceActor::new(pool.clone()).start();
    // Start the Actix Web server
    info!("Starting server on http://127.0.0.1:8000");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allow all origins for testing; restrict in production
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(), // Allow credentials
            )
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(presence_actor.clone()))
            .route("/api/auth/signup", web::post().to(signup_user))
            .route("/api/auth/signin", web::post().to(signin_user))
            .route("/api/users/{id}", web::get().to(get_user_info))
            .route("/api/chat_rooms", web::post().to(create_chat_room))
            .route("/api/chat_rooms/join", web::post().to(join_chat_room))
            .route("/api/chat_rooms/{id}", web::delete().to(delete_chat_room))
            .route("/api/chat_rooms", web::get().to(get_all_chat_rooms))
            .route(
                "/ws/{room_id}/{user_id}",
                web::get().to(
                    move |req: HttpRequest,
                          stream: web::Payload,
                          path: web::Path<(i32, i32)>,
                          srv: web::Data<Addr<ChatServer>>,
                          presence: web::Data<Addr<PresenceActor>>| {
                        chat_route(req, stream, path, srv, presence)
                    },
                ),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
