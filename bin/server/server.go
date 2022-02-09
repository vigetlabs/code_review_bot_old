package main

import (
	"context"
	"log"
	"strings"
	"time"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	ginzap "github.com/gin-contrib/zap"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	"github.com/vigetlabs/code_review_bot/codereview"
	"github.com/vigetlabs/code_review_bot/db"
	"github.com/vigetlabs/code_review_bot/slack"
	"go.uber.org/zap"
)

func main() {
	viper.SetEnvPrefix("code_review_bot")
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	viper.AutomaticEnv()

	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(".")
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			log.Fatalf("Failed to read in config, %v", err)
		}
	}

	var logger *zap.Logger
	if viper.GetBool("dev") {
		logger, _ = zap.NewDevelopment()
	} else {
		logger, _ = zap.NewProduction()
	}
	l := logger.Sugar()

	l.Infow("Env vars", "slack.channelID", viper.GetString("slack.channelID"))

	cfg, err := config.LoadDefaultConfig(context.Background())
	if err != nil {
		log.Fatalf("Unable to load SDK config, %v", err)
	}

	var opts []func(*dynamodb.Options)
	if viper.GetString("dynamodb.url") != "" {
		opts = append(opts, dynamodb.WithEndpointResolver(dynamodb.EndpointResolverFromURL(viper.GetString("dynamodb.url"))))
	}

	dbc := dynamodb.NewFromConfig(cfg, opts...)
	db := db.New(dbc)

	slackClient := slack.New(logger, viper.GetString("slack.token"))

	api := codereview.NewAPI(logger, codereview.NewService(logger, db, slackClient, viper.GetString("slack.channelID")))

	r := gin.Default()

	r.Use(ginzap.Ginzap(logger, time.RFC3339, true))
	r.Use(ginzap.RecoveryWithZap(logger, true))

	r.POST("/payload", api.Payload)

	l.Fatal(r.Run(viper.GetString("addr")))
}
