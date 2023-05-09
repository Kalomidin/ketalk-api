use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, HttpMessage};
use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager},
};
use s3::bucket::Bucket;

use super::models::{CreatePresignedUrlsRequest, CreatePresignedUrlsResponse, PresignedUrl};
use crate::repository::user::{get_user_by_id};
use super::DbPool;
use super::RouteError;
use crate::helpers::{get_timestamp_as_nano};

const IMAGE_UPLOAD_EXPIRATION_SECONDS: u32 = 4000;

#[post("/documents/uploadPresignedUrl")]
pub async fn create_upload_presigned_url(
    pool: web::Data<DbPool>,
    bucket: web::Data<Bucket>,
    form: web::Json<CreatePresignedUrlsRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {

    // get the user
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
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    let mut resp: Vec<PresignedUrl> = vec![];
    for file_name in form.file_names.to_owned().into_iter() {
        let object_name = format!("{0}/{1}-{2}", user.id, get_timestamp_as_nano(), file_name);
        // create the presigned url
        match bucket.presign_put(format!("images/{0}", object_name), IMAGE_UPLOAD_EXPIRATION_SECONDS, None) {
            Ok(url) => {
                resp.push(PresignedUrl {
                    key: object_name,
                    url: url,
                    file_name: file_name,
                });
            }
            Err(e) => {
                return Err(actix_web::error::ErrorInternalServerError(e));
            }
        }
    }

    Ok(HttpResponse::Ok().json(CreatePresignedUrlsResponse {
        presigned_urls: resp,
    }))
}