use crate::github::{PRAction, PullRequestEvent};
use crate::utils::app_config::AppConfig;
use actix_web::{error, HttpResponse, Json, State};
use futures::future::Future;

use crate::models::NewPullRequest;

pub fn route(
  (json, state): (Json<PullRequestEvent>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
  if let PRAction::Opened = json.action {
    let pull_request = &json.pull_request;
    let pr_files: String = state
      .github
      .get_files(pull_request, &state.language_lookup)
      .map_err(error::ErrorBadRequest)?;

    let result = state
      .slack
      .post_message(pull_request, &pr_files)
      .map_err(error::ErrorNotFound)?;

    let github_id = format!(
      "{}-{}",
      pull_request.base.repo.full_name, pull_request.number
    );
    let slack_message_id = result.ts;

    // TODO: Move this to a future -- This is not an ideal way to do this.
    // Currently it's waiting on the future to complete but should chain actions
    // on the future to be executed later
    let _ = state
      .db
      .send(NewPullRequest {
        github_id,
        state: "open".to_string(),
        slack_message_id,
      })
      .wait()
      .map_err(error::ErrorBadRequest)?;
  }

  prepare_response("".to_string())
}

fn prepare_response(body: String) -> actix_web::Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(body),
  )
}
