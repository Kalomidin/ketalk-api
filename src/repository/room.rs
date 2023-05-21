use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::room as room_table;
use crate::schema::room::dsl::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = room_table)]
pub struct InsertRoom {
  created_by: i64,
  item_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Room {
  pub id: i64,
  pub item_id: Option<i64>,
  pub created_by: i64,
  pub created_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn create_new_room(
  conn: &mut PgConnection,
  creator: &i64,
  _item_id: &i64,
) -> Result<Room, DieselError> {
  let new_room = InsertRoom {
    created_by: creator.to_owned(),
    item_id: _item_id.to_owned(),
  };

  let resp = diesel::insert_into(room)
    .values(&new_room)
    .get_result::<Room>(conn)?;
  return Ok(resp);
}

pub fn get_room_by_id(conn: &mut PgConnection, room_id: &i64) -> Result<Room, DieselError> {
  let result = room
    .filter(id.eq(room_id).and(deleted_at.is_null()))
    .first::<Room>(conn)
    .optional()?;
  match result {
    Some(r) => Ok(r),
    None => Err(diesel::result::Error::NotFound),
  }
}

pub fn get_room_by_item_and_creator(
  conn: &mut PgConnection,
  primary_user_id: &i64,
  secondary_user_id: &i64,
  _item_id: &i64,
) -> Result<Room, DieselError> {
  let result = room
    .filter(
      item_id
        .eq(_item_id)
        .and(
          created_by
            .eq(primary_user_id)
            .or(created_by.eq(secondary_user_id)),
        )
        .and(deleted_at.is_null()),
    )
    .first::<Room>(conn)
    .optional()?;
  match result {
    Some(r) => Ok(r),
    None => Err(diesel::result::Error::NotFound),
  }
}
