use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::helpers::new_naive_date;
use crate::schema::room::dsl as room_dsl;
use crate::schema::room_member as room_member_table;
use crate::schema::room_member::dsl::*;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = room_member_table)]
pub struct InsertRoomMember {
  pub room_id: i64,
  pub member_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RoomMember {
  pub id: i64,
  pub room_id: i64,
  pub member_id: i64,
  pub created_at: chrono::NaiveDateTime,
  pub last_joined_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RoomNameWithMember {
  pub room_id: i64,
  pub item_id: Option<i64>,
  pub member_id: i64,
  pub last_joined_at: chrono::NaiveDateTime,
}

pub fn create_new_room_member(
  conn: &mut PgConnection,
  rid: &i64,
  mid: &i64,
) -> Result<RoomMember, DieselError> {
  let new_room_member: InsertRoomMember = InsertRoomMember {
    room_id: rid.to_owned(),
    member_id: mid.to_owned(),
  };
  match get_room_member(conn, mid, rid) {
    Ok(rm) => Ok(rm),
    Err(_) => {
      let resp = diesel::insert_into(room_member)
        .values(&new_room_member)
        .get_result::<RoomMember>(conn)?;
      return Ok(resp);
    }
  }
}

pub fn get_rooms_by_user_id(
  conn: &mut PgConnection,
  mid: &i64,
) -> Result<Vec<RoomNameWithMember>, DieselError> {
  let cnv = room_member
    .inner_join(room_dsl::room)
    .select((room_dsl::id, room_dsl::item_id, member_id, last_joined_at))
    .filter(
      member_id
        .eq(mid)
        .and(deleted_at.is_null())
        .and(room_dsl::deleted_at.is_null())
        .and(room_dsl::id.eq(room_id)),
    )
    .load(conn)
    .optional()?;
  match cnv {
    Some(cnv) => Ok(cnv),
    None => Err(diesel::result::Error::NotFound),
  }
}

pub fn set_last_joined_at(
  conn: &mut PgConnection,
  mid: &i64,
  rid: &i64,
) -> Result<RoomMember, DieselError> {
  let resp = diesel::update(room_member.filter(member_id.eq(mid).and(room_id.eq(rid))))
    .set(last_joined_at.eq(new_naive_date()))
    .get_result::<RoomMember>(conn)?;
  Ok(resp)
}

pub fn get_room_member(
  conn: &mut PgConnection,
  mid: &i64,
  rid: &i64,
) -> Result<RoomMember, DieselError> {
  let cnv = room_member
    .filter(
      member_id
        .eq(mid)
        .and(room_id.eq(rid))
        .and(deleted_at.is_null()),
    )
    .first(conn)
    .optional()?;
  match cnv {
    Some(cnv) => Ok(cnv),
    None => Err(diesel::result::Error::NotFound),
  }
}
