package db

import "github.com/google/go-github/v42/github"

// PullRequest models a stored pull request
type PullRequest struct {
	RepoID                int64
	PullRequestID         int64
	PR                    *github.PullRequest
	SlackChannelID        string
	SlackMessageTimestamp string
	Files                 []*github.CommitFile
	Approvals             []*github.User
}
