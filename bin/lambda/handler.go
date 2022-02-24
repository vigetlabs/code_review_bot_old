package main

import (
	"context"
	"io/ioutil"
	"log"
	"strings"

	runtime "github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/google/go-github/v42/github"
	"github.com/spf13/viper"
	"github.com/vigetlabs/code_review_bot/pkg/codereview"
	"github.com/vigetlabs/code_review_bot/pkg/db"
	"github.com/vigetlabs/code_review_bot/pkg/lang"
	"github.com/vigetlabs/code_review_bot/pkg/slack"
	"go.uber.org/zap"
	"golang.org/x/oauth2"
)

func main() {
	// Configure viper environment variable consumption
	viper.SetEnvPrefix("code_review_bot")
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	viper.AutomaticEnv()

	// Set up logger
	var logger *zap.Logger
	if viper.GetBool("dev") {
		logger, _ = zap.NewDevelopment()
	} else {
		logger, _ = zap.NewProduction()
	}
	l := logger.Sugar()

	// Construct language index
	langIndex := lang.NewIndex()

	data, err := ioutil.ReadFile("languages.yml")
	if err != nil {
		l.Fatalw("Failed to read languages.yml", "err", err)
	}

	if err := langIndex.Load(data); err != nil {
		l.Fatalw("Failed to load languages", "err", err)
	}

	// AWS config
	cfg, err := config.LoadDefaultConfig(context.Background())
	if err != nil {
		log.Fatalf("Unable to load SDK config, %v", err)
	}

	// Set up DynamoDB client
	var opts []func(*dynamodb.Options)
	if viper.GetString("dynamodb.url") != "" {
		opts = append(opts, dynamodb.WithEndpointResolver(dynamodb.EndpointResolverFromURL(viper.GetString("dynamodb.url"))))
	}

	dbc := dynamodb.NewFromConfig(cfg, opts...)
	db := db.New(dbc)

	// Set up Slack client
	slackClient := slack.New(logger, viper.GetString("slack.token"))

	// Set up GitHub client
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: viper.GetString("github.accessToken")},
	)
	tc := oauth2.NewClient(context.Background(), ts)
	githubClient := github.NewClient(tc)

	// Set up lambda
	s := codereview.NewService(logger, db, slackClient, githubClient, langIndex, viper.GetString("slack.channelID"))
	lambda := codereview.NewLambda(logger, s)

	// Register lambda
	runtime.Start(lambda.Payload)
}
