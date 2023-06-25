use diesel::result::Error as DieselError;
use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager},
};
use std::fmt;

pub mod auth;
pub mod category;
pub mod geofence;
pub mod heartbeat;
pub mod item;
pub mod item_image;
pub mod karat;
pub mod models;
pub mod room;
pub mod users;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub enum RouteError {
  DbError(DieselError),
  CreateJwtErr,
  Unauthorized,
  BadRequest(String),
  InternalErr,
  PoolingErr,
  NoCoverImage,
  InvalidPassword,
  InvalidCategory,
}

unsafe impl Send for RouteError {}

impl From<DieselError> for RouteError {
  fn from(err: DieselError) -> RouteError {
    RouteError::DbError(err)
  }
}

impl fmt::Display for RouteError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RouteError::DbError(ref e) => e.fmt(f),
      RouteError::CreateJwtErr => write!(f, "Error creating jwt"),
      RouteError::Unauthorized => write!(f, "Unauthorized"),
      RouteError::InternalErr => write!(f, "Internal Server Error"),
      RouteError::PoolingErr => write!(f, "Error pooling connection"),
      RouteError::NoCoverImage => write!(f, "No cover image"),
      RouteError::InvalidPassword => write!(f, "Invalid password"),
      RouteError::InvalidCategory => write!(f, "Invalid category"),
      RouteError::BadRequest(mes) => write!(f, "Bad Request: {:?}", mes.as_str()),
    }
  }
}

fn route_error_handler(e: RouteError) -> actix_web::error::Error {
  println!("request failed with error: {:}", e);
  match e {
    RouteError::Unauthorized => actix_web::error::ErrorUnauthorized("Unauthorized"),
    RouteError::NoCoverImage => actix_web::error::ErrorBadRequest("No cover image found"),
    RouteError::InternalErr => actix_web::error::ErrorInternalServerError("Internal Server Error"),
    RouteError::CreateJwtErr => actix_web::error::ErrorInternalServerError("Error creating jwt"),
    RouteError::PoolingErr => {
      actix_web::error::ErrorInternalServerError("Error pooling connection")
    }
    RouteError::InvalidPassword => actix_web::error::ErrorBadRequest("Invalid password"),
    RouteError::InvalidCategory => actix_web::error::ErrorBadRequest("Invalid category"),
    RouteError::BadRequest(mes) => {
      actix_web::error::ErrorBadRequest(format!("Bad request: {:?}", mes))
    }
    RouteError::DbError(e) => match e {
      DieselError::NotFound => actix_web::error::ErrorNotFound("Not found"),
      e => actix_web::error::ErrorInternalServerError(format!("Internal Server Error: {}", e)),
    },
  }
}
