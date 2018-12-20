extern crate actix_web;
extern crate listenfd;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate reqwest;

mod github;
use github::{GithubClient, ParseError, PullRequest};

use actix_web::middleware::Logger;
use actix_web::{http, server, App, Form, HttpResponse, ResponseError, State};
use listenfd::ListenFd;

#[derive(Serialize, Debug)]
struct SlackResponse {
    text: String,
    response_type: String,
}

#[derive(Deserialize, Debug)]
struct SlackRequest {
    text: String,
    token: String,
}

pub struct AppConfig {
    pub github: GithubClient,
}

impl AppConfig {
    pub fn new(github_url: String) -> AppConfig {
        AppConfig {
            github: GithubClient::new(github_url),
        }
    }
}

impl ResponseError for ParseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().body(format!("Something went wrong: {}", self))
    }
}

fn code_review_bot(
    (form, state): (Form<SlackRequest>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
    if form.text.trim().is_empty() {
        return prepare_response(
            "Specify repository name to search. \
             For example: /code_review_bot linux"
                .to_string(),
        );
    }

    let url = form.text.to_lowercase().to_string();
    let repo: PullRequest = url.parse()?;
    let response_body = match state.github.search(&repo.name, 10) {
        Ok(result) => format!("{}", &result),
        Err(e) => format!("Something went wrong: {}", e),
    };

    prepare_response(response_body)
}

fn prepare_response(text: String) -> actix_web::Result<HttpResponse> {
    let body = serde_json::to_string(&SlackResponse {
        text: text,
        response_type: "in_channel".to_string(),
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn application(github_url: String) -> App<AppConfig> {
    App::with_state(AppConfig::new(github_url))
        .middleware(Logger::default())
        .resource("/review", |r| {
            r.method(http::Method::POST).with(code_review_bot)
        })
}

pub fn start_server(port: u32) -> Result<&'static str, std::io::Error> {
    server::new(move || application("https://api.github.com".to_string()))
        .bind(format!("0.0.0.0:{}", port))?
        .run();

    Ok("Done")
}

pub fn start_dev_server(port: u32) -> Result<&'static str, std::io::Error> {
    let mut listenfd = ListenFd::from_env();
    let server = server::new(move || application("https://api.github.com".to_string()));

    if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)
    } else {
        server.bind(format!("0.0.0.0:{}", port))?
    }.run();

    Ok("Done")
}
