use crate::github::{PRAction, PullRequestEvent};
use crate::utils::app_config::AppConfig;
use actix_web::{error, HttpResponse, Json, State};
use diesel::RunQueryDsl;

use crate::models::{NewPullRequest, PullRequest};
use crate::schema::pull_requests;

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

    let new_pull_request = NewPullRequest {
      github_id: &format!(
        "{}-{}",
        pull_request.base.repo.full_name, pull_request.number
      ),
      state: "open",
      slack_message_id: &result.ts,
    };

    let connection = state.connection.lock().unwrap();

    diesel::insert_into(pull_requests::table)
      .values(&new_pull_request)
      .get_result::<PullRequest>(&*connection)
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
