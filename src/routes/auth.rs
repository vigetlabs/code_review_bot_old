use actix_session::Session;
use actix_web::{
    http,
    web::{Data, Query},
    HttpResponse,
};

use crate::db::DBExecutor;
use crate::error::Result;
use crate::models::{NewUser, User};
use crate::utils::app_config::AppData;
use crate::utils::helpers;

#[derive(Deserialize)]
pub struct AuthRedirect {
    code: String,
}

pub async fn slack(
    state: AppData,
    db: Data<DBExecutor>,
    query: Query<AuthRedirect>,
    session: Session,
) -> Result<HttpResponse> {
    let response = state.slack.get_token(&query.code).await?;
    let user_data = response.authed_user;
    let user = User::create_or_udpate(
        &NewUser {
            slack_user_id: user_data.id,
            slack_access_token: user_data.access_token,
        },
        &db,
    )?;
    session.set("id", user.id)?;

    Ok(redirect_to("/"))
}

pub async fn github(
    state: AppData,
    db: Data<DBExecutor>,
    query: Query<AuthRedirect>,
    session: Session,
) -> Result<HttpResponse> {
    let response = state.github_oauth.get_token(&query.code).await?;
    let github_user = state.github.get_user(&response.access_token).await?;
    let user =
        helpers::get_current_user(&db, &session)?.ok_or(crate::error::Error::NotFoundError)?;
    user.connect_to_github_user(&response.access_token, &github_user, &db)?;

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
