use proxy_selector::{selector::ProxySelector, types::Plan};

#[tokio::main]
async fn main() {
  let mut selector = ProxySelector::new();

  //selector.add_elastic("http://localhost:9200").await.unwrap();
  selector.add_mysql("mysql://root:password@localhost/proxy").await.unwrap();
  selector.add_proxy("localhost:8887", Plan::Free);
  selector.add_proxy("localhost:8888", Plan::Basic);
  selector.add_proxy("localhost:8889", Plan::Premium);
  let user = selector.get_user("user@mail.com").await.unwrap();
  dbg!(&user);
  let proxy = selector.get_proxy(&user).await.unwrap();
  dbg!(proxy);
  selector.update_user_proxy(&user, proxy).await.unwrap();
}
