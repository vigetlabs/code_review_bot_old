package db

import (
	"context"
	"strconv"

	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)

var (
	attributeRepoID                = "repo_id"
	attributePullRequestID         = "pull_request_id"
	attributeSlackChannelID        = "slack_channel_id"
	attributeSlackMessageTimestamp = "slack_message_timestamp"
	tablePullRequests              = "code_review_bot_pull_requests"
	consistentRead                 = true
)

// DB describes db methods
type DB interface {
	// PullRequest gets a pull request. Returns `nil, nil` if no pull request exists.
	PullRequest(ctx context.Context, repoID, pullRequestID int64) (*PullRequest, error)
	// PutPullRequest creates or updates a pull request
	PutPullRequest(ctx context.Context, pullRequest *PullRequest) error
}

type db struct {
	c *dynamodb.Client
}

func (d *db) PullRequest(ctx context.Context, repoID, pullRequestID int64) (*PullRequest, error) {
	output, err := d.c.GetItem(ctx, &dynamodb.GetItemInput{
		Key: map[string]types.AttributeValue{
			attributeRepoID:        &types.AttributeValueMemberN{Value: strconv.FormatInt(repoID, 10)},
			attributePullRequestID: &types.AttributeValueMemberN{Value: strconv.FormatInt(pullRequestID, 10)},
		},
		TableName:      &tablePullRequests,
		ConsistentRead: &consistentRead,
	})
	if err != nil {
		return nil, err
	} else if output.Item == nil {
		return nil, nil
	}

	slackChannelID := output.Item[attributeSlackChannelID].(*types.AttributeValueMemberS)
	slackMessageTimestamp := output.Item[attributeSlackMessageTimestamp].(*types.AttributeValueMemberS)

	pr := &PullRequest{
		RepoID:                repoID,
		PullRequestID:         pullRequestID,
		SlackChannelID:        slackChannelID.Value,
		SlackMessageTimestamp: slackMessageTimestamp.Value,
	}

	return pr, nil
}

func (d *db) PutPullRequest(ctx context.Context, pullRequest *PullRequest) error {
	_, err := d.c.PutItem(ctx, &dynamodb.PutItemInput{
		Item: map[string]types.AttributeValue{
			attributeRepoID:                &types.AttributeValueMemberN{Value: strconv.FormatInt(pullRequest.RepoID, 10)},
			attributePullRequestID:         &types.AttributeValueMemberN{Value: strconv.FormatInt(pullRequest.PullRequestID, 10)},
			attributeSlackChannelID:        &types.AttributeValueMemberS{Value: pullRequest.SlackChannelID},
			attributeSlackMessageTimestamp: &types.AttributeValueMemberS{Value: pullRequest.SlackMessageTimestamp},
		},
		TableName: &tablePullRequests,
	})
	return err
}

// New constructs a db
func New(c *dynamodb.Client) DB {
	return &db{
		c: c,
	}
}
