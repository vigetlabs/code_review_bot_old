package codereview

import (
	"context"

	"github.com/google/go-github/v42/github"
	"github.com/vigetlabs/code_review_bot/db"
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
	l              *zap.SugaredLogger
	db             db.DB
	slackClient    slack.Client
	slackChannelID string
}

func (s *service) HandlePullRequestEvent(ctx context.Context, event github.PullRequestEvent) error {
	s.l.Infow("HandlePullRequestEvent", "action", event.Action)

	repoID := *event.PullRequest.Base.Repo.ID
	pullRequestID := *event.PullRequest.ID

	pr, err := s.db.PullRequest(ctx, repoID, pullRequestID)
	if err != nil {
		return err
	}

	info := pullRequestInfo(event.PullRequest)
	if pr == nil {
		channelID, timestamp, err := s.slackClient.SendPullRequestMessage(ctx, s.slackChannelID, info)
		if err != nil {
			return err
		}

		return s.db.PutPullRequest(ctx, &db.PullRequest{
			RepoID:                repoID,
			PullRequestID:         pullRequestID,
			SlackChannelID:        channelID,
			SlackMessageTimestamp: timestamp,
		})
	}

	return s.slackClient.UpdatePullRequestMessage(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp, info)
}

func (s *service) HandlePullRequestReviewEvent(ctx context.Context, event github.PullRequestReviewEvent) error {
	s.l.Infow("HandlePullRequestReviewEvent", "action", event.Action)

	return nil
}

// NewService constructs a code review service
func NewService(logger *zap.Logger, db db.DB, slackClient slack.Client, slackChannelID string) Service {
	return &service{
		l:              logger.Sugar(),
		db:             db,
		slackClient:    slackClient,
		slackChannelID: slackChannelID,
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
