use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer};
use dotenv::dotenv;
use diesel::r2d2;

use rust_chat_app::helpers::{get_env};
use rust_chat_app::routes::{create_user, get_user, heartbeat};
use rust_chat_app::repository::db::{connection_manager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok(); 
  let server_addr = get_env("SERVER_ADDR");
  let server_port: u16 = get_env("SERVER_PORT").parse().unwrap();
  
  // connect to postgres db
  let connection_manager = connection_manager();
  let pool = r2d2::Pool::builder().build(connection_manager).expect("Failed to create pool.");

  let app = HttpServer::new(move || {
      let cors = Cors::default()
          .allowed_origin("http://localhost:3000")
          .allowed_origin("http://localhost:8080")
          .allowed_methods(vec!["GET", "POST"])
          .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
          .allowed_header(http::header::CONTENT_TYPE)
          .max_age(3600); 
      App::new()
      .app_data(web::Data::new(pool.clone()))
      .wrap(cors)
      .service(heartbeat)
      .service(create_user)
      .service(get_user)
  })
  .workers(2)
  .bind((server_addr.as_str(), server_port))?
  .run(); 
  
  println!("Server running at http://{}:{}", server_addr, server_port);

  app.await
}
