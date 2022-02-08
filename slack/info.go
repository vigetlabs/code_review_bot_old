package slack

import (
	"fmt"

	"github.com/slack-go/slack"
)

type PullRequestInfo struct {
	UserName       string
	UserAvatarURL  string
	UserLogin      string
	Title          string
	URL            string
	Repo           string
	Commits        int
	ChangedFiles   int
	Additions      int
	Deletions      int
	MergeableState string
	State          string
}

func (i PullRequestInfo) Blocks() []slack.Block {
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
			"status",
			slack.NewTextBlockObject(
				slack.MarkdownType,
				fmt.Sprintf("(+%d -%d)", i.Additions, i.Deletions),
				false,
				false,
			),
		),
	}
}
