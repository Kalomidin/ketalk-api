use actix_web::{post, web, Error, HttpMessage, HttpRequest, HttpResponse};
use s3::bucket::Bucket;

use super::models::{
  CreateItemImagesRequest, CreateItemImagesResponse, ItemImage,
  ItemImagesUpdateStatusToUploadedRequest,
};
use super::DbPool;
use super::{route_error_handler, RouteError};
use crate::helpers::get_timestamp_as_nano;
use crate::repository::item::get_item_by_id;
use crate::repository::item_image::{insert_new_image, set_to_uploaded_to_cloud};
use crate::repository::user::get_user_by_id;

const IMAGE_UPLOAD_EXPIRATION_SECONDS: u32 = 4000;

#[post("/images/item/create")]
pub async fn create_upload_presigned_url(
  pool: web::Data<DbPool>,
  bucket: web::Data<Bucket>,
  form: web::Json<CreateItemImagesRequest>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();
  let item_id = form.item_id.to_owned();

  let resp = web::block(move || {
    if let Ok(mut conn) = pool.get() {
      // get the user
      let user = get_user_by_id(&mut conn, user_id)?;

      // get the item
      // TODO: get the item by id and user_id
      let item = get_item_by_id(&mut conn, item_id)?;
      if item.owner_id != user.id {
        return Err(RouteError::Unauthorized);
      }

      // TODO: validate item does not exist already in the item_image table

      // insert images
      let mut resp: Vec<ItemImage> = vec![];
      for image in form.images.to_owned().into_iter() {
        let object_name = format!(
          "images/{0}/{1}-{2}",
          user.id,
          get_timestamp_as_nano(),
          image.name
        );

        // create the presigned url
        match bucket.presign_put(
          format!("{0}", &object_name),
          IMAGE_UPLOAD_EXPIRATION_SECONDS,
          None,
        ) {
          Ok(url) => {
            let is_cover = image.is_cover.unwrap_or(false);
            let item_image = insert_new_image(
              &mut conn,
              user_id,
              item.id,
              object_name.clone(),
              false,
              is_cover,
            )?;
            resp.push(ItemImage {
              key: object_name,
              url: url,
              is_cover: is_cover,
              name: image.name,
              id: item_image.id,
            });
          }
          Err(_e) => {
            // just ask from user to reupload again
            return Err(RouteError::InternalErr);
          }
        }
      }

      return Ok(resp);
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;

  Ok(HttpResponse::Ok().json(CreateItemImagesResponse { images: resp }))
}

#[post("/images/item/uploaded")]
pub async fn update_status(
  pool: web::Data<DbPool>,
  req: HttpRequest,
  form: web::Json<ItemImagesUpdateStatusToUploadedRequest>,
) -> Result<HttpResponse, Error> {
  let ext = req.extensions();
  let user_id: i64 = ext.get::<i64>().unwrap().to_owned();

  web::block(move || {
    if let Ok(mut conn) = pool.get() {
      // verify user exists the user
      get_user_by_id(&mut conn, user_id)?;

      for image_id in form.ids.to_owned().into_iter() {
        set_to_uploaded_to_cloud(&mut conn, image_id)?;
      }
      return Ok(());
    }
    return Err(RouteError::PoolingErr);
  })
  .await?
  .map_err(|e| route_error_handler(e))?;
  Ok(HttpResponse::Ok().body("OK"))
}
