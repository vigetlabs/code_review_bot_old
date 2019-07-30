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
use actix_web::{guard, web, App, HttpServer};
use listenfd::ListenFd;

const LOG_FORMAT: &str =
    "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T \"%{X-GitHub-Event}i\"";

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/review", web::post().to(routes::slack_webhook::review))
        .route(
            "/slack_event",
            web::post().to_async(routes::slack_webhook::message),
        )
        .route(
            "/reviews",
            web::post().to_async(routes::slack_webhook::reviews),
        )
        .service(
            web::scope("/github_event")
                .route(
                    "/",
                    web::route()
                        .guard(guard::Header("X-GitHub-Event", "pull_request"))
                        .to_async(routes::github_webhook::pull_request),
                )
                .route(
                    "/",
                    web::route()
                        .guard(guard::Header("X-GitHub-Event", "pull_request_review"))
                        .to_async(routes::github_webhook::review),
                ),
        );
}

pub fn start_server(port: u32, app_config: AppConfig) -> Result<&'static str, std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(LOG_FORMAT))
            .data(app_config.clone())
            .configure(configure_app)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()?;

    Ok("Done")
}

pub fn start_dev_server(port: u32, app_config: AppConfig) -> Result<&'static str, std::io::Error> {
    let mut listenfd = ListenFd::from_env();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(LOG_FORMAT))
            .data(app_config.clone())
            .configure(configure_app)
    });

    if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(format!("0.0.0.0:{}", port))?
    }
    .run()?;

    Ok("Done")
}
