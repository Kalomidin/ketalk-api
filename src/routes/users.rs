use actix_web::{get, post, web, Error, HttpMessage, HttpRequest, HttpResponse};

use super::models::{
  GetUserResponse, ItemStatus, NewUserRequest, NewUserResponse, SiginRequest, UserItem, UserItems,
};
use super::DbPool;
use super::RouteError;
use crate::auth::{create_jwt, get_new_refresh_token};
use crate::repository::auth::insert_new_refresh_token;
use crate::repository::document::{get_docs_for_item, insert_new_document};
use crate::repository::user::{
  get_user_by__username_and_phone_number, get_user_by_id, insert_new_user,
};

use crate::repository::item::get_items_by_user_id;
use crate::routes::item::CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME;

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
    if let Ok(mut conn) = pool_cloned.get() {
      let user = insert_new_user(&mut conn, &user_name, &phone_number)?;
      return Ok(user);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;

  if let Ok(auth_token) = create_jwt(user.id) {
    // create new refresh token
    let pool_cloned = pool.clone();
    let refresh_token = web::block(move || {
      let new_token = get_new_refresh_token();
      if let Ok(mut conn) = pool_cloned.get() {
        let user = insert_new_refresh_token(&mut conn, user.id, &new_token)?;
        return Ok(user);
      }
      return Err(RouteError::PoolingErr);
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(NewUserResponse {
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

#[post("/users/signin")]
pub async fn signin(
  pool: web::Data<DbPool>,
  form: web::Json<SiginRequest>,
) -> Result<HttpResponse, Error> {
  println!("received sigin: {:?}", form);

  // save the user into the db
  let user_name = form.user_name.to_owned();
  let phone_number = form.phone_number.to_owned();
  let pool_cloned = pool.clone();
  let resp = web::block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      let user = get_user_by__username_and_phone_number(&mut conn, &user_name, &phone_number)?;
      if let Ok(auth_token) = create_jwt(user.id) {
        // create new refresh token
        let new_token = get_new_refresh_token();
        let refresh_token = insert_new_refresh_token(&mut conn, user.id, &new_token)?;

        return Ok(NewUserResponse {
          user_id: user.id,
          user_name: user.user_name,
          phone_number: user.phone_number,
          auth_token: auth_token,
          refresh_token: refresh_token.token,
        });
      } else {
        return Err(RouteError::CreateJwtErr);
      }
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;

  Ok(HttpResponse::Ok().json(resp))
}

#[get("/users")]
pub async fn get_user(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  println!("getting the user");
  let user = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let user = get_user_by_id(&mut conn, user_id.to_owned())?;
      return Ok(user);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;
  Ok(HttpResponse::Ok().json(GetUserResponse {
    id: user.id,
    user_name: user.user_name,
    phone_number: user.phone_number,
  }))
}

#[get("/user/items")]
pub async fn get_user_items(
  pool: web::Data<DbPool>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  println!("getting the user items");
  let items = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let items = get_items_by_user_id(&mut conn, user_id.to_owned())?;
      let mut resp = vec![];
      for item in items {
        let docs = get_docs_for_item(&mut conn, item.id)?;
        for doc in docs {
          if doc.is_cover && doc.uploaded_to_cloud {
            resp.push(UserItem {
              id: item.id,
              item_name: item.description,
              image: format!(
                "https://{}/{}",
                CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, doc.key,
              ),
              price: item.price,
              favorite_count: item.favorite_count,
              message_count: item.message_count,
              item_status: ItemStatus::Active,
              created_at: item.created_at,
              updated_at: item.updated_at,
            });
            break;
          }
        }
      }
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;
  Ok(HttpResponse::Ok().json(UserItems { items: items }))
}
