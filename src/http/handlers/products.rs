use actix_web::{web, HttpResponse};
use crate::http::server::{ServerData};

pub async fn index(data: web::Data<ServerData>) -> HttpResponse {
  let products = data.product_repository.list().await.expect("error");
  HttpResponse::Ok().json(products)
}

// pub fn show() -> HttpResponse {
//
// }
//
// pub fn create() -> HttpResponse{
//
// }
//
// pub fn destroy() -> HttpResponse{
//
// }
