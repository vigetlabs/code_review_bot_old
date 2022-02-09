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
	FileExtensions   map[string]int
	FilesErrorStatus string
}

// Blocks constructs the Slack message content blocks for a pull request
func (i PullRequestInfo) Blocks() []slack.Block {
	var files string
	if i.FilesErrorStatus == "" {
		if len(i.FileExtensions) > 0 {
			var exts []string
			for ext := range i.FileExtensions {
				md := fmt.Sprintf("`%s`", ext)
				exts = append(exts, md)
			}

			files = strings.Join(exts, ", ")
		} else {
			files = "unknown file types"
		}
	} else {
		files = fmt.Sprintf("GH request failed with %s status", i.FilesErrorStatus)
	}
	var exts []string
	for ext := range i.FileExtensions {
		exts = append(exts, ext)
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
				slack.MarkdownType,
				fmt.Sprintf("(+%d, -%d)", i.Additions, i.Deletions),
				false,
				false,
			),
			slack.NewTextBlockObject(
				slack.MarkdownType,
				files,
				false,
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
