use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, HttpMessage};
use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager},
};

use super::models::{LogoutRequest, RefreshAuthTokenRequest, RefreshAuthTokenResponse};
use crate::repository::auth::{insert_new_refresh_token, delete_refresh_token};
use crate::auth::{create_jwt, get_new_refresh_token};
use super::DbPool;
use super::RouteError;

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
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

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
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

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
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;
  Ok(HttpResponse::Ok().body("OK"))
}
