use actix_web::{get, post, web, web::Path, Error, HttpMessage, HttpRequest, HttpResponse};
use s3::bucket::Bucket;

use super::models::{
  CreateItemRequest, CreateItemResponse, GetItemResponse, GetItemsResponse, ItemResponse,
};
use super::DbPool;
use super::RouteError;
use crate::helpers::get_timestamp_as_nano;
use crate::repository::document::{get_docs_for_item, insert_new_document};
use crate::repository::item::{get_all, get_item_by_id, insert_new_item};
use crate::repository::user::get_user_by_id;

const GET_IMAGE_EXPIRATION_SECONDS: u32 = 200;
const CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME: &str = "d1a8cs8n1a8sq9.cloudfront.net";

#[post("/item/create")]
pub async fn create_item(
  pool: web::Data<DbPool>,
  form: web::Json<CreateItemRequest>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  println!("received create item: {:?}", form);

  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

  // save the user into the db
  let description = form.description.to_owned();
  let details = form.details.to_owned();
  let price = form.price.to_owned();
  let negotiable = form.negotiable.to_owned();
  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let new_item = insert_new_item(
        &mut conn,
        user_id,
        description,
        details,
        price,
        negotiable.unwrap_or(false),
      )?;
      return Ok(new_item);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;

  Ok(HttpResponse::Ok().json(CreateItemResponse {
    id: resp.id,
    description: resp.description,
    created_at: resp.created_at.to_string(),
  }))
}

#[get("/items")]
pub async fn get_items(
  pool: web::Data<DbPool>,
  req: HttpRequest,
  bucket: web::Data<Bucket>,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

  let items = web::block(move || -> Result<GetItemsResponse, RouteError> {
    if let Ok(mut conn) = pool.get() {
      // verify user exists the user
      let mut resp = GetItemsResponse { items: vec![] };
      let items = get_all(&mut conn)?;
      for item in items {
        let docs = get_docs_for_item(&mut conn, item.id)?;
        for doc in docs {
          if doc.is_cover && doc.uploaded_to_cloud {
            resp.items.push(GetItemResponse {
              id: item.id,
              price: 0,
              details: "".to_string(),
              description: item.description,
              owner_id: item.owner_id,
              created_at: item.created_at.to_string(),
              // TODO: create presigned url for cloudfront
              cover_image_url: format!(
                "https://{}/{}",
                CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, doc.key,
              ),
            });

            // We will return only after getting the cover
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

  Ok(HttpResponse::Ok().json(items))
}

#[get("/item/{item_id}")]
pub async fn get_item(
  pool: web::Data<DbPool>,
  item_id: Path<i64>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let item_response = web::block(move || -> Result<ItemResponse, RouteError> {
    if let Ok(mut conn) = pool.get() {
      // verify user exists the user
      let item = get_item_by_id(&mut conn, item_id.into_inner())?;
      let item_owner = get_user_by_id(&mut conn, item.owner_id)?;

      let mut resp = ItemResponse {
        id: item.id,
        price: 0,
        details: "".to_string(),
        description: item.description,
        owner_id: item.owner_id,
        owner_name: item_owner.user_name,
        owner_location: None,
        owner_image_url: "".to_string(),
        favorite_count: 0,
        negotiable: item.negotiable,
        message_count: 0,
        created_at: item.created_at.to_string(),
        presigned_urls: vec![],
        location: None,
      };
      let docs = get_docs_for_item(&mut conn, item.id)?;
      for doc in docs {
        if doc.uploaded_to_cloud {
          resp.presigned_urls.push(format!(
            "https://{}/{}",
            CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, doc.key
          ));
        }
      }
      // TODO: get the user image
      resp.owner_image_url = resp.presigned_urls[0].to_owned();
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(actix_web::error::ErrorUnprocessableEntity)?;

  Ok(HttpResponse::Ok().json(item_response))
}
