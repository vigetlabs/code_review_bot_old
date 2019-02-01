create TABLE pull_requests (
  id SERIAL PRIMARY KEY,
  github_id VARCHAR NOT NULL,
  state VARCHAR NOT NULL,
  slack_message_id VARCHAR NOT NULL
)
