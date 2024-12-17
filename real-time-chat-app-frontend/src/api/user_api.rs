use super::types::{
    ApiResponse, ChatRoom, CreateRoomRequest, ErrorResponse, JoinRoomRequest, SigninResponse, User,
    UserData, UserResponse, UserWithRooms,
};
use reqwasm::http;
use serde_json::json;
pub async fn api_signup_user(user_data: &str) -> Result<User, String> {
    let response = match http::Request::post("http://localhost:8000/api/auth/signup")
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    // Log the raw response for debugging
    let text = response.text().await.unwrap_or_default();
    web_sys::console::log_1(&format!("Raw Response: {}", text).into());

    // Handle non-200 status codes
    if response.status() != 200 {
        let error_response = serde_json::from_str::<ErrorResponse>(&text).ok();
        if let Some(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    // Parse the valid response
    let parsed_response = serde_json::from_str::<ApiResponse<UserData>>(&text);
    match parsed_response {
        Ok(data) => Ok(data.data.user),
        Err(err) => {
            web_sys::console::log_1(&format!("Failed to parse JSON: {:?}", err).into());
            Err("Failed to parse response".to_string())
        }
    }
}

pub async fn api_signin_user(user_data: &str) -> Result<SigninResponse, String> {
    let response = match http::Request::post("http://localhost:8000/api/auth/signin")
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<SigninResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_user_info() -> Result<User, String> {
    let response = match http::Request::get("http://localhost:8000/api/users/me")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_signout_user(user_id: i32) -> Result<(), String> {
    let url = format!("http://localhost:8000/api/auth/signout/{}", user_id); // Interpolate user_id

    let response = match http::Request::post(&url)
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    // Handle non-200 responses
    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    Ok(())
}

// API Calls
pub async fn fetch_user_info(user_id: i32) -> Result<UserWithRooms, String> {
    let url = format!("http://localhost:8000/api/users/{}", user_id);
    match http::Request::get(&url).send().await {
        Ok(response) if response.ok() => response
            .json::<UserWithRooms>()
            .await
            .map_err(|e| e.to_string()),
        Ok(response) => Err(format!("Error: {}", response.status())),
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn create_chat_room(room_name: String, user_id: i32) -> Result<ChatRoom, String> {
    match http::Request::post("http://localhost:8000/api/chat_rooms")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&CreateRoomRequest {
                name: room_name,
                user_id,
            })
            .unwrap(),
        )
        .send()
        .await
    {
        Ok(response) if response.ok() => {
            response.json::<ChatRoom>().await.map_err(|e| e.to_string())
        }
        Ok(response) => Err(format!("Error: {}", response.status())),
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

pub async fn join_chat_room(user_id: i32, room_id: i32) -> Result<(), String> {
    let payload = json!({
        "user_id": user_id,
        "room_id": room_id
    });

    match http::Request::post("http://localhost:8000/api/chat_rooms/join")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&JoinRoomRequest { user_id, room_id }).unwrap())
        .send()
        .await
    {
        Ok(response) if response.ok() => Ok(()),
        Ok(response) => Err(format!("Error: {}", response.status())),
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}
