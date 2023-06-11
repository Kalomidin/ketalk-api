use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::category as category_table;
use crate::schema::category::dsl::*;

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = category_table)]
pub struct InsertCategory {
  pub name: String,
  pub avatar: String,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct Category {
  pub id: i64,
  pub name: String,
  pub avatar: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn add_category(
  conn: &mut PgConnection,
  _name: String,
  _avatar: String,
) -> Result<Category, DieselError> {
  let new_favor_item = InsertCategory {
    name: _name,
    avatar: _avatar,
  };
  let resp = diesel::insert_into(category)
    .values(&new_favor_item)
    .get_result::<Category>(conn)?;
  return Ok(resp);
}

pub fn get_by_name(
  conn: &mut PgConnection,
  _name: String,
) -> Result<Option<Category>, DieselError> {
  let result = category
    .filter(name.eq(_name))
    .first::<Category>(conn)
    .optional()?;
  return Ok(result);
}

pub fn get_categories(conn: &mut PgConnection) -> Result<Vec<Category>, DieselError> {
  let result = category.load(conn).optional()?;
  match result {
    Some(categories) => Ok(categories),
    None => Ok(vec![]),
  }
}

pub fn delete_category(conn: &mut PgConnection, _name: String) -> Result<(), DieselError> {
  let result = diesel::update(category)
    .filter(name.eq(_name))
    .set(deleted_at.eq(chrono::Local::now().naive_local()))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}
