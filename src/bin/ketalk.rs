use actix::Actor;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use diesel::r2d2;
use dotenv::dotenv;

use actix_web_httpauth::middleware::HttpAuthentication;
use ketalk::auth::validator;
use ketalk::helpers::get_env;
use ketalk::repository::db::connection_manager;
use ketalk::routes::auth::{logout, refresh_auth_token};
use ketalk::routes::category::{create_category, delete_category, get_categories, get_category};
use ketalk::routes::heartbeat::heartbeat;
use ketalk::routes::item::{create_item, get_item, get_items, hide_or_unhide_item, new_item_status};
use ketalk::routes::item_image::{create_upload_presigned_url, update_status};
use ketalk::routes::room::{create_room, get_user_rooms, join_room};
use ketalk::routes::users::{get_user, get_user_items, signin, signup};
use ketalk::s3_bucket::get_s3_bucket;
use ketalk::ws::lobby::Lobby;

use local_ip_address::local_ip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let server_addr = get_env("SERVER_ADDR");
  let server_port: u16 = get_env("SERVER_PORT").parse().unwrap();

  let bucket = get_s3_bucket();

  let my_local_ip = local_ip();
  if let Ok(my_local_ip) = my_local_ip {
    println!("This is my local IP address: {:?}", my_local_ip);
  } else {
    println!("Error getting local IP: {:?}", my_local_ip);
  }

  // connect to postgres db
  let connection_manager = connection_manager();

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
      .service(create_category)
      .service(get_categories)
      .service(get_category)
      .service(delete_category)
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
          .service(get_item)
          .service(get_user_items)
          .service(new_item_status)
          .service(hide_or_unhide_item),
      )
  })
  .workers(2)
  .bind((server_addr.as_str(), server_port))?
  .run();

  println!("Server running at http://{}:{}", server_addr, server_port);

  app.await
}
