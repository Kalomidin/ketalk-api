use diesel::{
    prelude::*,
};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::users as user_table;
use crate::schema::users::dsl::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="user_table"]
pub struct InsertUser {
    pub user_name: String,
    pub phone_number: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub phone_number: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub fn insert_new_user(conn: &mut PgConnection, nm: &str, pn: &str) -> Result<User, DieselError> {

    let new_user = InsertUser {
        user_name: nm.to_owned(),
        phone_number: pn.to_owned(),
    };

    let resp = diesel::insert_into(users).values(&new_user).get_result::<User>(conn)?;
    return Ok(resp);
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: i64) -> Result<User, DieselError> {
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;
    match user {
        Some(user) => Ok(user),
        None => Err(diesel::result::Error::NotFound),
    }
}


pub fn get_user_by__username_and_phone_number(conn: &mut PgConnection, uname: &str, pnumber: &str) -> Result<User, DieselError> {
    let user = users
        .filter(user_name.eq(uname).and(phone_number.eq(pnumber)))
        .first::<User>(conn)
        .optional()?;
    match user {
        Some(user) => Ok(user),
        None => Err(diesel::result::Error::NotFound),
    }
}