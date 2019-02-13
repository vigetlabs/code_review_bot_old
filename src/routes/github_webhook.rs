use crate::github::{PRAction, PRReviewState, PullRequestEvent, ReviewAction, ReviewEvent};
use crate::slack::Reaction;
use crate::utils::app_config::AppConfig;
use crate::utils::db::{FindPullRequest, UpdatePullReqeustState};
use actix_web::AsyncResponder;
use actix_web::{error, FutureResponse, HttpResponse, Json, State};
use futures::future;
use futures::future::Future;

use crate::models::NewPullRequest;

pub fn pull_request(
  (json, state): (Json<PullRequestEvent>, State<AppConfig>),
) -> FutureResponse<HttpResponse> {
  if let PRAction::Opened = json.action {
    let github = state.github.clone();
    let slack = state.slack.clone();
    let language_lookup = state.language_lookup.clone();
    let pull_request = json.pull_request.clone();
    let pull_request1 = json.pull_request.clone();

    return future::ok(0)
      .and_then(move |_| github.get_files(&pull_request, &language_lookup))
      .map_err(error::ErrorBadRequest)
      .and_then(move |files| {
        slack
          .post_message(&pull_request1, &files)
          .map_err(error::ErrorBadRequest)
      })
      .and_then(move |result| {
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
          .map_err(error::ErrorBadRequest)
      })
      .map_err(error::ErrorBadRequest)
      .and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body("")))
      .responder();
  } else if let PRAction::Closed = json.action {
    return state
      .db
      .send(UpdatePullReqeustState {
        github_id: github_id(
          &json.pull_request.base.repo.full_name,
          json.pull_request.number,
        ),
        state: "closed".to_string(),
      })
      .map_err(error::ErrorNotFound)
      .and_then(move |res| match res {
        Ok(db_pr) => {
          let files = state
            .github
            .get_files(&json.pull_request, &state.language_lookup)
            .map_err(error::ErrorBadRequest)?;

          state
            .slack
            .update_message(
              &json.pull_request,
              &files,
              &db_pr.slack_message_id,
              &db_pr.channel,
            )
            .map_err(error::ErrorNotFound)
        }
        Err(e) => Err(error::ErrorNotFound(e)),
      })
      .and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body("")))
      .responder();
  }

  future::ok(HttpResponse::Ok().content_type("application/json").body("")).responder()
}

pub fn review(
  (json, state): (Json<ReviewEvent>, State<AppConfig>),
) -> FutureResponse<HttpResponse> {
  if let ReviewAction::Submitted = json.action {
    let mut reaction = Reaction::Comment;
    let mut approved = false;
    if let PRReviewState::Approved = json.review.state {
      reaction = Reaction::Approve;
      approved = true;
    }

    return state
      .db
      .send(FindPullRequest {
        github_id: github_id(
          &json.pull_request.base.repo.full_name,
          json.pull_request.number,
        ),
      })
      .map_err(error::ErrorNotFound)
      .and_then(move |res| res.map_err(error::ErrorNotFound))
      .and_then(move |db_pr| {
        let message_id = db_pr.slack_message_id;
        let channel = db_pr.channel;

        state
          .db
          .send(UpdatePullReqeustState {
            github_id: db_pr.github_id,
            state: next_state(&db_pr.state, approved),
          })
          .map_err(error::ErrorNotFound)
          .and_then(move |_| {
            state
              .slack
              .add_reaction(&reaction, &message_id, &channel)
              .map_err(error::ErrorNotFound)
          })
      })
      .and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body("")))
      .responder();
  }

  future::ok(HttpResponse::Ok().content_type("application/json").body("")).responder()
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
