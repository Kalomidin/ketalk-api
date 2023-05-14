use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::helpers::new_naive_date;
use crate::schema::item_document as document_table;
use crate::schema::item_document::dsl::*;
use diesel::result::Error as DieselError;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "document_table"]
pub struct InsertDocument {
  pub key: String,
  pub item_id: i64,
  pub user_id: i64,
  pub uploaded_to_cloud: bool,
  pub is_cover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Document {
  pub id: i64,
  // object name
  pub key: String,
  pub item_id: i64,
  pub user_id: i64,
  pub is_cover: bool,
  pub uploaded_to_cloud: bool,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub deleted_at: Option<NaiveDateTime>,
}

pub fn insert_new_document(
  conn: &mut PgConnection,
  uid: i64,
  iid: i64,
  object_name: String,
  uploaded: bool,
  _is_cover: bool,
) -> Result<Document, DieselError> {
  let new_document = InsertDocument {
    key: object_name,
    item_id: iid,
    user_id: uid,
    uploaded_to_cloud: uploaded,
    is_cover: _is_cover,
  };

  let resp = diesel::insert_into(item_document)
    .values(&new_document)
    .get_result::<Document>(conn)?;
  return Ok(resp);
}

pub fn set_to_uploaded_to_cloud(
  conn: &mut PgConnection,
  document_id: i64,
) -> Result<(), DieselError> {
  let result = diesel::update(item_document)
    .filter(deleted_at.is_null().and(id.eq(document_id)))
    .set(uploaded_to_cloud.eq(true))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}

pub fn get_docs_for_item(conn: &mut PgConnection, iid: i64) -> Result<Vec<Document>, DieselError> {
  let docs = item_document
    .filter(item_id.eq(iid).and(deleted_at.is_null()))
    .load(conn)
    .optional()?;
  match docs {
    Some(docs) => Ok(docs),
    None => Err(DieselError::NotFound),
  }
}
