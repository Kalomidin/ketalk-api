
use std::time::Instant;

use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager},
};

use crate::repository::models;
use crate::repository::db::{insert_new_user, get_user_by_id};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[get("/heartbeat")]
pub async fn heartbeat() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("OK"))
}

#[post("/users/create")]
pub async fn create_user(
  pool: web::Data<DbPool>,
  form: web::Json<models::NewUserRequest>,
) -> Result<HttpResponse, Error> {

  // save the user into the db
  let user = web::block(move || {
    let mut conn = pool.get()?;
    insert_new_user(&mut conn, &form.user_name, &form.phone_number)
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;

  // TODO: Return auth token
  Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{user_id}")]
pub async fn get_user(
  pool: web::Data<DbPool>,
  user_id: web::Path<i64>,
) -> Result<HttpResponse, Error> {
  let user = web::block(move || {
    let mut conn = pool.get()?;
    get_user_by_id(&mut conn, user_id.to_owned())
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;
  Ok(HttpResponse::Ok().json(user))
}