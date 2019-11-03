#![warn(clippy::all)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure_derive;

mod error;
mod github;
mod models;
mod routes;
mod schema;
mod slack;
mod utils;

pub use crate::utils::{app_config::AppConfig, db};

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::{self, guard, middleware::Logger, web, App, HttpServer};
use actix_web_flash::FlashMiddleware;
use listenfd::ListenFd;

const LOG_FORMAT: &str =
    "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T \"%{X-GitHub-Event}i\"";

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(routes::web::root))
        .service(
            web::scope("/auth")
                .route("/slack", web::get().to(routes::auth::slack))
                .route("/github", web::get().to(routes::auth::github)),
        )
        .service(
            web::resource("/github_event")
                .route(
                    web::post()
                        .guard(guard::Header("X-GitHub-Event", "pull_request"))
                        .guard(guard::fn_guard(|req| {
                            !req.headers().contains_key("X-Hub-Signature")
                        }))
                        .to(routes::github_webhook::pull_request),
                )
                .route(
                    web::post()
                        .guard(guard::Header("X-GitHub-Event", "pull_request_review"))
                        .to(routes::github_webhook::review),
                ),
        )
        .service(web::scope("/icons").route("", web::get().to(routes::web::show_icons)))
        .route("/review", web::post().to(routes::slack_webhook::review))
        .route("/reviews", web::post().to(routes::slack_webhook::reviews))
        .route(
            "/slack_event",
            web::post().to(routes::slack_webhook::message),
        )
        .service(
            web::scope("/webhooks")
                .route("", web::post().to(routes::web::create_webhook))
                .route("/{id}", web::post().to(routes::web::delete_webhook)),
        );
}

pub fn start_server(port: u32, app_config: AppConfig) -> Result<&'static str, std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(LOG_FORMAT))
            .wrap(CookieSession::signed(app_config.app_secret.as_bytes()).secure(false))
            .wrap(FlashMiddleware::default())
            .service(fs::Files::new("/public", "./public"))
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
            .wrap(CookieSession::signed(app_config.app_secret.as_bytes()).secure(false))
            .wrap(Logger::new(LOG_FORMAT))
            .wrap(FlashMiddleware::default())
            .service(fs::Files::new("/public", "./public"))
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
