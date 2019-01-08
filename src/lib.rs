use actix_web;

#[macro_use]
extern crate serde_derive;

mod github;
use crate::github::{GithubClient, ParseError, PullRequest};

mod slack;
use crate::slack::{SlackClient, SlackRequest};

use actix_web::middleware::Logger;
use actix_web::{error, http, server, App, Form, HttpResponse, ResponseError, State};
use listenfd::ListenFd;

pub struct AppConfig {
    pub github: GithubClient,
    pub slack: SlackClient,
}

impl AppConfig {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(github_url: String, github_token: &str) -> Result<Self, &'static str> {
        Ok(AppConfig {
            github: GithubClient::new(github_url, &github_token)?,
            slack: SlackClient::new()?,
        })
    }
}

impl ResponseError for ParseError {}

fn code_review_bot(
    (form, state): (Form<SlackRequest>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
    if form.text.trim().is_empty() {
        let response = state.slack.immediate_response(
            "Specify pull request For example: /code_review_bot http://github.com/facebook/react/pulls/123".to_string(),
        )?;
        return prepare_response(response);
    }

    let url = form.text.to_lowercase().to_string();
    let pull_request: PullRequest = url.parse()?;
    let pr_response = state
        .github
        .get_pr(&pull_request)
        .map_err(error::ErrorNotFound)?;

    let pr_files = state
        .github
        .get_files(&pull_request)
        .map_err(error::ErrorNotFound)?;

    state
        .slack
        .response(pr_response, &pr_files, &form.response_url)
        .map_err(error::ErrorNotFound)?;

    prepare_response("".to_string())
}

fn prepare_response(body: String) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn application(github_url: &str, github_token: &str) -> Result<App<AppConfig>, &'static str> {
    Ok(
        App::with_state(AppConfig::new(github_url.to_string(), github_token)?)
            .middleware(Logger::default())
            .resource("/review", |r| {
                r.method(http::Method::POST).with(code_review_bot)
            }),
    )
}

pub fn start_server(port: u32, github_token: String) -> Result<&'static str, std::io::Error> {
    server::new(move || application("https://api.github.com", &github_token).unwrap())
        .bind(format!("0.0.0.0:{}", port))?
        .run();

    Ok("Done")
}

pub fn start_dev_server(port: u32, github_token: String) -> Result<&'static str, std::io::Error> {
    let mut listenfd = ListenFd::from_env();
    let server = server::new(move || application("https://api.github.com", &github_token).unwrap());

    if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)
    } else {
        server.bind(format!("0.0.0.0:{}", port))?
    }
    .run();

    Ok("Done")
}
