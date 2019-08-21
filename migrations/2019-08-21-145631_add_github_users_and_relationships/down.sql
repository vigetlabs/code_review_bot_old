-- This file should undo anything in `up.sql`
ALTER TABLE pull_requests DROP COLUMN github_user_id;
DROP INDEX inx_reviews;
DROP TABLE reviews;
DROP INDEX inx_github_id;
DROP TABLE github_users;
