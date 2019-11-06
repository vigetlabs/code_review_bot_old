use actix_session::Session;

use crate::error::Result;
use crate::models::User;
use crate::utils::app_config::AppData;

pub fn get_current_user(state: &AppData, session: &Session) -> Result<Option<User>> {
    if let Some(id) = session.get("id")? {
        User::find(id, &state.db)
    } else {
        Ok(None)
    }
}
