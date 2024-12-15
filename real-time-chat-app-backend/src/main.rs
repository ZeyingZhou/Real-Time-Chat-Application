use actix::{Actor, Addr, SyncArbiter};
use actix_web::{web, App, HttpServer, Responder};
use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;

mod presence;

use presence::PresenceActor;

// App State for Connection Pool
pub struct AppState {
    pub db: Pool<SqliteConnectionManager>,
    pub presence_actor: Addr<PresenceActor>,
}

// Example Data Structure for Chat Rooms
#[derive(Serialize)]
struct ChatRoom {
    id: i32,
    name: String,
    created_at: String,
}

// Handler to Fetch All Chat Rooms
async fn get_chat_rooms(data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.get().unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, created_at FROM chat_rooms")
        .unwrap();
    let chat_rooms_iter = stmt
        .query_map([], |row| {
            Ok(ChatRoom {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .unwrap();

    let mut chat_rooms = vec![];
    for room in chat_rooms_iter {
        chat_rooms.push(room.unwrap());
    }

    web::Json(chat_rooms)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();

    // Set up SQLite database connection pool
    let manager = SqliteConnectionManager::file("chatapp.db");
    let pool = Pool::new(manager).expect("Failed to create pool.");

    // Start the presence detection actor
    let pool_for_actor: Pool<SqliteConnectionManager> = pool.clone();
    let presence_actor = PresenceActor::new(pool_for_actor.clone()).start();

    // Run Actix Web server
    info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                presence_actor: presence_actor.clone(),
            }))
            .route("/chat_rooms", web::get().to(get_chat_rooms)) // Route for fetching chat rooms
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
