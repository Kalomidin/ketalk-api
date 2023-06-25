use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::karat as karat_table;
use crate::schema::karat::dsl::*;

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = karat_table)]
pub struct InsertKarat {
  pub name: String,
  pub description: String,
  pub gold_purity: i32,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct Karat {
  pub id: i64,
  pub name: String,
  pub description: String,
  pub gold_purity: i32,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn add_karat(
  conn: &mut PgConnection,
  _name: String,
  _description: String,
  _gold_purity: i32,
) -> Result<Karat, DieselError> {
  let new_karat = InsertKarat {
    name: _name,
    description: _description,
    gold_purity: _gold_purity,
  };
  let resp = diesel::insert_into(karat)
    .values(&new_karat)
    .get_result::<Karat>(conn)?;
  return Ok(resp);
}

pub fn get_by_name(conn: &mut PgConnection, _name: String) -> Result<Option<Karat>, DieselError> {
  let result = karat
    .filter(name.eq(_name))
    .first::<Karat>(conn)
    .optional()?;
  return Ok(result);
}

pub fn get_karats(conn: &mut PgConnection) -> Result<Vec<Karat>, DieselError> {
  let result = karat.load(conn).optional()?;
  match result {
    Some(karats) => Ok(karats),
    None => Ok(vec![]),
  }
}

pub fn delete_karat(conn: &mut PgConnection, _name: String) -> Result<(), DieselError> {
  let result = diesel::update(karat)
    .filter(name.eq(_name))
    .set(deleted_at.eq(chrono::Local::now().naive_local()))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}
