use diesel::prelude::*;
use diesel::result::Error as DieselError;

use crate::helpers::new_naive_date;
use crate::schema::message as message_table;
use crate::schema::message::dsl::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = message_table)]
pub struct InsertMessage {
  pub room_id: i64,
  pub sender_id: i64,
  pub sender_name: String,
  pub msg: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Message {
  pub id: i64,
  pub room_id: i64,
  pub sender_id: i64,
  pub sender_name: String,
  pub msg: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn create_new_message(
  conn: &mut PgConnection,
  rid: &i64,
  sid: &i64,
  sname: &str,
  mes: &str,
) -> Result<Message, DieselError> {
  let dt = new_naive_date();
  let new_mes = InsertMessage {
    room_id: rid.to_owned(),
    sender_id: sid.to_owned(),
    sender_name: sname.to_owned(),
    msg: mes.to_owned(),
    created_at: dt,
    updated_at: dt,
  };

  let resp = diesel::insert_into(message)
    .values(&new_mes)
    .get_result::<Message>(conn)?;
  return Ok(resp);
}

pub fn create_new_message_with_date(
  conn: &mut PgConnection,
  rid: &i64,
  sid: &i64,
  sname: &str,
  mes: &str,
  dt: chrono::NaiveDateTime,
) -> Result<Message, DieselError> {
  let new_mes = InsertMessage {
    room_id: rid.to_owned(),
    sender_id: sid.to_owned(),
    sender_name: sname.to_owned(),
    msg: mes.to_owned(),
    created_at: dt,
    updated_at: dt,
  };
  let resp = diesel::insert_into(message)
    .values(&new_mes)
    .get_result::<Message>(conn)?;
  return Ok(resp);
}

pub fn get_messages_for_room_id(
  conn: &mut PgConnection,
  rid: &i64,
) -> Result<Vec<Message>, DieselError> {
  // TODO: Add pagination
  let cnv = message
    .filter(room_id.eq(rid).and(deleted_at.is_null()))
    .load(conn)
    .optional()?;
  match cnv {
    Some(cnv) => Ok(cnv),
    None => Err(DieselError::NotFound),
  }
}

pub fn get_last_message_by_room_id(
  conn: &mut PgConnection,
  rid: &i64,
) -> Result<Message, DieselError> {
  let cnv = message
    .filter(room_id.eq(rid).and(deleted_at.is_null()))
    .order(created_at.desc())
    .first(conn)
    .optional()?;
  match cnv {
    Some(cnv) => Ok(cnv),
    None => Err(DieselError::NotFound),
  }
}
