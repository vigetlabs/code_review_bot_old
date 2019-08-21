use actix_session::Session;
use actix_web::{web::Data, HttpResponse};
use askama::Template;

use crate::error::Result;
use crate::models::User;
use crate::AppConfig;

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate<'a> {
    client_id: &'a str,
    current_user: &'a Option<User>,
}

pub fn root(state: Data<AppConfig>, session: Session) -> Result<HttpResponse> {
    let current_user = &get_current_user(&state, &session)?;
    let r = RootTemplate {
        client_id: &state.slack.client_id,
        current_user,
    }
    .render()?;
    build_response(r)
}

fn get_current_user(state: &Data<AppConfig>, session: &Session) -> Result<Option<User>> {
    if let Some(id) = session.get("id")? {
        let conn = state.db.0.get()?;
        User::find(id, conn)
    } else {
        Ok(None)
    }
}

fn build_response(body: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
