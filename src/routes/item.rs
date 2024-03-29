use actix_web::web::Json;
use actix_web::{get, post, web, web::Path, Error, HttpMessage, HttpRequest, HttpResponse};

use s3::bucket::Bucket;

use super::models::{
  Buyer, Buyers, CreateItemRequest, CreateItemResponse, CreatePurchaseRequest, GetItemResponse,
  GetItemsResponse, HideUnhideItemRequest, ItemOwner, ItemResponse, ItemStatus,
  UpdateItemStatusRequest,
};
use super::DbPool;
use super::{route_error_handler, RouteError};

use crate::repository::item_image::get_docs_for_item;
use crate::repository::user_favorite::{
  add_item_favorite, get_favorite_item_by_user_id_and_item_id, update_item_favorite_status,
};

use crate::repository::item::{
  create_purchase as repo_create_purchase, get_all_visible, get_item_by_id, hide_unhide_item,
  insert_new_item, update_favorite_count, update_item_status, get_purchase_for_item,
};
use crate::repository::room_member::get_all_buyers_for_item;
use crate::repository::user::{self, get_user_by_id};
use crate::schema::item::owner_id;

use log::warn;

const GET_IMAGE_EXPIRATION_SECONDS: u32 = 200;
pub const CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME: &str = "d1a8cs8n1a8sq9.cloudfront.net";

#[post("/items/create")]
pub async fn create_item(
  pool: web::Data<DbPool>,
  form: web::Json<CreateItemRequest>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  log::info!("received create item: {:?}", form);

  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

  let description = form.description.to_owned();
  let title = form.title.to_owned();
  let price = form.price.to_owned();
  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      // verify user exists
      user::get_user_by_id(&mut conn, user_id)?;
      let new_item = insert_new_item(
        &mut conn,
        user_id,
        title,
        description,
        price,
        form.negotiable,
        form.size,
        form.weight,
        form.karat_id,
        form.category_id,
        form.geofence_id,
      )?;
      return Ok(new_item);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().json(CreateItemResponse {
    id: resp.id,
    created_at: resp.created_at.timestamp(),
  }))
}

