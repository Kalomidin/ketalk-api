use actix_web::{
  get, Error, HttpResponse,
};

#[get("/heartbeat")]
pub async fn heartbeat() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("OK"))
}
