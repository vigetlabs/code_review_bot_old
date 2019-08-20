use crate::AppConfig;
use actix_web::{
    http,
    web::{Data, Query},
    HttpResponse,
};

#[derive(Deserialize)]
pub struct SlackAuthQuery {
    code: String,
}

pub fn slack(state: Data<AppConfig>, query: Query<SlackAuthQuery>) -> HttpResponse {
    let access_token = state.slack.get_token(&query.code);
    redirect_to("/")
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
