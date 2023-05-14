use actix::Actor;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use diesel::r2d2;
use dotenv::dotenv;

use actix_web_httpauth::middleware::HttpAuthentication;
use rust_chat_app::auth::validator;
use rust_chat_app::helpers::get_env;
use rust_chat_app::repository::db::connection_manager;
use rust_chat_app::routes::auth::{logout, refresh_auth_token};
use rust_chat_app::routes::document::{create_upload_presigned_url, update_status};
use rust_chat_app::routes::heartbeat::heartbeat;
use rust_chat_app::routes::item::{create_item, get_item, get_items};
use rust_chat_app::routes::room::{create_room, get_user_rooms, join_room};
use rust_chat_app::routes::users::{get_user, signin, signup};
use rust_chat_app::s3_bucket::get_s3_bucket;
use rust_chat_app::ws::lobby::Lobby;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let server_addr = get_env("SERVER_ADDR");
  let server_port: u16 = get_env("SERVER_PORT").parse().unwrap();

  let bucket = get_s3_bucket();

  // connect to postgres db
  let connection_manager = connection_manager();
  // TODO: Run the migrations

  let pool = r2d2::Pool::builder()
    .build(connection_manager)
    .expect("Failed to create pool.");
  let chat_server: actix::Addr<Lobby> = Lobby::new(pool.clone()).start(); //create and spin up a lobby

  let app = HttpServer::new(move || {
    let bearer_middleware = HttpAuthentication::bearer(validator);
    let cors = Cors::default()
      .allowed_origin("http://localhost:3000")
      .allowed_origin("http://localhost:8080")
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(vec![
        http::header::AUTHORIZATION,
        http::header::ACCEPT,
        http::header::CONTENT_TYPE,
      ])
      .max_age(3600);
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .app_data(web::Data::new(chat_server.clone()))
      .app_data(web::Data::new(bucket.clone()))
      .wrap(cors)
      .service(heartbeat)
      .service(signin)
      .service(signup)
      .service(refresh_auth_token)
      // .service(web::scope("").wrap(bearer_middleware.clone()).service())
      .service(
        web::scope("")
          .wrap(bearer_middleware)
          .service(get_user)
          .service(create_room)
          .service(join_room)
          .service(logout)
          .service(get_user_rooms)
          .service(create_upload_presigned_url)
          .service(create_item)
          .service(update_status)
          .service(get_items)
          .service(get_item),
      )
  })
  .workers(2)
  .bind((server_addr.as_str(), server_port))?
  .run();

  println!("Server running at http://{}:{}", server_addr, server_port);

  app.await
}
