// Development server
package main

import (
	"context"
	"io/ioutil"
	"log"
	"strings"
	"time"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	ginzap "github.com/gin-contrib/zap"
	"github.com/gin-gonic/gin"
	"github.com/google/go-github/v42/github"
	"github.com/spf13/viper"
	"github.com/vigetlabs/code_review_bot/codereview"
	"github.com/vigetlabs/code_review_bot/db"
	"github.com/vigetlabs/code_review_bot/lang"
	"github.com/vigetlabs/code_review_bot/slack"
	"go.uber.org/zap"
	"golang.org/x/oauth2"
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

	// Set up API
	s := codereview.NewService(logger, db, slackClient, githubClient, langIndex, viper.GetString("slack.channelID"))
	api := codereview.NewAPI(logger, s)

	// Set up gin server
	r := gin.Default()

	r.Use(ginzap.Ginzap(logger, time.RFC3339, true))
	r.Use(ginzap.RecoveryWithZap(logger, true))

	r.POST("/payload", api.Payload)

	// Run server
	l.Fatal(r.Run(viper.GetString("addr")))
}
