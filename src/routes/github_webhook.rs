use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::error::{Error, Result};
use crate::github::{PRAction, PRReviewState, PullRequestEvent, ReviewAction, ReviewEvent};
use crate::models::{GithubUser, NewPullRequest, PullRequest, Review};
use crate::slack::Reaction;
use crate::utils::{app_config::AppConfig, prepare_response};

fn handle_pull_request_opened(
    state: Data<AppConfig>,
    json: PullRequestEvent,
) -> Result<HttpResponse> {
    if json.pull_request.draft {
        return Err(Error::GuardError("Ignoring Draft PR"));
    }

    let files = state
        .github
        .get_files(&json.pull_request, &state.language_lookup)?;
    let result = state
        .slack
        .post_message(&json.pull_request, &files, &state.slack.channel)?;

    let requester = GithubUser::find_or_create(&json.pull_request.user, &state.db)?;
    PullRequest::create(
        &NewPullRequest {
            github_id: github_id(
                &json.pull_request.base.repo.full_name,
                json.pull_request.number,
            ),
            state: "open".to_string(),
            slack_message_id: result.ts.unwrap_or_else(|| "".to_string()),
            channel: result.channel.unwrap_or_else(|| "".to_string()),
            display_text: format!("{}", json.pull_request),
            github_user_id: requester.github_id,
        },
        &state.db,
    )?;

    Ok(prepare_response(""))
}

fn handle_pull_request_closed(
    state: Data<AppConfig>,
    json: PullRequestEvent,
) -> Result<HttpResponse> {
    let db_pr = PullRequest::find(
        &github_id(
            &json.pull_request.base.repo.full_name,
            json.pull_request.number,
        ),
        &state.db,
    )?
    .update("closed", &state.db)?;

    let files = state
        .github
        .get_files(&json.pull_request, &state.language_lookup)?;
    state.slack.update_message(
        &json.pull_request,
        &files,
        &db_pr.slack_message_id,
        &db_pr.channel,
    )?;
    Ok(prepare_response(""))
}

pub fn pull_request(json: Json<PullRequestEvent>, state: Data<AppConfig>) -> Result<HttpResponse> {
    match json.action {
        PRAction::Opened | PRAction::ReadyForReview => handle_pull_request_opened(state, json.0),
        PRAction::Closed => handle_pull_request_closed(state, json.0),
        _ => Err(Error::GithubError(format!(
            "Unhandled PR Action: {:?}",
            json.action
        ))),
    }
}

pub fn review(json: Json<ReviewEvent>, state: Data<AppConfig>) -> Result<HttpResponse> {
    match json.action {
        ReviewAction::Submitted => handle_review_submitted(state, json.0),
        _ => Ok(prepare_response("")),
    }
}

fn handle_review_submitted(state: Data<AppConfig>, json: ReviewEvent) -> Result<HttpResponse> {
    if json.review.user.login == json.pull_request.user.login {
        return Err(Error::GuardError("Reviewer same as opened pull request"));
    }
    let mut reaction = Reaction::Comment;
    let mut approved = false;
    if let PRReviewState::Approved = json.review.state {
        reaction = Reaction::Approve;
        approved = true;
    }

    let reviewer = GithubUser::find_or_create(&json.review.user, &state.db)?;
    let mut db_pr = PullRequest::find(
        &github_id(
            &json.pull_request.base.repo.full_name,
            json.pull_request.number,
        ),
        &state.db,
    )?;
    db_pr = db_pr.update(&next_state(&db_pr.state, approved), &state.db)?;
    Review::create_or_update(&reviewer, &db_pr, &state.db)?;

    state
        .slack
        .add_reaction(&reaction, &db_pr.slack_message_id, &db_pr.channel)?;

    Ok(prepare_response(""))
}

fn github_id(repo: &str, number: u32) -> String {
    format!("{}-{}", repo, number)
}

fn next_state(state: &str, approved: bool) -> String {
    if state == "open" && approved {
        "approved".to_string()
    } else {
        state.to_string()
    }
}
