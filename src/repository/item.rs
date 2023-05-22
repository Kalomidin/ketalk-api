use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::item as item_table;
use crate::schema::item::dsl::*;

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = item_table)]
pub struct InsertItem {
  pub description: String,
  pub details: String,
  pub price: i64,
  pub negotiable: bool,
  pub owner_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct Item {
  pub id: i64,
  pub description: String,
  pub details: String,
  pub price: i64,
  pub negotiable: bool,
  pub owner_id: i64,
  pub item_status: String,
  pub favorite_count: i32,
  pub message_count: i32,
  pub seen_count: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub deleted_at: Option<NaiveDateTime>,
}

pub fn insert_new_item(
  conn: &mut PgConnection,
  _owner_id: i64,
  _description: String,
  _details: String,
  _price: i64,
  _negotiable: bool,
) -> Result<Item, DieselError> {
  let new_item = InsertItem {
    owner_id: _owner_id,
    description: _description,
    details: _details,
    price: _price,
    negotiable: _negotiable,
  };

  let resp = diesel::insert_into(item)
    .values(&new_item)
    .get_result(conn)?;
  return Ok(resp);
}

pub fn get_items_by_user_id(
  conn: &mut PgConnection,
  user_id: i64,
) -> Result<Vec<Item>, DieselError> {
  let result = item.filter(owner_id.eq(user_id)).load(conn).optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn get_item_by_id(conn: &mut PgConnection, item_id: i64) -> Result<Item, DieselError> {
  let result = item
    .filter(id.eq(item_id).and(deleted_at.is_null()))
    .first(conn)
    .optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Item>, DieselError> {
  let result = item
    .filter(deleted_at.is_null())
    .order(created_at.desc())
    .load(conn)
    .optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn increment_message_count(conn: &mut PgConnection, item_id: i64) -> Result<(), DieselError> {
  let result = diesel::update(item)
    .filter(deleted_at.is_null().and(id.eq(item_id)))
    .set(message_count.eq(message_count + 1))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}
