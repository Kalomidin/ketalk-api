use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

pub mod users;
pub mod heartbeat;
pub mod auth;
pub mod models;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
