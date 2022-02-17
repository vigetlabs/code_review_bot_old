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

	getFiles := action != "synchronize"
	pr, err := s.db.PullRequest(ctx, repoID, pullRequestID, false, getFiles, true)
	if err != nil {
		return err
	}

	// Fetch files if new PR, changed PR content (synchronize), or no DB files, otherwise use DB files
	var (
		files          []*github.CommitFile
		filesErrStatus string
	)
	fetchFiles := pr == nil || action == "synchronize" || pr.Files == nil
	if fetchFiles {
		files, filesErrStatus, err = s.fetchFiles(ctx, event.PullRequest)
		if err != nil {
			s.l.Errorw("fetchFiles", "status", filesErrStatus, "err", err)
		}
	} else {
		files = pr.Files
	}

	var approvals []*github.User
	if pr != nil {
		approvals = pr.Approvals
	}

	langs := s.langsForFiles(files)
	info := pullRequestInfo(event.PullRequest, langs, filesErrStatus, approvals)

	// If the PR isn't in the db, send a new message, otherwise update the existing message
	if pr == nil {
		channelID, timestamp, err := s.slackClient.SendPullRequestMessage(ctx, s.slackChannelID, info)
		if err != nil {
			return err
		}

		if err := s.slackClient.AddReactionIfNotExists(ctx, channelID, timestamp, "eyes"); err != nil {
			return err
		}

		return s.db.PutPullRequest(ctx, &db.PullRequest{
			RepoID:                repoID,
			PullRequestID:         pullRequestID,
			SlackChannelID:        channelID,
			SlackMessageTimestamp: timestamp,
			Files:                 files,
			Approvals:             nil,
		})
	}

	if err := s.slackClient.UpdatePullRequestMessage(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp, info); err != nil {
		return err
	}

	pr.PR = event.PullRequest
	pr.Files = files
	return s.db.UpdatePullRequest(ctx, pr, true, fetchFiles, false)
}

func (s *service) HandlePullRequestReviewEvent(ctx context.Context, event github.PullRequestReviewEvent) error {
	action := *event.Action

	s.l.Debugw("HandlePullRequestReviewEvent", "action", action, "state", *event.Review.State)

	if !(action == "submitted") {
		return nil
	}

	repoID := *event.PullRequest.Base.Repo.ID
	pullRequestID := *event.PullRequest.ID

	pr, err := s.db.PullRequest(ctx, repoID, pullRequestID, true, true, true)
	if err != nil {
		return err
	} else if pr == nil {
		return nil
	}

	state := *event.Review.State
	if state != "approved" {
		if err := s.slackClient.AddReactionIfNotExists(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp, "memo"); err != nil {
			return err
		}
	}

	updateApprovals := false
	if state == "approved" {
		login := *event.Review.User.Login
		updateApprovals = true
		for _, l := range pr.Approvals {
			if *l.Login == login {
				updateApprovals = false
				break
			}
		}

		if updateApprovals {
			pr.Approvals = append(pr.Approvals, event.Review.User)
		}
	}

	if len(pr.Approvals) >= 2 {
		if err := s.slackClient.AddReactionIfNotExists(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp, "shipit"); err != nil {
			return err
		}
	}

	if updateApprovals {
		langs := s.langsForFiles(pr.Files)
		info := pullRequestInfo(pr.PR, langs, "", pr.Approvals)
		if err := s.slackClient.UpdatePullRequestMessage(ctx, pr.SlackChannelID, pr.SlackMessageTimestamp, info); err != nil {
			return err
		}

		if err := s.db.UpdatePullRequest(ctx, pr, false, false, true); err != nil {
			return err
		}
	}

	return nil
}

func (s *service) fetchFiles(ctx context.Context, pr *github.PullRequest) ([]*github.CommitFile, string, error) {
	owner := *pr.Base.Repo.Owner.Login
	repo := *pr.Base.Repo.Name
	number := *pr.Number

	files, resp, err := s.githubClient.PullRequests.ListFiles(ctx, owner, repo, number, &github.ListOptions{PerPage: 100})
	if err != nil {
		if resp != nil {
			return nil, resp.Status, err
		}
		return nil, "unknown", err
	}

	return files, "", nil
}

func (s *service) langsForFiles(files []*github.CommitFile) []string {
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

	return langs
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

func pullRequestInfo(pullRequest *github.PullRequest, langs []string, filesErrStatus string, approvals []*github.User) slack.PullRequestInfo {
	var name string
	if pullRequest.User.Name != nil {
		name = *pullRequest.User.Name
	} else {
		name = *pullRequest.User.Login
	}

	users := make([]slack.PullRequestApprovalUser, len(approvals))
	for i, u := range approvals {
		users[i] = slack.PullRequestApprovalUser{
			AvatarURL: *u.AvatarURL,
			Login:     *u.Login,
		}
	}

	return slack.PullRequestInfo{
		UserName:       name,
		UserAvatarURL:  *pullRequest.User.AvatarURL,
		UserLogin:      *pullRequest.User.Login,
		Title:          *pullRequest.Title,
		URL:            *pullRequest.HTMLURL,
		Repo:           *pullRequest.Base.Repo.FullName,
		BaseRef:        *pullRequest.Base.Ref,
		HeadRef:        *pullRequest.Head.Ref,
		Commits:        *pullRequest.Commits,
		ChangedFiles:   *pullRequest.ChangedFiles,
		Additions:      *pullRequest.Additions,
		Deletions:      *pullRequest.Deletions,
		State:          *pullRequest.State,
		Merged:         *pullRequest.Merged,
		Langs:          langs,
		FilesErrStatus: filesErrStatus,
		Approvals:      users,
	}
}
