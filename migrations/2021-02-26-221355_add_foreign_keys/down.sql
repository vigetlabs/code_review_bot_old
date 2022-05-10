-- This file should undo anything in `up.sql`
/* ALTER TABLE pull_requests */
/*   DROP CONSTRAINT fk_pullrequestsgithubuser; */

ALTER TABLE reviews
  DROP CONSTRAINT fk_pullrequestreviews;

ALTER TABLE reviews
  DROP CONSTRAINT fk_githubusersreviews;

