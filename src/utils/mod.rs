extern crate serde_yaml;

pub mod app_config;
pub mod db;
pub mod helpers;
pub mod paginated_resource;

use actix_web::HttpResponse;

pub fn prepare_response(body: &str) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body.to_string())
}
