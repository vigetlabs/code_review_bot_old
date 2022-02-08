package slack

import (
	"context"

	"github.com/slack-go/slack"
	"go.uber.org/zap"
)

type Client interface {
	SendPullRequestMessage(ctx context.Context, info PullRequestInfo) error
}

type client struct {
	l         *zap.SugaredLogger
	c         *slack.Client
	channelID string
}

func (c *client) SendPullRequestMessage(ctx context.Context, info PullRequestInfo) error {
	c.l.Infow("SendPullRequestMessage", "info", info)

	_, _, _, err := c.c.SendMessageContext(
		ctx,
		c.channelID,
		slack.MsgOptionUsername(info.UserName),
		slack.MsgOptionIconURL(info.UserAvatarURL),
		slack.MsgOptionBlocks(info.Blocks()...),
	)
	if err != nil {
		return err
	}

	return nil
}

func New(logger *zap.Logger, token, channelID string) Client {
	return &client{
		l:         logger.Sugar(),
		c:         slack.New(token),
		channelID: channelID,
	}
}