#[get("/items")]
pub async fn get_items(
  pool: web::Data<DbPool>,
  req: HttpRequest,
  _bucket: web::Data<Bucket>,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

  let items = web::block(move || -> Result<GetItemsResponse, RouteError> {
    if let Ok(mut conn) = pool.get() {
      // verify user exists the user
      let mut resp = GetItemsResponse { items: vec![] };
      let items = get_all_visible(&mut conn)?;
      for item in items {
        if item.owner_id == user_id {
          continue;
        }
        let docs = get_docs_for_item(&mut conn, item.id)?;
        for doc in docs {
          if doc.is_cover && doc.uploaded_to_cloud {
            let item_status = match item.item_status.as_str() {
              "Active" => ItemStatus::Active,
              "Sold" => ItemStatus::Sold,
              _ => ItemStatus::Reserved,
            };
            resp.items.push(GetItemResponse {
              id: item.id,
              price: item.price,
              title: item.title,
              description: item.description,
              favorite_count: item.favorite_count,
              message_count: item.message_count,
              seen_count: item.seen_count,
              item_status,
              owner_id: item.owner_id,
              created_at: item.created_at.timestamp(),
              // TODO: create presigned url for cloudfront
              thumbnail: format!(
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
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().json(items))
}

#[get("/items/{item_id}")]
pub async fn get_item(
  pool: web::Data<DbPool>,
  item_id: Path<i64>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let item_response = web::block(move || -> Result<ItemResponse, RouteError> {
    if let Ok(mut conn) = pool.get() {
      // verify user exists
      let item = get_item_by_id(&mut conn, item_id.into_inner())?;
      if item.is_hidden && user_id != item.owner_id {
        return Err(RouteError::BadRequest(format!(
          "item is hidden, item id: {}",
          item.id
        )));
      }
      let item_owner = get_user_by_id(&mut conn, item.owner_id)?;
      let user_favorite = get_favorite_item_by_user_id_and_item_id(&mut conn, user_id, item.id);
      let mut is_user_favorite = false;
      if user_favorite.is_ok() {
        is_user_favorite = user_favorite.unwrap().is_favorite;
      }
      let item_status = match item.item_status.as_str() {
        "Active" => ItemStatus::Active,
        "Sold" => ItemStatus::Sold,
        _ => ItemStatus::Reserved,
      };
      let buyer_id = match get_purchase_for_item(&mut conn, item.id) {
        Ok(purchase) => Some(purchase.buyer_id),
        Err(_) => None,
      };

      let mut resp = ItemResponse {
        id: item.id,
        price: item.price,
        title: item.title,
        owner: ItemOwner {
          id: item.owner_id,
          name: item_owner.name,
          location: None,
          avatar: "".to_string(),
        },
        is_user_favorite: is_user_favorite,
        description: item.description,
        favorite_count: item.favorite_count,
        message_count: item.message_count,
        seen_count: item.seen_count,
        item_status: item_status,
        is_hidden: item.is_hidden,
        negotiable: item.negotiable,
        location: None,
        created_at: item.created_at.timestamp(),
        images: vec![],
        buyer_id,
      };
      let docs = get_docs_for_item(&mut conn, item.id)?;
      if docs.len() == 0 {
        warn!("No cover image for item: {}", item.id);
        return Err(RouteError::NoCoverImage);
      }
      for doc in docs {
        if doc.uploaded_to_cloud {
          resp.images.push(format!(
            "https://{}/{}",
            CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME, doc.key
          ));
        }
      }
      // TODO: get the user image
      resp.owner.avatar = resp.images[0].to_owned();
      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().json(item_response))
}

#[post("/items/{item_id}/status")]
pub async fn new_item_status(
  pool: web::Data<DbPool>,
  item_id: Path<i64>,
  req: HttpRequest,
  form: Json<UpdateItemStatusRequest>,
) -> Result<HttpResponse, Error> {
  let _item_id = item_id.into_inner();
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let new_item_status = match form.new_item_status {
    ItemStatus::Active => "Active",
    ItemStatus::Reserved => "Reserved",
    _ => "Sold",
  };
  web::block(move || -> Result<(), RouteError> {
    if let Ok(mut conn) = pool.get() {
      // verify user exists the user
      update_item_status(&mut conn, _item_id, user_id, new_item_status.to_string())?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().body("OK"))
}

#[post("/items/{item_id}/hide")]
pub async fn hide_or_unhide_item(
  pool: web::Data<DbPool>,
  item_id: Path<i64>,
  form: Json<HideUnhideItemRequest>,
) -> Result<HttpResponse, Error> {
  let _item_id = item_id.into_inner();
  web::block(move || -> Result<(), RouteError> {
    if let Ok(mut conn) = pool.get() {
      // verify user exists the user
      hide_unhide_item(&mut conn, _item_id, form.is_hidden)?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().body("OK"))
}

#[post("/items/{item_id}/favorite")]
pub async fn update_favorite_status(
  pool: web::Data<DbPool>,
  req: HttpRequest,
  item_id: Path<i64>,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let user_favorite =
        get_favorite_item_by_user_id_and_item_id(&mut conn, user_id, item_id.to_owned());
      let mut is_favorite = true;
      match user_favorite {
        Ok(user_favorite) => {
          is_favorite = !user_favorite.is_favorite;
          update_item_favorite_status(&mut conn, &user_id, &item_id, is_favorite)?;
        }
        Err(_) => {
          add_item_favorite(&mut conn, &user_id, &item_id, true)?;
        }
      }
      let count = if is_favorite { 1 } else { -1 };
      update_favorite_count(&mut conn, *item_id, count)?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}

// item buyers are all users who started a chat with the item owner
#[get("/items/{item_id}/buyers")]
pub async fn get_item_buyers(
  pool: web::Data<DbPool>,
  req: HttpRequest,
  item_id: Path<i64>,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      let buyers = get_all_buyers_for_item(&mut conn, item_id.to_owned(), user_id)?;
      let mut resp = vec![];
      for buyer in buyers {
        let avatar = if buyer.user_image != None {
          Some(format!(
            "https://{}/{}",
            CLOUD_FRONT_DISTRIBUTION_DOMAIN_NAME,
            buyer.user_image.unwrap()
          ))
        } else {
          None
        };
        resp.push(Buyer {
          id: buyer.id,
          name: buyer.user_name,
          avatar,
          last_messaged_at: buyer.last_joined_at.timestamp(),
        });
      }
      return Ok(Buyers { buyers: resp });
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().json(resp))
}

#[post("/items/{item_id}/purchase")]
pub async fn create_purchase(
  pool: web::Data<DbPool>,
  req: HttpRequest,
  item_id: Path<i64>,
  form: Json<CreatePurchaseRequest>,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let buyer_id = form.buyer_id;
  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      repo_create_purchase(&mut conn, buyer_id, user_id, item_id.to_owned())?;
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}
