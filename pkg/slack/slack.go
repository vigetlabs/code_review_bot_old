package slack

import (
	"context"
	"errors"

	"github.com/slack-go/slack"
	"go.uber.org/zap"
)

// Client describes Slack client methods
type Client interface {
	// SendPullRequestMessage constructs and sends a Slack message based on the pull request info. Returns the channel id and timestamp.
	SendPullRequestMessage(ctx context.Context, channelID string, info PullRequestInfo) (string, string, error)
	// UpdatePullRequestMessage updates an existing Slack message with a newly constructed message based on the pull request info. Returns the channel id and timestamp.
	UpdatePullRequestMessage(ctx context.Context, channelID, timestamp string, info PullRequestInfo) error
	// AddReactionIfNotExists adds a reaction
	AddReactionIfNotExists(ctx context.Context, channelID, timestamp, emojiName string) error
}

type client struct {
	l *zap.SugaredLogger
	c *slack.Client
}

func (c *client) SendPullRequestMessage(ctx context.Context, channelID string, info PullRequestInfo) (string, string, error) {
	c.l.Debugw("SendPullRequestMessage", "channelID", channelID, "info", info)

	channel, timestamp, _, err := c.c.SendMessageContext(
		ctx,
		channelID,
		slack.MsgOptionUsername(info.UserName),
		slack.MsgOptionIconURL(info.UserAvatarURL),
		slack.MsgOptionBlocks(info.Blocks()...),
	)
	if err != nil {
		return "", "", err
	}

	return channel, timestamp, nil
}

func (c *client) UpdatePullRequestMessage(ctx context.Context, channelID, timestamp string, info PullRequestInfo) error {
	c.l.Debugw("UpdatePullRequestMessage", "channelID", channelID, "timestamp", timestamp, "info", info)

	_, _, _, err := c.c.UpdateMessageContext(
		ctx,
		channelID,
		timestamp,
		slack.MsgOptionUsername(info.UserName),
		slack.MsgOptionIconURL(info.UserAvatarURL),
		slack.MsgOptionBlocks(info.Blocks()...),
	)
	if err != nil {
		return err
	}

	return nil
}

func (c *client) AddReactionIfNotExists(ctx context.Context, channelID, timestamp, emojiName string) error {
	c.l.Debugw("AddReactionIfNotExists", "channelID", channelID, "timestamp", timestamp, "emojiName", emojiName)

	err := c.c.AddReactionContext(ctx, emojiName, slack.NewRefToMessage(channelID, timestamp))
	var slackErr slack.SlackErrorResponse
	if errors.As(err, &slackErr) {
		if slackErr.Err == "already_reacted" {
			return nil
		}
	}

	return err
}

// New constructs a Slack client
func New(logger *zap.Logger, token string) Client {
	return &client{
		l: logger.Sugar(),
		c: slack.New(token),
	}
}
