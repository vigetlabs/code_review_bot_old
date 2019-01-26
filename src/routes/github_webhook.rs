use crate::github::PullRequestEvent;
use crate::utils::app_config::AppConfig;
use actix_web::{HttpResponse, Json, State};

pub fn route(
  (json, _state): (Json<PullRequestEvent>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
  println!("{:#?}", json);
  prepare_response("".to_string())
}

fn prepare_response(body: String) -> actix_web::Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(body),
  )
}
