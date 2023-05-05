use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer};
use dotenv::dotenv;
use diesel::r2d2;

use rust_chat_app::helpers::{get_env};
use rust_chat_app::routes::users::{signup, get_user};
use rust_chat_app::routes::heartbeat::{heartbeat};
use rust_chat_app::routes::ws::{join_room, create_room};
use rust_chat_app::routes::auth::{refresh_auth_token};
use rust_chat_app::repository::db::{connection_manager};
use actix_web_httpauth::middleware::HttpAuthentication;
use rust_chat_app::auth::validator;
use rust_chat_app::ws::lobby::Lobby;
use actix::Actor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok(); 
  let server_addr = get_env("SERVER_ADDR");
  let server_port: u16 = get_env("SERVER_PORT").parse().unwrap();

  // connect to postgres db
  let connection_manager = connection_manager();
  // TODO: Run the migrations
  
  let pool = r2d2::Pool::builder().build(connection_manager).expect("Failed to create pool.");
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
            http::header::CONTENT_TYPE
            ])
          .max_age(3600); 
      App::new()
      .app_data(web::Data::new(pool.clone()))
      .app_data(web::Data::new(chat_server.clone()))
      .wrap(cors)
      .service(heartbeat)
      .service(signup)
      .service(refresh_auth_token)
      .service(web::scope("").wrap(bearer_middleware.clone()).service(join_room))
      .service(web::scope("").wrap(bearer_middleware.clone()).service(create_room))
      .service(web::scope("").wrap(bearer_middleware).service(get_user))
  })
  .workers(2)
  .bind((server_addr.as_str(), server_port))?
  .run(); 
  
  println!("Server running at http://{}:{}", server_addr, server_port);

  app.await
}
