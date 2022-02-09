package main

import (
	"context"
	"encoding/json"
	"log"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)

var (
	attributeRepoID        = "repo_id"
	attributePullRequestID = "pull_request_id"
	tablePullRequests      = "code_review_bot_pull_requests"
)

func main() {
	cfg, err := config.LoadDefaultConfig(context.Background())
	if err != nil {
		log.Fatalf("Unable to load SDK config, %v", err)
	}

	db := dynamodb.NewFromConfig(cfg, dynamodb.WithEndpointResolver(dynamodb.EndpointResolverFromURL("http://localhost:8000")))

	output, err := db.CreateTable(context.Background(), &dynamodb.CreateTableInput{
		AttributeDefinitions: []types.AttributeDefinition{
			{
				AttributeName: &attributeRepoID,
				AttributeType: types.ScalarAttributeTypeN,
			},
			{
				AttributeName: &attributePullRequestID,
				AttributeType: types.ScalarAttributeTypeN,
			},
		},
		KeySchema: []types.KeySchemaElement{
			{
				AttributeName: &attributeRepoID,
				KeyType:       types.KeyTypeHash,
			},
			{
				AttributeName: &attributePullRequestID,
				KeyType:       types.KeyTypeRange,
			},
		},
		TableName:   &tablePullRequests,
		BillingMode: types.BillingModePayPerRequest,
	})
	if err != nil {
		log.Fatalf("Failed to create table, %v", err)
	}

	log.Printf("Created table %v\n", tablePullRequests)

	b, _ := json.MarshalIndent(output, "", "  ")
	log.Printf("Output:\n%s\n", b)
}
