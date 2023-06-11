use actix_web::{delete, get, post, web, Error, HttpMessage, HttpRequest, HttpResponse};
use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager},
};

use super::models::CreateCategoryRequest;
use super::DbPool;
use super::{route_error_handler, RouteError};
use crate::auth::{create_jwt, get_new_refresh_token};
use crate::repository::category::{
  add_category, delete_category as repo_delete_category, get_by_name,
  get_categories as repo_get_categories,
};

#[post("/categories/create")]
pub async fn create_category(
  form: web::Json<CreateCategoryRequest>,
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
  let pool_cloned: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>> = pool.clone();
  let name = form.name.to_owned();
  let avatar = form.avatar.to_owned();
  web::block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      add_category(&mut conn, name, avatar)?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().body("OK"))
}

#[get("/categories/{name}")]
pub async fn get_category(
  pool: web::Data<DbPool>,
  name: web::Path<String>,
) -> Result<HttpResponse, Error> {
  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let resp = get_by_name(&mut conn, name.to_string())?;
      if resp.is_none() {
        return Err(RouteError::InvalidCategory);
      }
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().json(resp))
}

#[get("/categories")]
pub async fn get_categories(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let resp = repo_get_categories(&mut conn)?;
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().json(resp))
}

#[delete("/categories/{name}")]
pub async fn delete_category(
  pool: web::Data<DbPool>,
  name: web::Path<String>,
) -> Result<HttpResponse, Error> {
  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      repo_delete_category(&mut conn, name.to_string())?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}
