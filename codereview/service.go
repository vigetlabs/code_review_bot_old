package codereview

import (
	"context"
	"sort"
	"strings"

	"github.com/google/go-github/v42/github"
	"github.com/vigetlabs/code_review_bot/db"
	"github.com/vigetlabs/code_review_bot/languages"
	"github.com/vigetlabs/code_review_bot/slack"
	"go.uber.org/zap"
)

// Service describes code review service methods
type Service interface {
	// HandlePullRequestEvent handles pull request events
	HandlePullRequestEvent(ctx context.Context, event github.PullRequestEvent) error
	// HandlePullRequestReviewEvent handles pull request review events
	HandlePullRequestReviewEvent(ctx context.Context, event github.PullRequestReviewEvent) error
}

type service struct {
	l              *zap.SugaredLogger
	db             db.DB
	slackClient    slack.Client
	githubClient   *github.Client
	slackChannelID string
	languageIndex  map[string]*languages.Language
}

func (s *service) HandlePullRequestEvent(ctx context.Context, event github.PullRequestEvent) error {
	s.l.Infow("HandlePullRequestEvent", "action", event.Action)

	repoID := *event.PullRequest.Base.Repo.ID
	pullRequestID := *event.PullRequest.ID

	pr, err := s.db.PullRequest(ctx, repoID, pullRequestID)
	if err != nil {
		return err
	}

	owner := *event.PullRequest.Base.Repo.Owner.Login
	repo := *event.PullRequest.Base.Repo.Name
	number := *event.PullRequest.Number

	filesErrorStatus := ""
	files, resp, err := s.githubClient.PullRequests.ListFiles(ctx, owner, repo, number, &github.ListOptions{PerPage: 100})
	if err != nil {
		if resp != nil {
			filesErrorStatus = resp.Status
		} else {
			filesErrorStatus = "unknown"
		}
	}

	exts := fileExtensions(files)
	fileTypesMap := make(map[string]bool)
	var fileTypes []string
	for _, ext := range exts {
		if l, ok := s.languageIndex[ext]; ok {
			if _, ok := fileTypesMap[l.Name]; !ok {
				fileTypesMap[l.Name] = true

				str := l.Name
				if emoji, ok := languages.Emojis[l.Name]; ok {
					str = emoji
				} else if l.Group != "" {
					if emoji, ok := languages.Emojis[l.Group]; ok {
						str = emoji
					}
				}
				fileTypes = append(fileTypes, str)
			}
		}
	}

	info := pullRequestInfo(event.PullRequest, fileTypes, filesErrorStatus)
	if pr == nil {
		channelID, timestamp, err := s.slackClient.SendPullRequestMessage(ctx, s.slackChannelID, info)
		if err != nil {
			return err
		}

		return s.db.PutPullRequest(ctx, &db.PullRequest{
			RepoID:                repoID,
			PullRequestID:         pullRequestID,
			SlackChannelID:        channelID,
			SlackMessageTimestamp: timestamp,
		})
	}

	return s.slackClient.UpdatePullRequestMessage(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp, info)
}

func (s *service) HandlePullRequestReviewEvent(ctx context.Context, event github.PullRequestReviewEvent) error {
	s.l.Infow("HandlePullRequestReviewEvent", "action", event.Action)

	repoID := *event.PullRequest.Base.Repo.ID
	pullRequestID := *event.PullRequest.ID

	pr, err := s.db.PullRequest(ctx, repoID, pullRequestID)
	if err != nil {
		return err
	}

	if pr != nil {
		return s.slackClient.AddReaction(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp)
	}

	return nil
}

// NewService constructs a code review service
func NewService(logger *zap.Logger, db db.DB, slackClient slack.Client, githubClient *github.Client, slackChannelID string, languages []*languages.Language) Service {
	return &service{
		l:              logger.Sugar(),
		db:             db,
		slackClient:    slackClient,
		githubClient:   githubClient,
		slackChannelID: slackChannelID,
		languageIndex:  indexLanguages(languages),
	}
}

func indexLanguages(ls []*languages.Language) map[string]*languages.Language {
	index := make(map[string]*languages.Language)
	for _, l := range ls {
		for _, ext := range l.Extensions {
			trimmed := ext[1:]
			if c, ok := index[trimmed]; ok {
				if len(c.Extensions) <= len(l.Extensions) {
					continue
				}
			}

			index[trimmed] = l
		}
	}

	return index
}

func fileExtensions(files []*github.CommitFile) []string {
	m := make(map[string]int)
	for _, f := range files {
		containsDot := strings.Contains(*f.Filename, ".")
		if !containsDot {
			continue
		}

		p := strings.Split(*f.Filename, ".")
		ext := p[len(p)-1]
		m[ext]++
	}

	exts := make([]string, len(m))
	i := 0
	for ext := range m {
		exts[i] = ext
		i++
	}

	// Order extension in descending order by file count
	sort.Slice(exts, func(i, j int) bool {
		return m[exts[i]] > m[exts[j]]
	})

	return exts
}

func pullRequestInfo(pullRequest *github.PullRequest, fileTypes []string, filesErrorStatus string) slack.PullRequestInfo {
	var name string
	if pullRequest.User.Name != nil {
		name = *pullRequest.User.Name
	} else {
		name = *pullRequest.User.Login
	}

	return slack.PullRequestInfo{
		UserName:         name,
		UserAvatarURL:    *pullRequest.User.AvatarURL,
		UserLogin:        *pullRequest.User.Login,
		Title:            *pullRequest.Title,
		URL:              *pullRequest.HTMLURL,
		Repo:             *pullRequest.Base.Repo.FullName,
		Commits:          *pullRequest.Commits,
		ChangedFiles:     *pullRequest.ChangedFiles,
		Additions:        *pullRequest.Additions,
		Deletions:        *pullRequest.Deletions,
		State:            *pullRequest.State,
		Merged:           *pullRequest.Merged,
		FileTypes:        fileTypes,
		FilesErrorStatus: filesErrorStatus,
	}
}
