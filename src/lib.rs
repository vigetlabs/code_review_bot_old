use actix_web;

#[macro_use]
extern crate serde_derive;

mod github;
use crate::github::ParseError;

mod slack;

mod utils;
pub use crate::utils::app_config::AppConfig;
pub use crate::utils::{load_languages, Languages};

mod routes;

use actix_web::middleware::Logger;
use actix_web::{http, server, App, ResponseError};
use listenfd::ListenFd;

impl ResponseError for ParseError {}

pub fn application(app_config: AppConfig) -> App<AppConfig> {
    App::with_state(app_config)
        .middleware(Logger::default())
        .resource("/review", |r| {
            r.method(http::Method::POST)
                .with(routes::slack_webhook::route)
        })
        .resource("/github_event", |r| {
            r.method(http::Method::POST)
                .with(routes::github_webhook::route)
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
    .run();

    Ok("Done")
}
