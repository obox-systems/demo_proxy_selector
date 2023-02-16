CREATE TABLE IF NOT EXISTS users
(
  id         BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
  email      TEXT   NOT NULL,
  country    TEXT   NOT NULL,
  last_proxy TEXT   NOT NULL,
  plan       TEXT   NOT NULL
);

INSERT INTO users (email, country, last_proxy, plan) VALUES ("user@mail.com", "UA", "{}", "Basic");