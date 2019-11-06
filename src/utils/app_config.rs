use actix_web::error::ErrorBadRequest;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use std::sync::{Arc, Mutex};

use crate::github::{GithubClient, GithubOauthClient};
use crate::slack::SlackClient;

#[derive(Clone)]
pub struct AppConfig {
    pub builder: AppDataBuilder,
    pub data: Arc<Mutex<Option<AppData>>>,
}

impl AppConfig {
    pub fn new(builder: AppDataBuilder, data: Option<AppData>) -> Self {
        Self {
            builder,
            data: Arc::new(Mutex::new(data)),
        }
    }
}

#[derive(Clone)]
pub struct AppData {
    pub github: GithubClient,
    pub github_oauth: GithubOauthClient,
    pub slack: SlackClient,
    pub app_url: String,
}

#[derive(Clone, Default)]
pub struct AppDataBuilder {
    github: GithubClient,
    app_url: Option<String>,
    github_oauth: Option<GithubOauthClient>,
    slack: Option<SlackClient>,
}

impl AppDataBuilder {
    pub fn github(mut self, client_id: &str, client_secret: &str) -> Self {
        self.github_oauth
            .replace(GithubOauthClient::new(client_id, client_secret));
        self
    }

    pub fn slack(
        mut self,
        client_id: &str,
        client_secret: &str,
        channel: &str,
        token: &str,
    ) -> Self {
        self.slack.replace(
            SlackClient::new(token, channel, client_id, client_secret)
                .expect("Error creating slack client"),
        );
        self
    }

    pub fn app_url(mut self, url: &str) -> Self {
        self.app_url.replace(url.to_string());
        self
    }

    pub fn build(mut self) -> Option<AppData> {
        Some(AppData {
            github: self.github,
            github_oauth: self.github_oauth.take()?,
            slack: self.slack.take()?,
            app_url: self.app_url.take()?,
        })
    }

    pub fn is_complete(&self) -> bool {
        let Self {
            github_oauth,
            slack,
            ..
        } = self;

        github_oauth.is_some() && slack.is_some()
    }
}

impl AppData {
    // TODO: Builder pattern
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AppDataBuilder {
        AppDataBuilder::default()
    }

    pub fn webhook_url(&self) -> String {
        format!("{}/github_event", self.app_url)
    }
}

impl FromRequest for AppData {
    type Error = Error;
    type Future = Result<Self, Self::Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        req.app_data::<AppConfig>()
            .ok_or_else(|| ErrorBadRequest("App data failed"))
            .and_then(|config| {
                config
                    .data
                    .lock()
                    .map_err(|_| ErrorBadRequest("App data failed"))
            })
            .and_then(|data| match &*data {
                Some(app_data) => Ok(app_data.clone()),
                None => Err(ErrorBadRequest("App data failed")),
            })
    }
}
