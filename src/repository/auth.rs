use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::helpers::new_naive_date;
use crate::schema::refresh_token as refresh_token_table;
use crate::schema::refresh_token::dsl::*;
use diesel::result::Error as DieselError;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = refresh_token_table)]
pub struct InsertRefreshToken {
  pub user_id: i64,
  pub token: String,
  pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RefreshToken {
  pub id: i64,
  pub user_id: i64,
  pub token: String,
  pub created_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub fn insert_new_refresh_token(
  conn: &mut PgConnection,
  requester_user_id: i64,
  refresh_access_token: &str,
) -> Result<RefreshToken, DieselError> {
  let new_refresh_token = InsertRefreshToken {
    user_id: requester_user_id,
    token: refresh_access_token.to_owned(),
    created_at: new_naive_date(),
  };

  let resp = diesel::insert_into(refresh_token)
    .values(&new_refresh_token)
    .get_result::<RefreshToken>(conn)?;
  return Ok(resp);
}

pub fn get_refresh_token(
  conn: &mut PgConnection,
  requester_user_id: i64,
  refresh_access_token: &str,
) -> Result<RefreshToken, DieselError> {
  let result = refresh_token
    .filter(
      deleted_at
        .is_null()
        .and(user_id.eq(requester_user_id))
        .and(token.eq(refresh_access_token)),
    )
    .first::<RefreshToken>(conn)
    .optional()?;
  match result {
    Some(val) => Ok(val),
    None => Err(DieselError::NotFound),
  }
}

pub fn delete_refresh_token(
  conn: &mut PgConnection,
  requester_user_id: i64,
  refresh_access_token: &str,
) -> Result<(), DieselError> {
  let result = diesel::update(refresh_token)
    .filter(
      deleted_at
        .is_null()
        .and(user_id.eq(requester_user_id))
        .and(token.eq(refresh_access_token)),
    )
    .set(deleted_at.eq(new_naive_date()))
    .execute(conn);
  if result.is_err() || result.unwrap() == 0 {
    return Err(DieselError::NotFound);
  }
  Ok(())
}
