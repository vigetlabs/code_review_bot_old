use actix_session::Session;
use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use actix_web_flash::{FlashMessage, FlashResponse};
use askama::Template;
use std::fmt;

use crate::db::DBExecutor;
use crate::error::{self, Result};
use crate::models::{Config, User};
use crate::utils::helpers::{get_current_user, sign_out_current_user};
use crate::{AppConfig, AppData};

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
    info: &'a Info<'a>,
}

#[derive(Template)]
#[template(path = "home/index.html")]
struct IndexTemplate<'a> {
    flash: &'a Option<Flash>,
    info: &'a Info<'a>,
}

struct Info<'a> {
    client_id: &'a str,
    gh_client_id: &'a str,
    current_user: &'a Option<User>,
}

pub async fn root(
    state: AppData,
    db: Data<DBExecutor>,
    session: Session,
    flash_message: Option<FlashMessage<Flash>>,
) -> Result<HttpResponse> {
    let flash = flash_message.map(|flash| flash.into_inner());
    let current_user = get_current_user(&db, &session)?;
    let is_gh_authed = current_user
        .clone()
        .and_then(|u| if u.is_gh_authed() { Some(u) } else { None })
        .is_some();

    let info = Info {
        client_id: &state.slack.client_id,
        gh_client_id: &state.github_oauth.client_id,
        current_user: &current_user,
    };

    let rendered_template = if is_gh_authed {
        IndexTemplate {
            flash: &flash,
            info: &info,
        }
        .render()?
    } else {
        LoginTemplate { info: &info }.render()?
    };

    build_response(rendered_template)
}

#[derive(Template)]
#[template(path = "setup/new.html")]
struct NewSetup;

pub async fn new_setup() -> Result<HttpResponse> {
    build_response(NewSetup.render()?)
}

#[derive(Deserialize)]
pub struct SetupData {
    slack_client_id: String,
    slack_client_secret: String,
    slack_token: String,
    slack_channel: String,
    github_client_id: String,
    github_client_secret: String,
    app_url: String,
}

pub async fn create_setup(
    form: Form<SetupData>,
    db: Data<DBExecutor>,
    config: Data<AppConfig>,
) -> Result<FlashResponse<HttpResponse, Flash>> {
    let mut builder = config.builder.lock().expect("Builder not available");

    *builder = builder
        .clone()
        .slack(
            &form.slack_client_id,
            &form.slack_client_secret,
            &form.slack_channel,
            &form.slack_token,
        )
        .github(&form.github_client_id, &form.github_client_secret)
        .app_url(&form.app_url);

    Config::create(
        &[
            Config::new("slack_client_id", &form.slack_client_id),
            Config::new("slack_client_secret", &form.slack_client_secret),
            Config::new("slack_channel", &form.slack_channel),
            Config::new("slack_token", &form.slack_token),
            Config::new("github_client_id", &form.github_client_id),
            Config::new("github_client_secret", &form.github_client_secret),
            Config::new("app_url", &form.app_url),
        ],
        &db,
    )?;

    Ok(FlashResponse::with_redirect(
        Flash::from_result(Ok(()), "Setup Complete!"),
        "/",
    ))
}

pub async fn logout(
    db: Data<DBExecutor>,
    session: Session,
) -> Result<FlashResponse<HttpResponse, Flash>> {
    let current_user = get_current_user(&db, &session)
        .and_then(|user| user.ok_or(error::Error::NotAuthedError))
        .map_err(|e| {
            println!("{:?}", e);
            e
        })?;
    current_user.logout(&db).map_err(|e| {
        println!("{:?}", e);
        e
    })?;

    sign_out_current_user(&session);
    Ok(FlashResponse::with_redirect(Flash::info("Signed Out"), "/"))
}

fn build_response(body: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
