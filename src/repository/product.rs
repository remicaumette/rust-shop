use sqlx::PgPool;
use anyhow::Result;
use crate::model::product::Product;

#[derive(Clone)]
pub struct ProductRepository {
  db: PgPool
}

impl ProductRepository {
  pub fn new(db: PgPool) -> Self {
    Self { db }
  }

  pub async fn list(&self) -> Result<Vec<Product>, sqlx::Error>{
    sqlx::query_as!(Product, "SELECT * FROM products").fetch_all(&self.db).await
  }
}
