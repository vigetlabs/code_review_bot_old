use crate::utils::app_config::AppConfig;
use actix_web::{HttpRequest, HttpResponse, Result};
use askama::actix_web::TemplateIntoResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate;

pub fn root(_req: HttpRequest<AppConfig>) -> Result<HttpResponse> {
    RootTemplate.into_response()
}
