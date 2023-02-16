# To begin

`docker run -d --name proxy-elasticsearch -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" elasticsearch:8.6.1`

`docker run --name proxy-mysql -p 3306:3306 -p 33060:33060 -e MYSQL_ROOT_PASSWORD=password -d mysql:latest`

`export DATABASE_URL=mysql://root:password@localhost:3306/proxy`

`sqlx db setup`

`sqlx migrate run`

`cargo run`