use actix_web::{web::Data, HttpResponse};
use askama::Template;

use crate::error::Result;
use crate::AppConfig;

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate<'a> {
    client_id: &'a str,
}

pub fn root(state: Data<AppConfig>) -> Result<HttpResponse> {
    let r = RootTemplate {
        client_id: &state.slack.client_id,
    }
    .render()?;
    build_response(r)
}

fn build_response(body: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
