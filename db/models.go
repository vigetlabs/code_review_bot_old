package db

// PullRequest models a stored pull request
type PullRequest struct {
	RepoID                int64
	PullRequestID         int64
	SlackChannelID        string
	SlackMessageTimestamp string
}
