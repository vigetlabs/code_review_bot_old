use crate::github::PullRequest;
use crate::slack::SlackRequest;
use crate::AppConfig;
use actix_web::{error, Form, HttpResponse, State};

pub fn route(
  (form, state): (Form<SlackRequest>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
  if form.text.trim().is_empty() {
    let response = state.slack.immediate_response(
            "Specify pull request For example: /code_review_bot http://github.com/facebook/react/pulls/123".to_string(),
        )?;
    return prepare_response(response);
  }

  let url = form.text.to_lowercase().to_string();
  let pull_request: PullRequest = url.parse()?;
  let pr_response = state
    .github
    .get_pr(&pull_request)
    .map_err(error::ErrorNotFound)?;

  let pr_files: String = state
    .github
    .get_files(&pr_response, &state.language_lookup)
    .map_err(error::ErrorBadRequest)?;

  state
    .slack
    .response(&pr_response, &pr_files, &form.response_url)
    .map_err(error::ErrorNotFound)?;

  prepare_response("".to_string())
}

fn prepare_response(body: String) -> actix_web::Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(body),
  )
}
