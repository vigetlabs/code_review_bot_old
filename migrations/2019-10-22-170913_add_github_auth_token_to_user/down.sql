ALTER TABLE users DROP COLUMN github_access_token;
ALTER TABLE users ALTER COLUMN slack_access_token DROP NOT NULL;
