use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row};

#[derive(Debug)]
pub enum PFError {
  NoDB,
  NotFound,
  Elastic(elasticsearch::Error),
  MySql(sqlx::Error)
}

impl From<elasticsearch::Error> for PFError {
  fn from(value: elasticsearch::Error) -> Self {
    Self::Elastic(value)
  }
}

impl From<sqlx::Error> for PFError {
  fn from(value: sqlx::Error) -> Self {
    Self::MySql(value)
  }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Plan {
  Free,
  Basic,
  Premium
} 

impl From<&str> for Plan {
  fn from(value: &str) -> Self {
    match value {
      "Basic" => Self::Basic,
      "Premium" => Self::Premium,
      _ => Self::Free
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct User {
  pub email: String,
  pub country: String,
  pub last_proxy: Option<Proxy>,
  pub plan: Plan
}

impl From<MySqlRow> for User {
  fn from(value: MySqlRow) -> Self {
    Self {
      email: value.get(0),
      country: value.get(1),
      last_proxy: serde_json::from_str(value.get(1)).map_or(None, |p| Some(p)),
      plan: (value.get::<&str, usize>(3)).into()
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Proxy {
  pub addr: String,
  pub country: String,
  pub latency: u16,
  pub plan: Plan
}