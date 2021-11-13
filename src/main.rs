use std::env;
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use rust_shop::http::server::Server;
use rust_shop::repository::product::ProductRepository;

fn get_env(key: &str) -> String {
  match env::var(key) {
    Ok(v) => v,
    Err(_) => "".to_string()
  }
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&get_env("DATABASE_URL")).await.unwrap();

  let server = Server::new(ProductRepository::new(pool));
  server.listen(get_env("ADDR")).await?;

  Ok(())
}
