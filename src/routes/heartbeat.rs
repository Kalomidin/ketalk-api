use actix_web::{get, post, web, Error, http::header, HttpRequest, HttpResponse, Responder, HttpMessage};

#[get("/heartbeat")]
pub async fn heartbeat() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("OK"))
}