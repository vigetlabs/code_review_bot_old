use actix_web::{Error, HttpResponse};
use juniper_actix::playground_handler;

pub async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("graphql", None).await
}
