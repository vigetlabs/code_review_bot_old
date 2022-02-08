package codereview

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/google/go-github/v42/github"
	"go.uber.org/zap"
)

// API describes code review API handlers
type API interface {
	// Payload handles a GitHub webhook payload
	Payload(c *gin.Context)
}

type api struct {
	l *zap.SugaredLogger
	s Service
}

func (a *api) Payload(c *gin.Context) {
	eventName := c.GetHeader("X-GitHub-Event")
	switch eventName {
	case "ping":
		c.Status(http.StatusOK)
	case "pull_request":
		a.pullRequest(c)
	case "pull_request_review":
		a.pullRequestReview(c)
	default:
		c.AbortWithStatus(http.StatusBadRequest)
	}
}

func (a *api) pullRequest(c *gin.Context) {
	var event github.PullRequestEvent
	if err := c.BindJSON(&event); err != nil {
		c.AbortWithStatus(http.StatusBadRequest)
		return
	}

	if err := a.s.HandlePullRequestEvent(c, event); err != nil {
		c.AbortWithStatus(http.StatusInternalServerError)
		return
	}

	c.Status(http.StatusOK)
}

func (a *api) pullRequestReview(c *gin.Context) {
	var event github.PullRequestReviewEvent
	if err := c.BindJSON(&event); err != nil {
		c.AbortWithStatus(http.StatusBadRequest)
		return
	}

	if err := a.s.HandlePullRequestReviewEvent(c, event); err != nil {
		c.AbortWithStatus(http.StatusInternalServerError)
		return
	}

	c.Status(http.StatusOK)
}

// NewAPI constructs a code review API
func NewAPI(logger *zap.Logger, service Service) API {
	return &api{
		l: logger.Sugar(),
		s: service,
	}
}
