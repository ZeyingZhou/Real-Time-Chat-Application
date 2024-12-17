use chrono::prelude::*;
use yew::Properties;
use serde::{Deserialize, Serialize};

/// User Model
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub status: String,
    pub last_seen: String, // Changed from DateTime<Utc> to String
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Deserialize)]
pub struct SigninResponse {
    pub user_id: i32, // Include user ID in the backend response
}

/// Chat Room Model
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct ChatRoom {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct JoinRoomRequest {
    pub user_id: i32,
    pub room_id: i32,
}
/// Message Model
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Message {
    pub id: String,
    pub room_id: String,
    pub user_id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

/// User-Chat Room Membership
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct UserChatRoomMembership {
    pub user_id: String,
    pub room_id: String,
}

/// Response for a List of Chat Rooms
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoomListResponse {
    pub chat_rooms: Vec<ChatRoom>,
}

/// Response for a Single Chat Room
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoomResponse {
    pub chat_room: ChatRoom,
}

/// Response for Messages in a Chat Room
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageListResponse {
    pub messages: Vec<Message>,
}

/// Response for a Single Message
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    pub message: Message,
}

/// Request Payload for Creating a Chat Room
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatRoomRequest {
    pub name: String,
}

/// Request Payload for Sending a Message
#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageRequest {
    pub room_id: String,
    pub user_id: String,
    pub content: String,
}

/// Request Payload for Joining a Chat Room
#[derive(Serialize, Deserialize, Debug)]
pub struct JoinChatRoomRequest {
    pub room_id: String,
    pub user_id: String,
}

#[derive(Properties, PartialEq)]
pub struct ChatRoomProps {
    pub room_id: i32,
    pub user_id: i32,
}

/// Error Response
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserWithRooms {
    pub user: User,
    pub chat_rooms: Vec<ChatRoom>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoomRequest {
    pub name: String,
    pub user_id: i32,
}
