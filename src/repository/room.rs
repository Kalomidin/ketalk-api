use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::room as room_table;
use crate::schema::room::dsl::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "room_table"]
pub struct InsertRoom {
  created_by: i64,
  name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Room {
  pub id: i64,
  pub name: String,
  pub created_by: i64,
  pub created_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn create_new_room(
  conn: &mut PgConnection,
  creator: &i64,
  room_name: &str,
) -> Result<Room, DieselError> {
  let new_room = InsertRoom {
    created_by: creator.to_owned(),
    name: room_name.to_owned(),
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

pub fn get_room_by_name_and_creator(
  conn: &mut PgConnection,
  primary_user_id: &i64,
  secondary_user_id: &i64,
  room_name: &str,
) -> Result<Room, DieselError> {
  let result = room
    .filter(
      name
        .eq(room_name)
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
