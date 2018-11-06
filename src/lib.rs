extern crate actix_web;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate reqwest;

mod github;
use github::GithubClient;

use actix_web::{http, server, App, Form, HttpResponse, State};

#[derive(Serialize)]
struct SlackResponse {
    text: String,
    response_type: String,
}

#[derive(Deserialize)]
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

fn code_review_bot(
    (form, state): (Form<SlackRequest>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
    if form.text.trim().is_empty() {
        return prepare_response(
            "Specify repository name to search. \
             For example: /code_review_bot linut"
                .to_string(),
        );
    }

    let repository = form.text.to_lowercase().to_string();
    let response_body = match state.github.search(&repository, 10) {
        Ok(result) => format!("{}", &result),
        Err(e) => format!("Something went wrong: {}", e),
    };

    prepare_response(response_body)
}

fn prepare_response(text: String) -> actix_web::Result<HttpResponse> {
    let body = serde_json::to_string(&SlackResponse {
        text: text,
        response_type: "in_channel".to_string(),
    }).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn application(github_url: String) -> App<AppConfig> {
    App::with_state(AppConfig::new(github_url)).resource("/code_review_bot", |r| {
        r.method(http::Method::POST).with(code_review_bot)
    })
}

pub fn start_server() {
    server::new(move || application("https://api.github.com".to_string()))
        .bind("0.0.0.0:8088")
        .unwrap()
        .run();
}
