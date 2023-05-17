use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row};

/// Custom error type 
#[derive(Debug)]
pub enum PFError {
  /// DB connection missing.
  NoDB,
  /// Object in the DB missing.
  NotFound,
  /// Elasticsearch related error.
  Elastic(elasticsearch::Error),
  /// MySQL related error.
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

/// Describes available subscription plans.  
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Plan {
  /// No subscription.
  Free,
  /// Cheap subscription.
  Basic,
  /// Full access subscription.
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

/// Describes the user entity.
#[derive(Debug, Deserialize)]
pub struct User {
  /// User's email.
  pub email: String,
  /// User's country.
  pub country: String,
  /// Last used proxy.
  pub last_proxy: Option<Proxy>,
  /// Subscription plan.
  pub plan: Plan
}

impl From<MySqlRow> for User {
  fn from(value: MySqlRow) -> Self {
    Self {
      email: value.get(0),
      country: value.get(1),
      last_proxy: serde_json::from_str(value.get(1)).ok(),
      plan: (value.get::<&str, usize>(3)).into()
    }
  }
}

/// Describes proxy entity.
#[derive(Debug, Deserialize, Serialize)]
pub struct Proxy {
  /// Proxy address.
  pub addr: String,
  /// Proxy location.
  pub country: String,
  /// Latency from the user.
  pub latency: u16,
  /// Minimum subscription plan. 
  pub plan: Plan
}