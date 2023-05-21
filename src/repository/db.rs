use diesel::{
  prelude::*,
  r2d2::{ConnectionManager},
};

use crate::helpers::get_env;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn connection_manager() -> ConnectionManager<PgConnection> {
  let db_url = get_db_url();
  let client = ConnectionManager::<PgConnection>::new(&db_url);
  client
}

fn get_db_url() -> String {
  let postgres_user = get_env("POSTGRES_USER");
  let postgres_pwd = get_env("POSTGRES_PASSWORD");
  let postgres_db_port = get_env("POSTGRES_DB_PORT");
  let postgres_db = get_env("POSTGRES_DB");
  format!(
    "postgresql://{}:{}@localhost:{}/{}",
    postgres_user, postgres_pwd, postgres_db_port, postgres_db
  )
}
