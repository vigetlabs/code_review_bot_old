use actix_session::Session;
use actix_web::{
    http,
    web::{Data, Form, Path, Query},
    HttpResponse,
};
use askama::Template;

use crate::error::{Error, Result};
use crate::github;
use crate::models::{NewWebhook, User, Webhook};
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
    repos: &'a Vec<(&'a github::Repo, Option<&'a Webhook>)>,
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

        let webhooks = Webhook::for_repos(&github_repos.resources, &state.db)?;
        let repos = github_repos
            .resources
            .iter()
            .map(|r| {
                (
                    r,
                    webhooks
                        .iter()
                        .find(|w| w.owner == r.owner.login && w.name == r.name),
                )
            })
            .collect();

        IndexTemplate {
            repos: &repos,
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

#[derive(Deserialize)]
pub struct WebhookParams {
    owner: String,
    name: String,
}

pub fn create_webhook(
    form: Form<WebhookParams>,
    state: Data<AppConfig>,
    session: Session,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&state, &session)?.ok_or(Error::NotAuthedError)?;
    let webhook = state.github.create_webhook(
        &github::PullRequest {
            owner: form.owner.clone(),
            name: form.name.clone(),
            id: "".to_owned(),
        },
        &state.webhook_url,
        current_user.github_access_token,
    )?;
    Webhook::create(
        &NewWebhook {
            hook_id: format!("{}", webhook.id),
            name: form.name.clone(),
            owner: form.owner.clone(),
        },
        &state.db,
    )?;

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish())
}

pub fn delete_webhook(
    state: Data<AppConfig>,
    session: Session,
    path: Path<(i32,)>,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&state, &session)?.ok_or(Error::NotAuthedError)?;
    let webhook = Webhook::find(path.0, &state.db)?;
    state
        .github
        .delete_webhook(&webhook, current_user.github_access_token)?;
    webhook.delete(&state.db)?;

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish())
}

fn build_response(body: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
