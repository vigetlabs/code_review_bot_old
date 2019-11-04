use actix_session::Session;
use actix_web::{
    web::{Data, Form, Path, Query},
    HttpResponse,
};
use actix_web_flash::{FlashMessage, FlashResponse};
use askama::Template;
use std::fmt;

use crate::error::{Error, Result};
use crate::github;
use crate::models::{NewWebhook, User, Webhook};
use crate::utils::helpers::get_current_user;
use crate::utils::paginated_resource;
use crate::AppConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flash {
    message_type: FlashType,
    message: String,
}

impl Flash {
    fn info(message: &str) -> Self {
        Self {
            message_type: FlashType::Info,
            message: message.to_owned(),
        }
    }

    fn err(message: &str) -> Self {
        Self {
            message_type: FlashType::Error,
            message: message.to_owned(),
        }
    }

    fn from_result<T>(result: Result<T>, message: &str) -> Self {
        match result {
            Ok(_) => Self::info(message),
            Err(_) => Self::err("Something went wrong!"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FlashType {
    Info,
    Warn,
    Error,
}

impl fmt::Display for FlashType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

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
    flash: &'a Option<Flash>,
}

pub fn root(
    state: Data<AppConfig>,
    session: Session,
    params: Query<paginated_resource::PaginationParams>,
    flash_message: Option<FlashMessage<Flash>>,
) -> Result<HttpResponse> {
    let flash = flash_message.map(|flash| flash.into_inner());
    let current_user = get_current_user(&state, &session)?;
    let is_gh_authed = current_user
        .clone()
        .and_then(|u| if u.is_gh_authed() { Some(u) } else { None })
        .is_some();

    let rendered_template = if is_gh_authed {
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
            flash: &flash,
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

    build_response(rendered_template)
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
) -> Result<FlashResponse<HttpResponse, Flash>> {
    let current_user = get_current_user(&state, &session)?.ok_or(Error::NotAuthedError)?;

    let result = state
        .github
        .create_webhook(
            &github::ReviewRequest {
                owner: form.owner.clone(),
                name: form.name.clone(),
                id: "".to_owned(),
            },
            &state.webhook_url,
            current_user.github_access_token,
        )
        .and_then(|webhook| {
            Webhook::create(
                &NewWebhook {
                    hook_id: format!("{}", webhook.id),
                    name: form.name.clone(),
                    owner: form.owner.clone(),
                },
                &state.db,
            )
        });

    Ok(FlashResponse::with_redirect(
        Flash::from_result(result, "Webhook Created!"),
        "/",
    ))
}

pub fn delete_webhook(
    state: Data<AppConfig>,
    session: Session,
    path: Path<(i32,)>,
) -> Result<FlashResponse<HttpResponse, Flash>> {
    let current_user = get_current_user(&state, &session)?.ok_or(Error::NotAuthedError)?;
    let result = Webhook::find(path.0, &state.db).and_then(|webhook| {
        state
            .github
            .delete_webhook(&webhook, current_user.github_access_token)?;
        webhook.delete(&state.db)
    });

    Ok(FlashResponse::with_redirect(
        Flash::from_result(result, "Webhook Deleted!"),
        "/",
    ))
}

fn build_response(body: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
