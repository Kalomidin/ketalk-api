use actix_web::{get, post, delete, web, Error, HttpMessage, HttpRequest, HttpResponse};
use s3::bucket;

use super::models::{
  CreatePresignedUrlResponse, GetUserResponse, ItemStatus, NewUserRequest, NewUserResponse,
  SignInRequest, UpdateProfileRequest, UserItem, UserItems,
};
use super::DbPool;
use super::{route_error_handler, RouteError};
use crate::auth::{create_jwt, get_new_refresh_token};
use crate::repository::auth::insert_new_refresh_token;
use crate::repository::item::get_favorite_items;
use crate::repository::item_image::get_docs_for_item;

use crate::helpers::get_timestamp_as_nano;
use crate::repository::user::{
  get_user_by_id, get_user_by_phone_number, insert_new_user, update_profile as repo_update_profile, delete_cover_image as repo_delete_cover_image,
};

use crate::repository::item::get_items_by_user_id;
use crate::routes::item::CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME;
use s3::bucket::Bucket;

const IMAGE_UPLOAD_EXPIRATION_SECONDS: u32 = 4000;

#[post("/users/signup")]
pub async fn signup(
  pool: web::Data<DbPool>,
  form: web::Json<NewUserRequest>,
) -> Result<HttpResponse, Error> {
  // save the user into the db
  let user_name = form.name.to_owned();
  let phone_number = form.phone_number.to_owned();
  let password = form.password.to_owned();

  let pool_cloned = pool.clone();
  let user = web::block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      let user = insert_new_user(&mut conn, &user_name, &phone_number, &password)?;
      return Ok(user);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

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
    .map_err(|e| route_error_handler(e))?;

    Ok(HttpResponse::Ok().json(NewUserResponse {
      id: user.id,
      name: user.name,
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
  form: web::Json<SignInRequest>,
) -> Result<HttpResponse, Error> {
  log::info!("received sigin: {:?}", form);

  // save the user into the db
  let phone_number = form.phone_number.to_owned();
  let password = form.password.to_owned();
  let pool_cloned = pool.clone();
  let resp = web::block(move || {
    if let Ok(mut conn) = pool_cloned.get() {
      let user = get_user_by_phone_number(&mut conn, &phone_number)?;
      if user.password != password {
        return Err(RouteError::InvalidPassword);
      }
      if let Ok(auth_token) = create_jwt(user.id) {
        // create new refresh token
        let new_token = get_new_refresh_token();
        let refresh_token = insert_new_refresh_token(&mut conn, user.id, &new_token)?;

        return Ok(NewUserResponse {
          id: user.id,
          name: user.name,
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
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().json(resp))
}

#[get("/users")]
pub async fn get_user(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let user = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let user = get_user_by_id(&mut conn, user_id.to_owned())?;
      return Ok(user);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  let avatar = if user.cover_image != None {
    format!(
      "https://{}/{}",
      CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, user.cover_image.unwrap()
    )
  } else {
    "".to_owned()
  };
  Ok(HttpResponse::Ok().json(GetUserResponse {
    id: user.id,
    name: user.name,
    phone_number: user.phone_number,
    avatar: avatar,
  }))
}

#[get("/users/items")]
pub async fn get_user_items(
  pool: web::Data<DbPool>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let items = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let items = get_items_by_user_id(&mut conn, user_id.to_owned())?;
      let mut resp = vec![];
      for item in items {
        let docs = get_docs_for_item(&mut conn, item.id)?;
        for doc in docs {
          if doc.is_cover && doc.uploaded_to_cloud {
            let item_status = match item.item_status.as_str() {
              "Active" => ItemStatus::Active,
              "Sold" => ItemStatus::Sold,
              _ => ItemStatus::Reserved,
            };
            resp.push(UserItem {
              id: item.id,
              item_name: item.title,
              image: format!(
                "https://{}/{}",
                CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, doc.key,
              ),
              price: item.price,
              favorite_count: item.favorite_count,
              message_count: item.message_count,
              item_status: item_status,
              is_hidden: item.is_hidden,
              created_at: item.created_at.timestamp(),
              updated_at: item.updated_at.timestamp(),
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
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().json(UserItems { items: items }))
}

#[get("/users/items/favorite")]
pub async fn get_user_favorite_items(
  pool: web::Data<DbPool>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let items = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let items = match get_favorite_items(&mut conn, user_id.to_owned()) {
        Ok(items) => items,
        Err(e) => {
          return Ok(vec![]);
        }
      };
      let mut resp = vec![];
      for item in items {
        let docs = get_docs_for_item(&mut conn, item.id)?;
        for doc in docs {
          if doc.is_cover && doc.uploaded_to_cloud {
            let item_status = match item.item_status.as_str() {
              "Active" => ItemStatus::Active,
              "Sold" => ItemStatus::Sold,
              _ => ItemStatus::Reserved,
            };
            resp.push(UserItem {
              id: item.id,
              item_name: item.title,
              image: format!(
                "https://{}/{}",
                CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, doc.key,
              ),
              price: item.price,
              favorite_count: item.favorite_count,
              message_count: item.message_count,
              item_status: item_status,
              is_hidden: item.is_hidden,
              created_at: item.created_at.timestamp(),
              updated_at: item.updated_at.timestamp(),
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
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().json(UserItems { items: items }))
}

#[get("/users/coverImage/presignedUrl")]
pub async fn get_presigned_url_for_cover_image(
  req: HttpRequest,
  bucket: web::Data<Bucket>,
) -> Result<HttpResponse, Error> {
  // create presigned url for the user and respond back
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let object_name = format!("images/{0}/{1}", user_id, get_timestamp_as_nano(),);
  match bucket.presign_put(
    format!("{0}", &object_name),
    IMAGE_UPLOAD_EXPIRATION_SECONDS,
    None,
  ) {
    Ok(url) => {
      return Ok(HttpResponse::Ok().json(CreatePresignedUrlResponse { 
        url: url,
        image_name: object_name,
      }));
    }
    Err(_e) => {
      // just ask from user to reupload again
      return Err(route_error_handler(RouteError::InternalErr));
    }
  };
}

#[post("/users/update")]
pub async fn update_profile(
  pool: web::Data<DbPool>,
  req: HttpRequest,
  form: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse, Error> {
  // create presigned url for the user and respond back
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let new_cover_image = form.image.to_owned();
  let new_name = form.name.to_owned();
  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      repo_update_profile(&mut conn, user_id, new_cover_image, new_name)?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}

#[delete("/users/coverImage")]
pub async fn delete_cover_image(
  pool: web::Data<DbPool>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  // create presigned url for the user and respond back
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      repo_delete_cover_image(&mut conn, user_id)?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}
