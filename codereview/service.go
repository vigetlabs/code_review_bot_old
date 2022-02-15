package codereview

import (
	"context"
	"sort"

	"github.com/google/go-github/v42/github"
	"github.com/vigetlabs/code_review_bot/db"
	"github.com/vigetlabs/code_review_bot/lang"
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
	langIndex      *lang.Index
	slackChannelID string
}

func (s *service) HandlePullRequestEvent(ctx context.Context, event github.PullRequestEvent) error {
	action := *event.Action

	s.l.Debugw("HandlePullRequestEvent", "action", action)

	if !(action == "closed" || action == "edited" || action == "opened" || action == "reopened" || action == "synchronize") {
		return nil
	}

	repoID := *event.PullRequest.Base.Repo.ID
	pullRequestID := *event.PullRequest.ID

	pr, err := s.db.PullRequest(ctx, repoID, pullRequestID)
	if err != nil {
		return err
	}

	ls, langsErrorStatus := s.langs(ctx, event.PullRequest)

	info := pullRequestInfo(event.PullRequest, ls, langsErrorStatus)

	// If the PR isn't in the db, send a new message, otherwise update the existing message
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
	action := *event.Action

	s.l.Debugw("HandlePullRequestReviewEvent", "action", action, "state", *event.Review.State)

	if !(action == "submitted") {
		return nil
	}

	repoID := *event.PullRequest.Base.Repo.ID
	pullRequestID := *event.PullRequest.ID

	pr, err := s.db.PullRequest(ctx, repoID, pullRequestID)
	if err != nil {
		return err
	} else if pr == nil {
		return nil
	}

	var reaction string
	if *event.Review.State == "approved" {
		reaction = "approved-pull-request"
	} else {
		reaction = "memo"
	}

	return s.slackClient.AddReaction(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp, reaction)
}

func (s *service) langs(ctx context.Context, pr *github.PullRequest) ([]string, string) {
	owner := *pr.Base.Repo.Owner.Login
	repo := *pr.Base.Repo.Name
	number := *pr.Number

	files, resp, err := s.githubClient.PullRequests.ListFiles(ctx, owner, repo, number, &github.ListOptions{PerPage: 100})
	if err != nil {
		if resp != nil {
			return nil, resp.Status
		}
		return nil, "unknown"
	}

	changes := make(map[*lang.Lang]int)
	for _, f := range files {
		l := s.langIndex.LangForFile(*f.Filename)
		if l == nil {
			continue
		}

		changes[l] += *f.Changes
	}

	langMap := make(map[string]int)
	for l, c := range changes {
		e := l.Emoji()
		if e != "" {
			langMap[e] += c
			continue
		}

		if l.Group != "" {
			gl := s.langIndex.LangByName(l.Group)
			e := gl.Emoji()
			if e != "" {
				langMap[e] += c
				continue
			}
		}

		langMap[l.Name] += c
	}

	langs := make([]string, len(langMap))
	i := 0
	for l := range langMap {
		langs[i] = l
		i++
	}

	// Sort langs by change count, descending
	sort.Slice(langs, func(i, j int) bool {
		return langMap[langs[i]] > langMap[langs[j]]
	})

	return langs, ""
}

// NewService constructs a code review service
func NewService(logger *zap.Logger, db db.DB, slackClient slack.Client, githubClient *github.Client, langIndex *lang.Index, slackChannelID string) Service {
	return &service{
		l:              logger.Sugar(),
		db:             db,
		slackClient:    slackClient,
		githubClient:   githubClient,
		langIndex:      langIndex,
		slackChannelID: slackChannelID,
	}
}

func pullRequestInfo(pullRequest *github.PullRequest, langs []string, langsErrorStatus string) slack.PullRequestInfo {
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
		BaseRef:          *pullRequest.Base.Ref,
		HeadRef:          *pullRequest.Head.Ref,
		Commits:          *pullRequest.Commits,
		ChangedFiles:     *pullRequest.ChangedFiles,
		Additions:        *pullRequest.Additions,
		Deletions:        *pullRequest.Deletions,
		State:            *pullRequest.State,
		Merged:           *pullRequest.Merged,
		Langs:            langs,
		LangsErrorStatus: langsErrorStatus,
	}
}
