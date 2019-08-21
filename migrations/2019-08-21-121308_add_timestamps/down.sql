-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp on users;
ALTER TABLE users DROP COLUMN created_at;
ALTER TABLE users DROP COLUMN updated_at;

DROP TRIGGER set_timestamp on pull_requests;
ALTER TABLE pull_requests DROP COLUMN created_at;
ALTER TABLE pull_requests DROP COLUMN updated_at;

DROP FUNCTION trigger_set_timestamp();
