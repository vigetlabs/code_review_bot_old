#![warn(clippy::all)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure_derive;

mod error;
mod github;
mod middlewares;
mod models;
mod routes;
mod schema;
mod slack;
mod utils;

pub use crate::models::Config;
pub use crate::utils::{
    app_config::{AppConfig, AppData},
    db,
};

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::{self, guard, middleware::Logger, web, App, HttpServer};
use actix_web_flash::FlashMiddleware;
use listenfd::ListenFd;

const LOG_FORMAT: &str =
    "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T \"%{X-GitHub-Event}i\"";

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(routes::web::root))
        .route("/logout", web::get().to(routes::web::logout))
        .service(
            web::scope("/github")
                .route("/repos", web::get().to(routes::github::repos))
                .route("/webhooks", web::post().to(routes::github::create_webhook))
                .route(
                    "/webhooks/{id}",
                    web::post().to(routes::github::delete_webhook),
                ),
        )
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
        .route("/review", web::post().to(routes::slack_webhook::review))
        .route("/reviews", web::post().to(routes::slack_webhook::reviews))
        .route(
            "/slack_event",
            web::post().to(routes::slack_webhook::message),
        )
        .service(
            web::resource("/setup")
                .route(web::get().to(routes::web::new_setup))
                .route(web::post().to(routes::web::create_setup)),
        );
}

pub async fn start_server(
    port: u32,
    app_config: AppConfig,
    app_secret: String,
    db: db::DBExecutor,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(LOG_FORMAT))
            .wrap(CookieSession::signed(app_secret.as_bytes()).secure(false))
            .wrap(FlashMiddleware::default())
            .wrap(middlewares::SetupRedirect)
            .service(fs::Files::new("/public", "./public"))
            .data(db.clone())
            .app_data(web::Data::new(app_config.clone()))
            .configure(configure_app)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

pub async fn start_dev_server(
    port: u32,
    app_config: AppConfig,
    app_secret: String,
    db: db::DBExecutor,
) -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(app_secret.as_bytes()).secure(false))
            .wrap(Logger::new(LOG_FORMAT))
            .wrap(FlashMiddleware::default())
            .wrap(middlewares::SetupRedirect)
            .service(fs::Files::new("/public", "./public"))
            .data(app_config.clone())
            .data(db.clone())
            .configure(configure_app)
    });

    if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(format!("0.0.0.0:{}", port))?
    }
    .run()
    .await
}
