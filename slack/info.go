package slack

import (
	"fmt"
	"strings"

	"github.com/slack-go/slack"
)

// PullRequestInfo holds the info required to generate a Slack message describing a pull request
type PullRequestInfo struct {
	UserName         string
	UserAvatarURL    string
	UserLogin        string
	Title            string
	URL              string
	Repo             string
	Commits          int
	ChangedFiles     int
	Additions        int
	Deletions        int
	State            string
	Merged           bool
	FileTypes        []string
	FilesErrorStatus string
}

// Blocks constructs the Slack message content blocks for a pull request
func (i PullRequestInfo) Blocks() []slack.Block {
	var files string
	if i.FilesErrorStatus == "" {
		if len(i.FileTypes) > 0 {
			files = strings.Join(i.FileTypes, ", ")
		} else {
			files = "unknown file types"
		}
	} else {
		files = fmt.Sprintf("GH request failed with %s status", i.FilesErrorStatus)
	}

	var state string
	if i.Merged {
		state = "merged-pull-request"
	} else if i.State == "closed" {
		state = "closed-pull-request"
	} else {
		state = "open-pull-request"
	}

	return []slack.Block{
		slack.NewSectionBlock(
			slack.NewTextBlockObject(
				slack.MarkdownType,
				fmt.Sprintf("*%s*\n<%s|%s> by %s", i.Title, i.URL, i.Repo, i.UserLogin),
				false,
				false,
			),
			nil,
			nil,
		),
		slack.NewContextBlock(
			"changes",
			slack.NewTextBlockObject(
				slack.MarkdownType,
				fmt.Sprintf("%d commits, %d %s changed", i.Commits, i.ChangedFiles, fileStr(i.ChangedFiles)),
				false,
				false,
			),
		),
		slack.NewContextBlock(
			"files",
			slack.NewTextBlockObject(
				slack.PlainTextType,
				fmt.Sprintf(":%s:", state),
				true,
				false,
			),
			slack.NewTextBlockObject(
				slack.MarkdownType,
				fmt.Sprintf("(+%d, -%d)", i.Additions, i.Deletions),
				false,
				false,
			),
			slack.NewTextBlockObject(
				slack.PlainTextType,
				files,
				true,
				false,
			),
		),
	}
}

func fileStr(changedFiles int) string {
	if changedFiles == 1 {
		return "file"
	}
	return "files"
}
