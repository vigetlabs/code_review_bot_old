use actix_session::Session;
use actix_web::web::Data;

use crate::error::Result;
use crate::models::User;
use crate::utils::app_config::AppConfig;

pub fn get_current_user(state: &Data<AppConfig>, session: &Session) -> Result<Option<User>> {
    if let Some(id) = session.get("id")? {
        User::find(id, &state.db)
    } else {
        Ok(None)
    }
}
