use std::io::Result;
use actix_web::{web, middleware, App, HttpServer};
use crate::http::handlers;
use crate::repository::product::ProductRepository;

#[derive(Clone)]
pub struct Server {
  data: ServerData,
}

#[derive(Clone)]
pub struct ServerData {
  pub product_repository: ProductRepository
}

impl Server {
  pub fn new(product_repository: ProductRepository) -> Self {
    Self {
      data: ServerData {
        product_repository
      }
    }
  }

  pub async fn listen(&self, addr: String) -> Result<()> {
    let data = web::Data::new(self.data.clone());

    HttpServer::new(move || App::new()
      .wrap(middleware::Logger::default())
      .app_data(data.clone())
      .route("/v1/products", web::get().to(handlers::products::index))
      // .route("/v1/products", web::post().to(handlers::products::create))
      // .route("/v1/products/{id}", web::get().to(handlers::products::show))
      // .route("/v1/products/{id}", web::get().to(handlers::products::destroy))
    )
      .bind(addr)?
      .run()
      .await
  }
}
