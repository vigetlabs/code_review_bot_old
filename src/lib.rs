use actix_web;

#[macro_use]
extern crate serde_derive;

mod github;
use crate::github::{GithubClient, ParseError, PullRequest};

mod slack;
use crate::slack::{SlackClient, SlackRequest};

mod utils;
pub use crate::utils::{load_languages, Languages};

mod routes;

use actix_web::middleware::Logger;
use actix_web::{error, http, server, App, Form, HttpResponse, ResponseError, State};
use listenfd::ListenFd;

pub struct AppConfig {
    pub github: GithubClient,
    pub slack: SlackClient,
    pub language_lookup: Languages,
}

impl AppConfig {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        github_url: String,
        github_token: &str,
        language_lookup: Languages,
    ) -> Result<Self, &'static str> {
        Ok(AppConfig {
            github: GithubClient::new(github_url, &github_token)?,
            slack: SlackClient::new()?,
            language_lookup,
        })
    }
}

impl ResponseError for ParseError {}


pub fn application(
    github_url: &str,
    github_token: &str,
    language_lookup: Languages,
) -> Result<App<AppConfig>, &'static str> {
    Ok(App::with_state(AppConfig::new(
        github_url.to_string(),
        github_token,
        language_lookup,
    )?)
    .middleware(Logger::default())
    .resource("/review", |r| {
        r.method(http::Method::POST)
            .with(routes::slack_webhook::route)
    }))
}

pub fn start_server(
    port: u32,
    github_token: String,
    language_lookup: Languages,
) -> Result<&'static str, std::io::Error> {
    server::new(move || {
        application(
            "https://api.github.com",
            &github_token,
            language_lookup.clone(),
        )
        .unwrap()
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run();

    Ok("Done")
}

pub fn start_dev_server(
    port: u32,
    github_token: String,
    language_lookup: Languages,
) -> Result<&'static str, std::io::Error> {
    let mut listenfd = ListenFd::from_env();
    let server = server::new(move || {
        application(
            "https://api.github.com",
            &github_token,
            language_lookup.clone(),
        )
        .unwrap()
    });

    if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)
    } else {
        server.bind(format!("0.0.0.0:{}", port))?
    }
    .run();

    Ok("Done")
}
