package db

import (
	"context"
	"encoding/json"
	"fmt"
	"strconv"
	"strings"

	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
	"github.com/google/go-github/v42/github"
)

var (
	attributeRepoID                = "repo_id"
	attributePRID                  = "pr_id"
	attributePR                    = "pr"
	attributeSlackChannelID        = "slack_channel_id"
	attributeSlackMessageTimestamp = "slack_message_timestamp"
	attributeFiles                 = "files"
	attributeApprovals             = "approvals"
	tablePRs                       = "code_review_bot_pull_requests"
	consistentRead                 = true
)

// DB describes db methods
type DB interface {
	// PullRequest gets a pull request. Returns `nil, nil` if no pull request exists.
	PullRequest(ctx context.Context, repoID, prID int64, getPR, getFiles, getApprovals bool) (*PullRequest, error)
	// PutPullRequest creates or replaces a pull request
	PutPullRequest(ctx context.Context, pr *PullRequest) error
	// UpdatePullRequest updates a pull request
	UpdatePullRequest(ctx context.Context, pr *PullRequest, updatePR, updateFiles, updateApprovals bool) error
}

type db struct {
	c *dynamodb.Client
}

func (d *db) PullRequest(ctx context.Context, repoID, prID int64, getPR, getFiles, getApprovals bool) (*PullRequest, error) {
	attributes := []string{attributeSlackChannelID, attributeSlackMessageTimestamp}
	if getPR {
		attributes = append(attributes, attributePR)
	}
	if getFiles {
		attributes = append(attributes, attributeFiles)
	}
	if getApprovals {
		attributes = append(attributes, attributeApprovals)
	}

	output, err := d.c.GetItem(ctx, &dynamodb.GetItemInput{
		Key: map[string]types.AttributeValue{
			attributeRepoID: &types.AttributeValueMemberN{Value: strconv.FormatInt(repoID, 10)},
			attributePRID:   &types.AttributeValueMemberN{Value: strconv.FormatInt(prID, 10)},
		},
		TableName:       &tablePRs,
		ConsistentRead:  &consistentRead,
		AttributesToGet: attributes,
	})
	if err != nil {
		return nil, err
	} else if output.Item == nil {
		return nil, nil
	}

	slackChannelID := output.Item[attributeSlackChannelID].(*types.AttributeValueMemberS).Value
	slackMessageTimestamp := output.Item[attributeSlackMessageTimestamp].(*types.AttributeValueMemberS).Value

	var eventPR *github.PullRequest
	if item, ok := output.Item[attributePR]; ok {
		s := item.(*types.AttributeValueMemberS)
		_ = json.Unmarshal([]byte(s.Value), &eventPR)
	}

	var files []*github.CommitFile
	if item, ok := output.Item[attributeFiles]; ok {
		s := item.(*types.AttributeValueMemberS)
		_ = json.Unmarshal([]byte(s.Value), &files)
	}

	var approvals []*github.User
	if item, ok := output.Item[attributeApprovals]; ok {
		s := item.(*types.AttributeValueMemberS)
		_ = json.Unmarshal([]byte(s.Value), &approvals)
	}

	pr := &PullRequest{
		RepoID:                repoID,
		PullRequestID:         prID,
		PR:                    eventPR,
		SlackChannelID:        slackChannelID,
		SlackMessageTimestamp: slackMessageTimestamp,
		Files:                 files,
		Approvals:             approvals,
	}

	return pr, nil
}

func (d *db) PutPullRequest(ctx context.Context, pr *PullRequest) error {
	b, _ := json.Marshal(pr.Files)
	files := string(b)
	b, _ = json.Marshal(pr.Approvals)
	approvals := string(b)
	b, _ = json.Marshal(pr.PR)
	prJSON := string(b)

	_, err := d.c.PutItem(ctx, &dynamodb.PutItemInput{
		Item: map[string]types.AttributeValue{
			attributeRepoID:                &types.AttributeValueMemberN{Value: strconv.FormatInt(pr.RepoID, 10)},
			attributePRID:                  &types.AttributeValueMemberN{Value: strconv.FormatInt(pr.PullRequestID, 10)},
			attributePR:                    &types.AttributeValueMemberS{Value: prJSON},
			attributeSlackChannelID:        &types.AttributeValueMemberS{Value: pr.SlackChannelID},
			attributeSlackMessageTimestamp: &types.AttributeValueMemberS{Value: pr.SlackMessageTimestamp},
			attributeFiles:                 &types.AttributeValueMemberS{Value: files},
			attributeApprovals:             &types.AttributeValueMemberS{Value: approvals},
		},
		TableName:    &tablePRs,
		ReturnValues: types.ReturnValueNone,
	})
	return err
}

func (d *db) UpdatePullRequest(ctx context.Context, pr *PullRequest, updatePR, updateFiles, updateApprovals bool) error {
	var updates []string
	values := make(map[string]types.AttributeValue)
	if updatePR {
		b, _ := json.Marshal(pr.PR)
		updates = append(updates, fmt.Sprintf("%s = :pr", attributePR))
		values[":pr"] = &types.AttributeValueMemberS{Value: string(b)}
	}
	if updateFiles {
		b, _ := json.Marshal(pr.Files)
		updates = append(updates, fmt.Sprintf("%s = :fs", attributeFiles))
		values[":fs"] = &types.AttributeValueMemberS{Value: string(b)}
	}
	if updateApprovals {
		b, _ := json.Marshal(pr.Approvals)
		updates = append(updates, fmt.Sprintf("%s = :as", attributeApprovals))
		values[":as"] = &types.AttributeValueMemberS{Value: string(b)}
	}

	updateExpr := fmt.Sprintf("SET %s", strings.Join(updates, ", "))

	_, err := d.c.UpdateItem(ctx, &dynamodb.UpdateItemInput{
		Key: map[string]types.AttributeValue{
			attributeRepoID: &types.AttributeValueMemberN{Value: strconv.FormatInt(pr.RepoID, 10)},
			attributePRID:   &types.AttributeValueMemberN{Value: strconv.FormatInt(pr.PullRequestID, 10)},
		},
		UpdateExpression:          &updateExpr,
		ExpressionAttributeValues: values,
		TableName:                 &tablePRs,
		ReturnValues:              types.ReturnValueNone,
	})
	return err
}

// New constructs a db
func New(c *dynamodb.Client) DB {
	return &db{
		c: c,
	}
}
