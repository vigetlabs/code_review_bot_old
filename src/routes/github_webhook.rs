use crate::github::{PRAction, PullRequestEvent};
use crate::utils::app_config::AppConfig;
use actix_web::AsyncResponder;
use actix_web::{error, FutureResponse, HttpResponse, Json, State};
use futures::future;
use futures::future::Future;

use crate::models::NewPullRequest;

pub fn route(
  (json, state): (Json<PullRequestEvent>, State<AppConfig>),
) -> FutureResponse<HttpResponse> {
  if let PRAction::Opened = json.action {
    let github = state.github.clone();
    let slack = state.slack.clone();
    let language_lookup = state.language_lookup.clone();
    let pull_request = json.pull_request.clone();
    let pull_request2 = pull_request.clone();

    return future::ok(0)
      .and_then(move |_| github.get_files(&pull_request, &language_lookup))
      .map_err(|_| "Couldn't get files")
      .and_then(move |files| slack.post_message(&pull_request2, &files))
      .map_err(error::ErrorBadRequest)
      .and_then(move |res| {
        let github_id = format!(
          "{}-{}",
          &json.pull_request.base.repo.full_name, &json.pull_request.number
        );
        let slack_message_id = res.ts;

        state
          .db
          .send(NewPullRequest {
            github_id,
            state: "open".to_string(),
            slack_message_id,
          })
          .map_err(error::ErrorBadRequest)
      })
      .and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body("")))
      .responder();
  }

  Box::new(future::ok(
    HttpResponse::Ok().content_type("application/json").body(""),
  ))
}
