use actix_session::Session;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use askama::Template;

use crate::error::Result;
use crate::github;
use crate::models::User;
use crate::utils::helpers::get_current_user;
use crate::utils::paginated_resource;
use crate::AppConfig;

#[derive(Template)]
#[template(path = "home/login.html")]
struct LoginTemplate<'a> {
    client_id: &'a str,
    gh_client_id: &'a str,
    current_user: &'a Option<User>,
}

#[derive(Template)]
#[template(path = "home/index.html")]
struct IndexTemplate<'a> {
    repos: &'a Vec<github::Repo>,
    pagination: &'a paginated_resource::PaginatedResource<github::Repo>,
}

pub fn root(
    state: Data<AppConfig>,
    session: Session,
    params: Query<paginated_resource::PaginationParams>,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&state, &session)?;
    let is_gh_authed = current_user
        .clone()
        .and_then(|u| if u.is_gh_authed() { Some(u) } else { None })
        .is_some();

    let r = if is_gh_authed {
        let user = current_user.unwrap();
        let github_repos = state
            .github
            .get_repos(&user.github_access_token.unwrap(), &*params)?;
        IndexTemplate {
            repos: &github_repos.resources,
            pagination: &github_repos,
        }
        .render()?
    } else {
        LoginTemplate {
            client_id: &state.slack.client_id,
            gh_client_id: &state.github_oauth.client_id,
            current_user: &current_user,
        }
        .render()?
    };

    build_response(r)
}

fn build_response(body: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
