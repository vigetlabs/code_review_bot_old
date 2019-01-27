use crate::github::{PRAction, PullRequestEvent};
use crate::utils::app_config::AppConfig;
use actix_web::{error, HttpResponse, Json, State};

pub fn route(
  (json, state): (Json<PullRequestEvent>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
  if let PRAction::Opened = json.action {
    let pull_request = &json.pull_request;
    let pr_files: String = state
      .github
      .get_files(pull_request)
      .map_err(error::ErrorNotFound)?
      .iter()
      .filter_map(|ext| state.language_lookup.get(ext))
      .map(|icon| icon.to_string())
      .collect::<Vec<String>>()
      .join(" ");

    state
      .slack
      .post_message(pull_request, &pr_files)
      .map_err(error::ErrorNotFound)?;
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
