use actix::prelude::{Actor, Handler, Message, SyncContext};
use actix_web::{error, Error};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::{NewPullRequest, PullRequest};

pub struct DBExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Message for NewPullRequest {
  type Result = Result<PullRequest, Error>;
}

impl Actor for DBExecutor {
  type Context = SyncContext<Self>;
}

impl Handler<NewPullRequest> for DBExecutor {
  type Result = Result<PullRequest, Error>;

  fn handle(&mut self, msg: NewPullRequest, _: &mut Self::Context) -> Self::Result {
    use crate::schema::pull_requests::dsl::*;

    let conn = &*self.0.get().map_err(error::ErrorInternalServerError)?;

    diesel::insert_into(pull_requests)
      .values(msg)
      .get_result::<PullRequest>(conn)
      .map_err(error::ErrorNotFound)
  }
}
