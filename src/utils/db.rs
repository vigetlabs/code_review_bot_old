use actix::prelude::{Actor, Handler, Message, SyncContext};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::error::Result;
pub use crate::models::{NewPullRequest, PullRequest};

pub struct DBExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub struct FindPullRequest {
    pub github_id: String,
}

pub struct UpdatePullRequestState {
    pub github_id: String,
    pub state: String,
}

pub struct PullRequestsByState {
    pub state: String,
}

impl Message for FindPullRequest {
    type Result = Result<PullRequest>;
}

impl Message for NewPullRequest {
    type Result = Result<PullRequest>;
}

impl Message for UpdatePullRequestState {
    type Result = Result<PullRequest>;
}

impl Message for PullRequestsByState {
    type Result = Result<Vec<PullRequest>>;
}

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<NewPullRequest> for DBExecutor {
    type Result = Result<PullRequest>;

    fn handle(&mut self, msg: NewPullRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pull_requests::dsl::*;

        let conn = &*self.0.get()?;

        diesel::insert_into(pull_requests)
            .values(msg)
            .get_result::<PullRequest>(conn)
            .map_err(|e| e.into())
    }
}

impl Handler<FindPullRequest> for DBExecutor {
    type Result = Result<PullRequest>;

    fn handle(&mut self, msg: FindPullRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pull_requests::dsl::*;

        let conn = &*self.0.get()?;

        pull_requests
            .filter(github_id.eq(&msg.github_id))
            .first(conn)
            .map_err(|e| e.into())
    }
}

impl Handler<UpdatePullRequestState> for DBExecutor {
    type Result = Result<PullRequest>;

    fn handle(&mut self, msg: UpdatePullRequestState, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pull_requests::dsl::*;

        let conn = &*self.0.get()?;

        diesel::update(pull_requests.filter(github_id.eq(&msg.github_id)))
            .set(state.eq(&msg.state))
            .get_result(conn)
            .map_err(|e| e.into())
    }
}

impl Handler<PullRequestsByState> for DBExecutor {
    type Result = Result<Vec<PullRequest>>;

    fn handle(&mut self, msg: PullRequestsByState, _: &mut Self::Context) -> Self::Result {
        use crate::schema::pull_requests::dsl::*;

        let conn = &*self.0.get()?;

        pull_requests
            .filter(state.eq(&msg.state))
            .load::<PullRequest>(conn)
            .map_err(|e| e.into())
    }
}
