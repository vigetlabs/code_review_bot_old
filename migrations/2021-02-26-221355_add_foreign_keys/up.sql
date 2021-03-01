-- Your SQL goes here
/* ALTER TABLE pull_requests */
/*   ADD CONSTRAINT fk_pullrequestsgithubusers */
/*   FOREIGN KEY (github_user_id) */
/*   REFERENCES github_users(id); */

ALTER TABLE reviews
  ADD CONSTRAINT fk_pullrequestreviews
  FOREIGN KEY (pull_request_id)
  REFERENCES pull_requests(id);

ALTER TABLE reviews
  ADD CONSTRAINT fk_githubusersreviews
  FOREIGN KEY (github_user_id)
  REFERENCES github_users(id);

