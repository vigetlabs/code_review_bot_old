-- Your SQL goes here
ALTER TABLE github_users
  ADD CONSTRAINT fk_githubuserusers
  FOREIGN KEY (user_id)
  REFERENCES users(id);
