-- Your SQL goes here
UPDATE pull_requests
  SET github_user_id = (
    SELECT id
    FROM github_users
    WHERE github_users.github_id = pull_requests.github_user_id
  );

ALTER TABLE pull_requests
  ADD CONSTRAINT fk_pullrequestsgithubusers
  FOREIGN KEY (github_user_id)
  REFERENCES github_users(id);
