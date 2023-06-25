use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

use crate::schema::user_favorite as user_favorite_table;
use crate::schema::user_favorite::dsl::*;

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = user_favorite_table)]
pub struct InsertUserFavorite {
  pub user_id: i64,
  pub item_id: i64,
  pub is_favorite: bool,
}

#[derive(Clone, Serialize, Deserialize, Queryable)]
pub struct UserFavorite {
  pub id: i64,
  pub user_id: i64,
  pub item_id: i64,
  pub is_favorite: bool,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

pub fn add_item_favorite(
  conn: &mut PgConnection,
  _user_id: &i64,
  _item_id: &i64,
  _is_favorite: bool,
) -> Result<UserFavorite, DieselError> {
  let new_favor_item = InsertUserFavorite {
    user_id: _user_id.to_owned(),
    item_id: _item_id.to_owned(),
    is_favorite: _is_favorite,
  };
  let resp = diesel::insert_into(user_favorite)
    .values(&new_favor_item)
    .get_result::<UserFavorite>(conn)?;
  return Ok(resp);
}

pub fn get_by_user_id(
  conn: &mut PgConnection,
  _user_id: &i64,
) -> Result<Option<UserFavorite>, DieselError> {
  let result = user_favorite
    .filter(user_id.eq(_user_id))
    .first::<UserFavorite>(conn)
    .optional()?;
  return Ok(result);
}

pub fn update_item_favorite_status(
  conn: &mut PgConnection,
  _user_id: &i64,
  _item_id: &i64,
  _is_favorite: bool,
) -> Result<(), DieselError> {
  let result = diesel::update(user_favorite)
    .filter(user_id.eq(_user_id).and(item_id.eq(_item_id)))
    .set(is_favorite.eq(_is_favorite))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}

pub fn get_favorite_item_by_user_id_and_item_id(
  conn: &mut PgConnection,
  _user_id: i64,
  _item_id: i64,
) -> Result<UserFavorite, DieselError> {
  let result = user_favorite
    .filter(user_id.eq(_user_id).and(item_id.eq(_item_id)))
    .first::<UserFavorite>(conn)?;
  return Ok(result);
}
