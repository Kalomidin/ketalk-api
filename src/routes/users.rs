use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, HttpMessage};
use uuid::Uuid;

use super::models::{NewUserRequest, NewUserResponse};
use crate::auth::{create_jwt, get_new_refresh_token};
use crate::repository::auth::insert_new_refresh_token;
use crate::repository::user::{insert_new_user, get_user_by_id};
use super::DbPool;

#[post("/users/signup")]
pub async fn signup(
  pool: web::Data<DbPool>,
  form: web::Json<NewUserRequest>,
) -> Result<HttpResponse, Error> {
  println!("received create request: {:?}", form);

  // save the user into the db
  let user_name = form.user_name.to_owned();
  let phone_number = form.phone_number.to_owned();
  let pool_cloned = pool.clone();
  let user = web::block(move || {
    let mut conn = pool_cloned.get()?;
    insert_new_user(&mut conn, &user_name, &phone_number)
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;

  if let Ok(auth_token) = create_jwt(user.id) {
    // create new refresh token
    let pool_cloned = pool.clone();
    let refresh_token = web::block(move || {
      let new_token = get_new_refresh_token();
      let mut conn = pool_cloned.get()?;
      insert_new_refresh_token(&mut conn, user.id, &new_token)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(NewUserResponse{
      user_id: user.id,
      user_name: user.user_name,
      phone_number: user.phone_number,
      auth_token: auth_token,
      refresh_token: refresh_token.token,
    }))
  } else {
    Ok(HttpResponse::InternalServerError().finish())
  }
}

#[get("/users")]
pub async fn get_user(
  pool: web::Data<DbPool>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

  let user = web::block(move || {
    let mut conn = pool.get()?;
    get_user_by_id(&mut conn, user_id.to_owned())
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;
  Ok(HttpResponse::Ok().json(user))
}