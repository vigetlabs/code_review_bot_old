use actix_web::AsyncResponder;
use actix_web::{FromRequest, Json, State};
use futures::future;
use futures::future::Future;

use crate::error::Error;
use crate::github::{PRAction, PRReviewState, PullRequestEvent, ReviewAction, ReviewEvent};
use crate::models;
use crate::models::NewPullRequest;
use crate::slack::Reaction;
use crate::utils::app_config::AppConfig;
use crate::utils::db::{FindPullRequest, UpdatePullRequestState};
use crate::utils::{prepare_response, RequestAction, Response};

fn handle_pull_request_opened(
    state: State<AppConfig>,
    json: PullRequestEvent,
    is_auto_webhook: bool,
) -> Response {
    if is_auto_webhook {
        return future::err(Error::GuardError("Ignoring for automatic webhook")).responder();
    }

    if json.pull_request.draft {
        return future::err(Error::GuardError("Ignoring Draft PR")).responder();
    }

    future::result(
        state
            .github
            .get_files(&json.pull_request, &state.language_lookup)
            .map(|files| RequestAction::new(state, json, files)),
    )
    .and_then(|pr_open| {
        let RequestAction {
            state,
            json,
            value: files,
        } = &pr_open;
        state
            .slack
            .post_message(&json.pull_request, &files, &state.slack.channel)
            .map(|result| pr_open.with_value(result))
    })
    .and_then(|pr_open| {
        let RequestAction {
            state,
            json,
            value: result,
        } = pr_open;
        state
            .db
            .send(NewPullRequest {
                github_id: github_id(
                    &json.pull_request.base.repo.full_name,
                    json.pull_request.number,
                ),
                state: "open".to_string(),
                slack_message_id: result.ts.unwrap_or_else(|| "".to_string()),
                channel: result.channel.unwrap_or_else(|| "".to_string()),
                display_text: format!("{}", json.pull_request),
            })
            .map_err(|e| e.into())
    })
    .map(|_| prepare_response(""))
    .responder()
}

fn handle_pull_request_closed(
    state: State<AppConfig>,
    json: PullRequestEvent,
    is_auto_webhook: bool,
) -> Response {
    let update_state = state
        .db
        .send(UpdatePullRequestState {
            github_id: github_id(
                &json.pull_request.base.repo.full_name,
                json.pull_request.number,
            ),
            state: "closed".to_string(),
        })
        .map_err(|e| e.into())
        .and_then(|res| res)
        .map(|db_pr| RequestAction::new(state, json, db_pr));

    if is_auto_webhook {
        return update_state.map(|_| prepare_response("")).responder();
    }

    update_state
        .and_then(|pr_open| {
            let RequestAction { state, json, .. } = &pr_open;

            state
                .github
                .get_files(&json.pull_request, &state.language_lookup)
                .map(|files| pr_open.add_value(files))
        })
        .and_then(|pr_open| {
            let RequestAction {
                state,
                json,
                value: (db_pr, files),
            } = pr_open;

            state.slack.update_message(
                &json.pull_request,
                &files,
                &db_pr.slack_message_id,
                &db_pr.channel,
            )
        })
        .map(|_| prepare_response(""))
        .responder()
}

pub fn pull_request(req: actix_web::HttpRequest<AppConfig>) -> Response {
    let state = State::<AppConfig>::extract(&req);
    let is_auto_webhook = req.headers().get("X-Hub-Signature").is_some();
    Json::<PullRequestEvent>::extract(&req)
        .map(|json| (state, json.0))
        .map_err(|e| e.into())
        .and_then(move |(state, json)| match json.action {
            PRAction::Opened | PRAction::ReadyForReview => {
                handle_pull_request_opened(state, json, is_auto_webhook)
            }
            PRAction::Closed => handle_pull_request_closed(state, json, is_auto_webhook),
            _ => future::err(Error::GithubError(format!(
                "Unhandled PR Action: {:?}",
                json.action
            )))
            .responder(),
        })
        .responder()
}

pub fn review((json, state): (Json<ReviewEvent>, State<AppConfig>)) -> Response {
    match json.action {
        ReviewAction::Submitted => handle_review_submitted(state, json.0),
        _ => future::ok(prepare_response("")).responder(),
    }
}

fn handle_review_submitted(state: State<AppConfig>, json: ReviewEvent) -> Response {
    if json.review.user.login == json.pull_request.user.login {
        return future::err(Error::GuardError("Reviewer same as opened pull request")).responder();
    }
    let mut reaction = Reaction::Comment;
    let mut approved = false;
    if let PRReviewState::Approved = json.review.state {
        reaction = Reaction::Approve;
        approved = true;
    }

    state
        .db
        .send(FindPullRequest {
            github_id: github_id(
                &json.pull_request.base.repo.full_name,
                json.pull_request.number,
            ),
        })
        .map_err(|e| e.into())
        .and_then(|res| res)
        .and_then(move |db_pr| {
            let models::PullRequest {
                github_id,
                state: pr_state,
                ..
            } = &db_pr;

            state
                .db
                .send(UpdatePullRequestState {
                    github_id: github_id.to_string(),
                    state: next_state(&pr_state, approved),
                })
                .map(|_| RequestAction::new(state, json, db_pr))
                .map_err(|e| e.into())
        })
        .and_then(move |req_action| {
            let RequestAction {
                state,
                value:
                    models::PullRequest {
                        slack_message_id,
                        channel,
                        ..
                    },
                ..
            } = req_action;
            state
                .slack
                .add_reaction(&reaction, &slack_message_id, &channel)
        })
        .map(|_| prepare_response(""))
        .responder()
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
