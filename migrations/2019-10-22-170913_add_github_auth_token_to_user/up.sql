ALTER TABLE users ADD COLUMN github_access_token VARCHAR;
ALTER TABLE users ALTER COLUMN slack_access_token SET NOT NULL;
