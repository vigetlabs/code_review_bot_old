use actix_web::{web::Data, Error, HttpResponse};
use juniper_actix::{graphql_handler, playground_handler};

use crate::db;
use crate::graphql::schema::Schema;

pub async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("graphql", None).await
}

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: Data<Schema>,
    db: Data<db::DBExecutor>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &db, req, payload).await
}
