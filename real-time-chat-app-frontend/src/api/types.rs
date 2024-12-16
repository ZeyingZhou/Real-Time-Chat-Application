use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub status: bool,
    pub last_seen: DateTime<Utc>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}