package codereview

import (
	"context"
	"encoding/json"
	"net/http"

	"github.com/aws/aws-lambda-go/events"
	"github.com/google/go-github/v42/github"
	"go.uber.org/zap"
)

// Lambda describes lambda handlers
type Lambda interface {
	// Payload handles a GitHub webhook payload
	Payload(ctx context.Context, req events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error)
}

type lambda struct {
	l *zap.SugaredLogger
	s Service
}

func (l *lambda) Payload(ctx context.Context, req events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	eventName := req.Headers["x-github-event"]
	switch eventName {
	case "ping":
		return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusOK}, nil
	case "pull_request":
		return l.pullRequest(ctx, req)
	case "pull_request_review":
		return l.pullRequestReview(ctx, req)
	default:
		l.l.Errorf("bad event name: %s", eventName)

		return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusBadRequest}, nil
	}
}

func (l *lambda) pullRequest(ctx context.Context, req events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	var event github.PullRequestEvent
	if err := json.Unmarshal([]byte(req.Body), &event); err != nil {
		return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusBadRequest}, err
	}

	if err := l.s.HandlePullRequestEvent(ctx, event); err != nil {
		l.l.Errorw("pullRequest", "err", err)

		return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusInternalServerError}, err
	}

	return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusOK}, nil
}

func (l *lambda) pullRequestReview(ctx context.Context, req events.APIGatewayV2HTTPRequest) (events.APIGatewayV2HTTPResponse, error) {
	var event github.PullRequestReviewEvent
	if err := json.Unmarshal([]byte(req.Body), &event); err != nil {
		return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusBadRequest}, err
	}

	if err := l.s.HandlePullRequestReviewEvent(ctx, event); err != nil {
		l.l.Errorw("pullRequestReview", "err", err)

		return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusInternalServerError}, err
	}

	return events.APIGatewayV2HTTPResponse{StatusCode: http.StatusOK}, nil
}

// NewLambda constructs a code review lambda
func NewLambda(logger *zap.Logger, service Service) Lambda {
	return &lambda{
		l: logger.Sugar(),
		s: service,
	}
}
