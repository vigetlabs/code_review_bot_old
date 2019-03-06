use crate::github::{PRAction, PRReviewState, PullRequestEvent, ReviewAction, ReviewEvent};
use crate::slack::Reaction;
use crate::utils::app_config::AppConfig;
use crate::utils::db::{FindPullRequest, UpdatePullRequestState};
use actix_web::AsyncResponder;
use actix_web::{error, FromRequest, FutureResponse, HttpResponse, Json, State};
use futures::future;
use futures::future::Future;

use crate::models::NewPullRequest;

fn handle_pull_request_opened(
  state: State<AppConfig>,
  json: PullRequestEvent,
  header_exists: bool,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
  let happy_path = future::ok(0)
    .and_then(move |_| {
      state
        .github
        .get_files(&json.pull_request, &state.language_lookup)
        .map(|files| (state, json, files))
    })
    .map_err(error::ErrorBadRequest)
    .and_then(move |(state, json, files)| {
      state
        .slack
        .post_message(&json.pull_request, &files, &state.slack.channel)
        .map(|result| (state, json, result))
        .map_err(error::ErrorBadRequest)
    })
    .and_then(move |(state, json, result)| {
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
    .and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body("")));

  if header_exists {
    future::Either::A(
      future::ok(0).and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body(""))),
    )
  } else {
    future::Either::B(happy_path)
  }
}

fn handle_pull_request_closed(
  state: State<AppConfig>,
  json: PullRequestEvent,
  header_exists: bool,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
  let update_state = state
    .db
    .send(UpdatePullRequestState {
      github_id: github_id(
        &json.pull_request.base.repo.full_name,
        json.pull_request.number,
      ),
      state: "closed".to_string(),
    })
    .map_err(error::ErrorNotFound)
    .and_then(|res| res.map_err(error::ErrorNotFound))
    .map(|db_pr| (json, db_pr));

  if header_exists {
    future::Either::A(
      update_state.and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body(""))),
    )
  } else {
    future::Either::B(
      update_state
        .and_then(|(json, db_pr)| {
          state
            .github
            .get_files(&json.pull_request, &state.language_lookup)
            .map(|files| (state, json, db_pr, files))
            .map_err(error::ErrorBadRequest)
        })
        .and_then(|(state, json, db_pr, files)| {
          state
            .slack
            .update_message(
              &json.pull_request,
              &files,
              &db_pr.slack_message_id,
              &db_pr.channel,
            )
            .map_err(error::ErrorNotFound)
        })
        .and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body(""))),
    )
  }
}

pub fn pull_request(req: actix_web::HttpRequest<AppConfig>) -> FutureResponse<HttpResponse> {
  let state = State::<AppConfig>::extract(&req);
  let header_exists = req.headers().get("X-Hub-Signature").is_some();
  Json::<PullRequestEvent>::extract(&req)
    .map(|json| (state, json.0))
    .and_then(move |(state, json)| match json.action {
      PRAction::Opened => future::Either::A(future::Either::A(handle_pull_request_opened(
        state,
        json,
        header_exists,
      ))),
      PRAction::Closed => future::Either::A(future::Either::B(handle_pull_request_closed(
        state,
        json,
        header_exists,
      ))),
      _ => future::Either::B(future::err(error::ErrorNotFound("Unhanlded PR Action"))),
    })
    .responder()
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

    return if json.review.user.login != json.pull_request.user.login {
      future::ok(())
    } else {
      future::err(error::ErrorNotFound("Reviewer same as opened pull request"))
    }
    .and_then(move |_| {
      state
        .db
        .send(FindPullRequest {
          github_id: github_id(
            &json.pull_request.base.repo.full_name,
            json.pull_request.number,
          ),
        })
        .map_err(error::ErrorNotFound)
        .and_then(|res| res.map_err(error::ErrorNotFound))
        .map(|db_pr| (state, db_pr))
    })
    .and_then(move |(state, db_pr)| {
      let message_id = db_pr.slack_message_id;
      let channel = db_pr.channel;

      state
        .db
        .send(UpdatePullRequestState {
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
