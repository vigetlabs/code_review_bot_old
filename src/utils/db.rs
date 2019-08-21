use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct DBExecutor(pub Pool<ConnectionManager<PgConnection>>);
pub type Connection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
