use crate::types::{PFError, Plan, Proxy, User};
use elasticsearch::{http::transport::Transport, Elasticsearch, SearchParts};
use serde_json::json;
use sqlx::{mysql::MySqlArguments, Arguments, MySqlPool};

#[derive(Debug)]
struct DBConnector {
  elastic: Option<Elasticsearch>,
  mysql: Option<MySqlPool>,
}

impl DBConnector {
  pub async fn get_user(&self, email: &str) -> Result<User, PFError> {
    if let Some(elastic) = &self.elastic {
      let response = elastic
        .search(SearchParts::Index(&["users"]))
        .from(0)
        .size(1)
        .body(json!({
          "query": {
            "match": {
              "email": email
            }
          }
        }))
        .send()
        .await?;
      return Ok(response.json().await?);
    } else if let Some(mysql) = &self.mysql {
      let row = sqlx::query(
        r#"
SELECT email, country, last_proxy, plan
FROM users
        "#,
      )
      .fetch_one(mysql)
      .await?;
      return Ok(row.into());
    }
    Err(PFError::NotFound)
  }

  pub async fn update_proxy(&self, email: &str, proxy: &Proxy) -> Result<(), PFError> {
    if let Some(mysql) = &self.mysql {
      let proxy = serde_json::to_string(&proxy).unwrap();
      let mut args = MySqlArguments::default();
      args.add(proxy);
      args.add(email);
      sqlx::query_with(
        r#"
UPDATE users SET last_proxy = ? WHERE email = ?
        "#,
        args,
      )
      .execute(mysql)
      .await?;
    }

    Ok(())
  }
}

impl From<Elasticsearch> for DBConnector {
  fn from(value: Elasticsearch) -> Self {
    Self {
      elastic: Some(value),
      mysql: None,
    }
  }
}

impl From<MySqlPool> for DBConnector {
  fn from(value: MySqlPool) -> Self {
    Self {
      elastic: None,
      mysql: Some(value),
    }
  }
}

/// Selector class holds connections to DBs and list of proxies.
#[derive(Debug, Default)]
pub struct ProxySelector {
  connectors: Vec<DBConnector>,
  proxy_list: Vec<Proxy>,
}

impl ProxySelector {
  /// Creates new selector object.
  pub fn new() -> Self {
    Self {
      connectors: vec![],
      proxy_list: vec![],
    }
  }

  /// Add Elasticsearch to data sources.
  pub async fn add_elastic(&mut self, url: &str) -> Result<(), elasticsearch::Error> {
    let transport = Transport::single_node(url)?;
    let client = Elasticsearch::new(transport);
    self.connectors.push(client.into());
    Ok(())
  }

  /// Add MySQL to data sources.
  pub async fn add_mysql(&mut self, url: &str) -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect(url).await?;
    self.connectors.push(pool.into());
    Ok(())
  }

  /// Add custom proxy locally.
  pub fn add_proxy<S: Into<String>>(&mut self, addr: S, plan: Plan) {
    self.proxy_list.push(Proxy {
      addr: addr.into(),
      country: "UA".into(),
      latency: 100,
      plan,
    })
  }

  /// Get user entity by email.
  pub async fn get_user(&self, email: &str) -> Result<User, PFError> {
    for connector in self.connectors.iter() {
      match connector.get_user(email).await {
        Ok(user) => return Ok(user),
        Err(PFError::NotFound) => {}
        Err(err) => return Err(err),
      }
    }
    Err(PFError::NoDB)
  }

  /// Get appropriate proxy for specific user.
  pub async fn get_proxy(&self, user: &User) -> Result<&Proxy, PFError> {
    let proxy = self
      .proxy_list
      .iter()
      .filter(|proxy| proxy.plan == user.plan)
      .filter(|proxy| proxy.country.eq(&user.country))
      .min_by(|x, y| x.latency.cmp(&y.latency));
    if let Some(proxy) = proxy {
      return Ok(proxy);
    } else {
      let any_proxy = self
        .proxy_list
        .iter()
        .min_by(|x, y| x.latency.cmp(&y.latency));
      if let Some(proxy) = any_proxy {
        return Ok(proxy);
      }
    }
    Err(PFError::NotFound)
  }

  /// Update last used proxy for a user.
  pub async fn update_user_proxy(&self, user: &User, proxy: &Proxy) -> Result<(), PFError> {
    for connector in self.connectors.iter() {
      connector.update_proxy(&user.email, proxy).await?;
    }
    Ok(())
  }
}
