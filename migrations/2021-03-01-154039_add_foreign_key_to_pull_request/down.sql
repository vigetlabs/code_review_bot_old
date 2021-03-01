-- This file should undo anything in `up.sql`
ALTER TABLE pull_requests
  DROP CONSTRAINT fk_pullrequestsgithubusers;

UPDATE pull_requests
  SET github_user_id = (
    SELECT github_id
    FROM github_users
    WHERE github_users.id = pull_requests.github_user_id
  );
