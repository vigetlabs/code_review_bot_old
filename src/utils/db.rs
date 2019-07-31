use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::error::{DatabaseError, Result};
pub use crate::models::{NewPullRequest, PullRequest};

#[derive(Clone)]
pub struct DBExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub type Connection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;

pub enum Queries {
    FindPullRequest { github_id: String },
    UpdatePullRequestState { github_id: String, state: String },
    PullRequestsByState { state: String },
    CreatePullRequest(NewPullRequest),
}

pub fn execute(
    executor: &DBExecutor,
    query: Queries,
) -> Result<Vec<PullRequest>> {
    let pool = executor.0.clone();
    match query {
        Queries::FindPullRequest { github_id } => find_pull_request(pool.get()?, github_id),
        Queries::UpdatePullRequestState { github_id, state } => {
            update_pull_request_state(pool.get()?, github_id, state)
        }
        Queries::PullRequestsByState { state } => pull_requests_by_state(pool.get()?, state),
        Queries::CreatePullRequest(pull_request) => create_pull_request(pool.get()?, pull_request),
    }.map_err(|e| e.into())
}

fn create_pull_request(
    conn: Connection,
    pr: NewPullRequest,
) -> std::result::Result<Vec<PullRequest>, DatabaseError> {
    use crate::schema::pull_requests::dsl::*;

    diesel::insert_into(pull_requests)
        .values(pr)
        .get_result(&conn)
        .map(|pr| vec![pr])
        .map_err(|e| e.into())
}

fn find_pull_request(
    conn: Connection,
    gh_id: String,
) -> std::result::Result<Vec<PullRequest>, DatabaseError> {
    use crate::schema::pull_requests::dsl::*;

    pull_requests
        .filter(github_id.eq(gh_id))
        .first(&conn)
        .map(|pr| vec![pr])
        .map_err(|e| e.into())
}

fn update_pull_request_state(
    conn: Connection,
    gh_id: String,
    new_state: String,
) -> std::result::Result<Vec<PullRequest>, DatabaseError> {
    use crate::schema::pull_requests::dsl::*;

    diesel::update(pull_requests.filter(github_id.eq(gh_id)))
        .set(state.eq(new_state))
        .get_result(&conn)
        .map(|pr| vec![pr])
        .map_err(|e| e.into())
}

fn pull_requests_by_state(
    conn: Connection,
    query_state: String,
) -> std::result::Result<Vec<PullRequest>, DatabaseError> {
    use crate::schema::pull_requests::dsl::*;

    pull_requests
        .filter(state.eq(query_state))
        .load::<PullRequest>(&conn)
        .map_err(|e| e.into())
}
