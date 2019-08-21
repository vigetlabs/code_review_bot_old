-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  slack_user_id VARCHAR NOT NULL,
  slack_access_token VARCHAR
)
