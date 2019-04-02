use actix_web;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure_derive;

mod models;
mod schema;

mod github;

mod slack;

mod utils;
pub use crate::utils::app_config::AppConfig;
pub use crate::utils::{db, load_languages, Languages};

mod error;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{http, pred, server, App};
use listenfd::ListenFd;

const LOG_FORMAT: &str =
    "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T \"%{X-GitHub-Event}i\"";

pub fn application(app_config: AppConfig) -> App<AppConfig> {
    App::with_state(app_config)
        .middleware(Logger::new(LOG_FORMAT))
        .resource("/review", |r| {
            r.method(http::Method::POST)
                .with(routes::slack_webhook::review)
        })
        .resource("/slack_event", |r| {
            r.method(http::Method::POST)
                .with(routes::slack_webhook::message)
        })
        .resource("/reviews", |r| {
            r.method(http::Method::POST)
                .with(routes::slack_webhook::reviews)
        })
        .resource("/github_event", |r| {
            r.method(http::Method::POST)
                .filter(pred::Header("X-GitHub-Event", "pull_request"))
                .with(routes::github_webhook::pull_request);
            r.method(http::Method::POST)
                .filter(pred::Header("X-GitHub-Event", "pull_request_review"))
                .with(routes::github_webhook::review);
        })
}

pub fn start_server(port: u32, app_config: AppConfig) -> Result<&'static str, std::io::Error> {
    server::new(move || application(app_config.clone()))
        .bind(format!("0.0.0.0:{}", port))?
        .run();

    Ok("Done")
}

pub fn start_dev_server(port: u32, app_config: AppConfig) -> Result<&'static str, std::io::Error> {
    let mut listenfd = ListenFd::from_env();
    let server = server::new(move || application(app_config.clone()));

    if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)
    } else {
        server.bind(format!("0.0.0.0:{}", port))?
    }
    .start();

    Ok("Done")
}
