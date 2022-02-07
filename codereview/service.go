package codereview

import (
	"github.com/google/go-github/v42/github"
	"go.uber.org/zap"
)

type Service interface {
	HandlePullRequestEvent(event github.PullRequestEvent) error
	HandlePullRequestReviewEvent(event github.PullRequestReviewEvent) error
}

type service struct {
	l *zap.SugaredLogger
}

func (s *service) HandlePullRequestEvent(event github.PullRequestEvent) error {
	s.l.Infow("HandlePullRequestEvent", "action", event.Action)

	return nil
}

func (s *service) HandlePullRequestReviewEvent(event github.PullRequestReviewEvent) error {
	s.l.Infow("HandlePullRequestReviewEvent", "action", event.Action)

	return nil
}

func NewService(logger *zap.Logger) Service {
	return &service{l: logger.Sugar()}
}
