package main

import (
	"context"
	"encoding/json"
	"log"
	"strings"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
	"github.com/spf13/viper"
)

var (
	attributeRepoID        = "repo_id"
	attributePullRequestID = "pr_id"
	tablePullRequests      = "code_review_bot_pull_requests"
)

func main() {
	// Configure viper environment variable consumption
	viper.SetEnvPrefix("code_review_bot")
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	viper.AutomaticEnv()

	// Load config.yaml if it exists
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(".")
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			log.Fatalf("Failed to read in config, %v", err)
		}
	}

	cfg, err := config.LoadDefaultConfig(context.Background())
	if err != nil {
		log.Fatalf("Unable to load SDK config, %v", err)
	}

	// Set up DynamoDB client
	var opts []func(*dynamodb.Options)
	if viper.GetString("dynamodb.url") != "" {
		opts = append(opts, dynamodb.WithEndpointResolver(dynamodb.EndpointResolverFromURL(viper.GetString("dynamodb.url"))))
	}

	db := dynamodb.NewFromConfig(cfg, opts...)

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
