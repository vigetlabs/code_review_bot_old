use actix_session::Session;
use actix_web::{
    http,
    web::{Data, Query},
    HttpResponse,
};

use crate::error::Result;
use crate::models::{NewUser, User};
use crate::AppConfig;

#[derive(Deserialize)]
pub struct SlackAuthQuery {
    code: String,
}

pub fn slack(
    state: Data<AppConfig>,
    query: Query<SlackAuthQuery>,
    session: Session,
) -> Result<HttpResponse> {
    let response = state.slack.get_token(&query.code)?;
    let access_token = response.access_token.unwrap();
    let user_data = response.user.unwrap();
    let user = User::create_or_udpate(
        &NewUser {
            username: user_data.name,
            slack_user_id: user_data.id,
            slack_access_token: access_token,
        },
        &state.db,
    )?;
    session.set("id", user.id)?;

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
