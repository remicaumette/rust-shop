use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
  pub id: String,
  pub name: String,
  pub description: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>
}
