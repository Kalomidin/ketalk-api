use actix_web::{get, web, Error, HttpResponse};

use super::DbPool;
use super::{route_error_handler, RouteError};
use crate::repository::geofence::get_geofences as repo_get_geofences;

#[get("/geofences")]
pub async fn get_geofences(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let resp = repo_get_geofences(&mut conn)?;
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().json(resp))
}
