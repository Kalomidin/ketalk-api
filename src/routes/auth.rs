use actix_web::{post, web, Error, HttpMessage, HttpRequest, HttpResponse};
use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager},
};

use super::models::{LogoutRequest, RefreshAuthTokenRequest, RefreshAuthTokenResponse};
use super::DbPool;
use super::{route_error_handler, RouteError};
use crate::auth::{create_jwt, get_new_refresh_token};
use crate::repository::auth::{delete_refresh_token, insert_new_refresh_token};

#[post("/auth/refreshAccessAuthToken")]
pub async fn refresh_auth_token(
  form: web::Json<RefreshAuthTokenRequest>,
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
  // invalidate old refresh access token
  let pool_cloned: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>> = pool.clone();
  let user_id = form.user_id.to_owned();
  let refresh_token = form.refresh_token.to_owned();
  web::block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      delete_refresh_token(&mut conn, user_id, &refresh_token)?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  // create new refresh access token
  let pool_cloned = pool.clone();
  let new_refresh_token = web::block(move || {
    let new_token = get_new_refresh_token();
    if let Ok(mut conn) = pool_cloned.get() {
      let res = insert_new_refresh_token(&mut conn, user_id, &new_token)?;
      return Ok(res);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  // create new jwt token and return
  let auth_token = create_jwt(new_refresh_token.user_id)?;
  Ok(HttpResponse::Ok().json(RefreshAuthTokenResponse {
    refresh_token: new_refresh_token.token,
    auth_token: auth_token,
  }))
}

#[post("/auth/logout")]
pub async fn logout(
  pool: web::Data<DbPool>,
  form: web::Json<LogoutRequest>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let refresh_token = form.refresh_token.to_owned();
  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      delete_refresh_token(&mut conn, user_id, &refresh_token)?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}
