use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserRequest {
    pub user_name: String,
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct InsertUser {
    pub user_name: String,
    pub phone_number: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub phone_number: String,
}