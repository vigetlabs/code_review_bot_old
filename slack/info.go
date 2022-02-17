package slack

import (
	"fmt"

	"github.com/slack-go/slack"
)

// A PullRequestApprovalUser holds info about a user who approved a pull request
type PullRequestApprovalUser struct {
	AvatarURL string
	Login     string
}

// A PullRequestInfo holds the info required to generate a Slack message describing a pull request
type PullRequestInfo struct {
	UserName       string
	UserAvatarURL  string
	UserLogin      string
	Title          string
	URL            string
	Repo           string
	BaseRef        string
	HeadRef        string
	Commits        int
	ChangedFiles   int
	Additions      int
	Deletions      int
	State          string
	Merged         bool
	Langs          []string
	FilesErrStatus string
	Approvals      []PullRequestApprovalUser
}

// Blocks constructs the Slack message content blocks for a pull request
func (i PullRequestInfo) Blocks() []slack.Block {
	var state string
	if i.Merged {
		state = "merged-pull-request"
	} else if i.State == "closed" {
		state = "closed-pull-request"
	} else {
		state = "open-pull-request"
	}

	files := []slack.MixedElement{
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
	}
	if i.FilesErrStatus == "" {
		if len(i.Langs) > 0 {
			for _, l := range i.Langs {
				files = append(files, slack.NewTextBlockObject(
					slack.PlainTextType,
					l,
					true,
					false,
				))
			}
		} else {
			files = append(files, slack.NewTextBlockObject(
				slack.MarkdownType,
				"_unknown languages_",
				false,
				false,
			))
		}
	} else {
		files = append(files, slack.NewTextBlockObject(
			slack.MarkdownType,
			fmt.Sprintf("GH request failed with `%s` status", i.FilesErrStatus),
			false,
			false,
		))
	}

	blocks := []slack.Block{
		slack.NewSectionBlock(
			slack.NewTextBlockObject(
				slack.MarkdownType,
				fmt.Sprintf("*%s*  `%s` ← `%s`\n<%s|%s> by %s", i.Title, i.BaseRef, i.HeadRef, i.URL, i.Repo, i.UserLogin),
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
				fmt.Sprintf("%d %s, %d %s changed", i.Commits, commitStr(i.Commits), i.ChangedFiles, fileStr(i.ChangedFiles)),
				false,
				false,
			),
		),
		slack.NewContextBlock(
			"files",
			files...,
		),
	}

	if len(i.Approvals) > 0 {
		approvals := []slack.MixedElement{slack.NewTextBlockObject(
			slack.PlainTextType,
			":approved-pull-request:",
			true,
			false,
		)}

		for _, u := range i.Approvals {
			approvals = append(approvals, slack.NewImageBlockElement(u.AvatarURL, u.Login))
		}

		blocks = append(blocks, slack.NewContextBlock("approvals", approvals...))
	}

	return blocks
}

func commitStr(commits int) string {
	if commits == 1 {
		return "commit"
	}
	return "commits"
}

func fileStr(changedFiles int) string {
	if changedFiles == 1 {
		return "file"
	}
	return "files"
}
