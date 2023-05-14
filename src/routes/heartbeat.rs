use actix_web::{
  get, http::header, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};

#[get("/heartbeat")]
pub async fn heartbeat() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("OK"))
}
