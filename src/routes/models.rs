use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NewUserRequest {
    pub user_name: String,
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct NewUserResponse {
    pub user_id: i64,
    pub user_name: String,
    pub phone_number: String,
    pub auth_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GetUserResponse {
    pub id: i64,
    pub user_name: String,
    pub phone_number: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RefreshAuthTokenRequest {
    pub refresh_token: String,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct RefreshAuthTokenResponse {
    pub auth_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateRoomRequest {
    pub secondary_user_id: i64,
    pub room_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreateRoomResponse {
    pub secondary_user_id: i64,
    pub room_name: String,
    pub room_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GetUserRoomsResponse {
    pub rooms: Vec<UserRoom>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserRoom {
    pub room_name: String,
    pub last_message: String,
    pub last_message_time: String,
    pub last_message_sender_id: i64,
    pub room_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SiginRequest {
    pub user_name: String,
    pub phone_number: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreatePresignedUrlsRequest {
    pub file_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreatePresignedUrlsResponse {
    pub presigned_urls: Vec<PresignedUrl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PresignedUrl {
    pub key: String,
    pub url: String,
    pub file_name: String,
}