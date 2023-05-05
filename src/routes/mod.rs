use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use diesel::result::Error as DieselError;
use std::fmt;

pub mod users;
pub mod heartbeat;
pub mod auth;
pub mod models;
pub mod ws;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub enum RouteError {
    DbError(DieselError),
    PoolingErr
}

unsafe impl Send for RouteError {}

impl From<DieselError> for RouteError {
    fn from(err: DieselError) -> RouteError {
        RouteError::DbError(err)
    }
}



impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RouteError::DbError(ref e) => e.fmt(f),
            _ => write!(f, "Unknown Error"),
        }
    }
}