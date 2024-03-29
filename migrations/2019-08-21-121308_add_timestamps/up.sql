-- Your SQL goes here
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

ALTER TABLE pull_requests ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE pull_requests ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

UPDATE pull_requests SET created_at = NOW() - INTERVAL '1 month', updated_at = NOW() - INTERVAL '1 month';

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON pull_requests
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

ALTER TABLE users ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE users ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
