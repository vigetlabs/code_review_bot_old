package main

import (
	"time"

	ginzap "github.com/gin-contrib/zap"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	"github.com/vigetlabs/code_review_bot/codereview"
	"github.com/vigetlabs/code_review_bot/slack"
	"go.uber.org/zap"
)

func main() {
	logger, err := zap.NewProduction()
	if err != nil {
		panic(err)
	}
	l := logger.Sugar()

	viper.SetEnvPrefix("code_review_bot")
	viper.AutomaticEnv()

	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(".")
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); ok {
			l.Info("Starting with no config file...")
		} else {
			l.Fatalw("failed to read in config", "err", err)
		}
	}

	slackClient := slack.New(logger, viper.GetString("slack.token"), viper.GetString("slack.channelID"))

	api := codereview.NewAPI(logger, codereview.NewService(logger, slackClient))

	r := gin.Default()

	r.Use(ginzap.Ginzap(logger, time.RFC3339, true))
	r.Use(ginzap.RecoveryWithZap(logger, true))

	r.POST("/payload", api.Payload)

	l.Fatal(r.Run(viper.GetString("addr")))
}
