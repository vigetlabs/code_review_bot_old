-- This file should undo anything in `up.sql`
ALTER TABLE github_users
  DROP CONSTRAINT fk_githubuserusers;
