use actix_session::Session;

use crate::db::DBExecutor;
use crate::error::Result;
use crate::models::User;

pub fn get_current_user(db: &DBExecutor, session: &Session) -> Result<Option<User>> {
    if let Some(id) = session.get("id")? {
        User::find(id, db)
    } else {
        Ok(None)
    }
}
