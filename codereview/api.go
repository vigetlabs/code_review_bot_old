package codereview

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/google/go-github/v42/github"
	"go.uber.org/zap"
)

type API interface {
	Payload(c *gin.Context)
}

type api struct {
	l *zap.SugaredLogger
	s Service
}

func (a *api) Payload(c *gin.Context) {
	eventName := c.GetHeader("X-GitHub-Event")
	if eventName == "ping" {
		c.Status(http.StatusOK)
		return
	} else if eventName == "" || !(eventName == "pull_request" || eventName == "pull_request_review") {
		c.AbortWithStatus(http.StatusBadRequest)
		return
	}

	if eventName == "pull_request" {
		a.pullRequest(c)
	} else if eventName == "pull_request_review" {
		a.pullRequestReview(c)
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

func NewAPI(logger *zap.Logger, service Service) API {
	return &api{
		l: logger.Sugar(),
		s: service,
	}
}
