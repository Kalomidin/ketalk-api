use serde::{Deserialize, Serialize};

type Timestamp = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NewUserRequest {
  pub name: String,
  pub phone_number: String,
  pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct NewUserResponse {
  pub id: i64,
  pub name: String,
  pub phone_number: String,
  pub auth_token: String,
  pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GetUserResponse {
  pub id: i64,
  pub name: String,
  pub phone_number: String,
  pub avatar: String,
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
  pub item_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreateRoomResponse {
  pub secondary_user_id: i64,
  pub item_id: i64,
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
  pub title: String,
  pub last_message: String,
  pub last_message_time: chrono::NaiveDateTime,
  pub room_id: i64,
  pub last_message_sender_id: i64,
  pub secondary_user_image_url: String,
  pub item_id: i64,
  pub item_image_url: String,
  pub is_message_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SignInRequest {
  pub phone_number: String,
  pub password: String,
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
  pub title: String,
  pub description: String,
  pub negotiable: bool,
  pub price: i64,
  pub size: f64,
  pub weight: f64,
  pub karat_id: i64,
  pub category_id: i64,
  pub geofence_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreateItemResponse {
  pub id: i64,
  pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ItemImagesUpdateStatusToUploadedRequest {
  pub ids: Vec<i64>,
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
  pub title: String,
  pub description: String,
  pub price: i64,
  pub owner_id: i64,
  pub favorite_count: i32,
  pub message_count: i32,
  pub seen_count: i32,
  pub item_status: ItemStatus,
  pub created_at: Timestamp,
  pub thumbnail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ItemResponse {
  pub id: i64,
  pub title: String,
  pub description: String,
  pub price: i64,
  pub negotiable: bool,
  pub owner: ItemOwner,
  pub item_status: ItemStatus,
  pub is_hidden: bool,
  pub favorite_count: i32,
  pub message_count: i32,
  pub seen_count: i32,
  pub is_user_favorite: bool,
  pub images: Vec<String>,
  pub location: Option<Location>,
  pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ItemOwner {
  pub id: i64,
  pub name: String,
  pub location: Option<Location>,
  pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Location {
  pub latitude: f64,
  pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserItems {
  pub items: Vec<UserItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserItem {
  pub id: i64,
  pub item_name: String,
  pub image: String,
  pub price: i64,
  pub favorite_count: i32,
  pub message_count: i32,
  pub item_status: ItemStatus,
  pub is_hidden: bool,
  pub created_at: Timestamp,
  pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemStatus {
  Active,
  Sold,
  Reserved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateItemStatusRequest {
  pub new_item_status: ItemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HideUnhideItemRequest {
  pub is_hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateCategoryRequest {
  pub name: String,
  pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreatePresignedUrlResponse {
  pub url: String,
  pub image_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateProfileRequest {
  pub image: Option<String>,
  pub name: Option<String>,
}
