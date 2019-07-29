use crate::AppConfig;
use actix_web::{http, HttpResponse, Query, State};

#[derive(Deserialize)]
pub struct SlackAuthQuery {
    code: String,
    state: Option<String>,
}

pub fn slack(state: State<AppConfig>, query: Query<SlackAuthQuery>) -> HttpResponse {
    // state.slack.get_token(query.code);
    redirect_to("/")
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
