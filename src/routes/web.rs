use actix_session::Session;
use actix_web::{web::Data, HttpResponse};
use askama::Template;

use crate::error::Result;
use crate::models::User;
use crate::utils::helpers::get_current_user;
use crate::AppConfig;

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate<'a> {
    client_id: &'a str,
    gh_client_id: &'a str,
    current_user: &'a Option<User>,
}

pub fn root(state: Data<AppConfig>, session: Session) -> Result<HttpResponse> {
    let current_user = &get_current_user(&state, &session)?;
    let r = RootTemplate {
        client_id: &state.slack.client_id,
        gh_client_id: &state.github_oauth.client_id,
        current_user,
    }
    .render()?;
    build_response(r)
}

fn build_response(body: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
