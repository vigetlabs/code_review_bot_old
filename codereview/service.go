package codereview

import (
	"context"

	"github.com/google/go-github/v42/github"
	"github.com/vigetlabs/code_review_bot/slack"
	"go.uber.org/zap"
)

// Service describes code review service methods
type Service interface {
	// HandlePullRequestEvent handles pull request events
	HandlePullRequestEvent(ctx context.Context, event github.PullRequestEvent) error
	// HandlePullRequestReviewEvent handles pull request review events
	HandlePullRequestReviewEvent(ctx context.Context, event github.PullRequestReviewEvent) error
}

type service struct {
	l           *zap.SugaredLogger
	slackClient slack.Client
}

func (s *service) HandlePullRequestEvent(ctx context.Context, event github.PullRequestEvent) error {
	s.l.Infow("HandlePullRequestEvent", "action", event.Action)

	return s.slackClient.SendPullRequestMessage(ctx, pullRequestInfo(event.PullRequest))
}

func (s *service) HandlePullRequestReviewEvent(ctx context.Context, event github.PullRequestReviewEvent) error {
	s.l.Infow("HandlePullRequestReviewEvent", "action", event.Action)

	return nil
}

// NewService constructs a code review service
func NewService(logger *zap.Logger, slackClient slack.Client) Service {
	return &service{
		l:           logger.Sugar(),
		slackClient: slackClient,
	}
}

func pullRequestInfo(pullRequest *github.PullRequest) slack.PullRequestInfo {
	var name string
	if pullRequest.User.Name != nil {
		name = *pullRequest.User.Name
	} else {
		name = *pullRequest.User.Login
	}

	return slack.PullRequestInfo{
		UserName:       name,
		UserAvatarURL:  *pullRequest.User.AvatarURL,
		UserLogin:      *pullRequest.User.Login,
		Title:          *pullRequest.Title,
		URL:            *pullRequest.HTMLURL,
		Repo:           *pullRequest.Base.Repo.FullName,
		Commits:        *pullRequest.Commits,
		ChangedFiles:   *pullRequest.ChangedFiles,
		Additions:      *pullRequest.Additions,
		Deletions:      *pullRequest.Deletions,
		MergeableState: *pullRequest.MergeableState,
		State:          *pullRequest.State,
	}
}
