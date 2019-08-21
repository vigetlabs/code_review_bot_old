-- Your SQL goes here
CREATE TABLE github_users(
  id SERIAL PRIMARY KEY,
  login VARCHAR NOT NULL,
  avatar_url VARCHAR NOT NULL,
  github_id INTEGER NOT NULL,
  user_id INTEGER
);

CREATE UNIQUE INDEX inx_github_id ON github_users(github_id);

CREATE TABLE reviews(
    id SERIAL PRIMARY KEY,
    pull_request_id INTEGER NOT NULL,
    github_user_id INTEGER NOT NULL,
    state VARCHAR NOT NULL
);

CREATE UNIQUE INDEX inx_reviews ON reviews(pull_request_id, github_user_id);

ALTER TABLE pull_requests ADD COLUMN github_user_id INTEGER NOT NULL DEFAULT 0;

INSERT INTO github_users (login, avatar_url, github_id) VALUES ('unknown', '', 0);

WITH query AS (
    SELECT id FROM github_users
)
UPDATE pull_requests
SET github_user_id = query.id
FROM query
WHERE pull_requests.github_user_id = 0;
