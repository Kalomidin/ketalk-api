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
pub struct CreateItemImagesRequest {
  pub item_id: i64,
  pub images: Vec<CreateItemImageRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateItemImageRequest {
  pub name: String,
  pub is_cover: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreateItemImagesResponse {
  pub images: Vec<ItemImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ItemImage {
  pub id: i64,
  pub key: String,
  pub url: String,
  pub is_cover: bool,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateItemRequest {
  pub description: String,
  pub details: String,
  pub price: i64,
  pub negotiable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreateItemResponse {
  pub id: i64,
  pub description: String,
  pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ItemImagesUpdateStatusToUploadedRequest {
  pub document_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GetItemsResponse {
  pub items: Vec<GetItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GetItemResponse {
  pub id: i64,
  pub details: String,
  pub description: String,
  pub price: i32,
  pub owner_id: i64,
  pub created_at: String,
  pub cover_image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ItemResponse {
  pub id: i64,
  pub details: String,
  pub description: String,
  pub price: i32,
  pub owner_id: i64,
  pub owner_name: String,
  pub owner_location: Option<Location>,
  pub owner_image_url: String,
  pub negotiable: bool,
  pub favorite_count: i64,
  pub message_count: i64,
  pub created_at: String,
  pub presigned_urls: Vec<String>,
  pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Location {
  pub latitude: f64,
  pub longitude: f64,
}
