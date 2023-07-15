use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::users as user_table;
use crate::schema::users::dsl::*;

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = user_table)]
pub struct InsertUser {
  pub name: String,
  pub phone_number: String,
  pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub password: String,
  pub phone_number: String,
  pub cover_image: Option<String>,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

pub fn insert_new_user(
  conn: &mut PgConnection,
  nm: &str,
  pn: &str,
  _password: &str,
) -> Result<User, DieselError> {
  let new_user = InsertUser {
    name: nm.to_owned(),
    phone_number: pn.to_owned(),
    password: _password.to_owned(),
  };

  let resp = diesel::insert_into(users)
    .values(&new_user)
    .get_result::<User>(conn)?;
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

pub fn get_user_by_phone_number(
  conn: &mut PgConnection,
  pnumber: &str,
) -> Result<User, DieselError> {
  let user = users
    .filter(phone_number.eq(pnumber))
    .first::<User>(conn)
    .optional()?;
  match user {
    Some(user) => Ok(user),
    None => Err(diesel::result::Error::NotFound),
  }
}

pub fn update_profile(
  conn: &mut PgConnection,
  user_id: i64,
  new_cover_image: Option<String>,
  new_name: Option<String>,
) -> Result<(), DieselError> {
  let result = diesel::update(users)
    .filter(id.eq(user_id))
    .set((
      new_cover_image.map(|value| cover_image.eq(value)),
      new_name.map(|value| name.eq(value)),
      updated_at.eq(chrono::Local::now().naive_local()),
    ))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}

pub fn delete_cover_image(
  conn: &mut PgConnection,
  user_id: i64,
) -> Result<(), DieselError> {
  let deleted_cover_image: Option<String> = None;
  let result = diesel::update(users)
    .filter(id.eq(user_id))
    .set((
      cover_image.eq(deleted_cover_image),
      updated_at.eq(chrono::Local::now().naive_local()),
    ))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}
