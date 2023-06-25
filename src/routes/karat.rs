use actix_web::{delete, get, web, Error, HttpResponse};

use super::DbPool;
use super::{route_error_handler, RouteError};
use crate::repository::karat::{
  delete_karat as repo_delete_karat, get_by_name, get_karats as repo_get_karats,
};

#[get("/karats/{name}")]
pub async fn get_karat(
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

#[get("/karats")]
pub async fn get_karats(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let resp = repo_get_karats(&mut conn)?;
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().json(resp))
}

#[delete("/karats/{name}")]
pub async fn delete_karat(
  pool: web::Data<DbPool>,
  name: web::Path<String>,
) -> Result<HttpResponse, Error> {
  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      repo_delete_karat(&mut conn, name.to_string())?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}
