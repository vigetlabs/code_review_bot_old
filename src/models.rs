use diesel::prelude::*;

use crate::db::Connection;
use crate::error::Result;
use crate::schema::{pull_requests, users};

#[derive(Debug, Insertable)]
#[table_name = "pull_requests"]
pub struct NewPullRequest {
    pub github_id: String,
    pub state: String,
    pub slack_message_id: String,
    pub channel: String,
    pub display_text: String,
}

#[derive(Clone, Debug, Queryable)]
pub struct PullRequest {
    pub id: i32,
    pub github_id: String,
    pub state: String,
    pub slack_message_id: String,
    pub channel: String,
    pub display_text: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub slack_user_id: String,
    pub slack_access_token: String,
}

#[derive(Clone, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub slack_user_id: String,
    pub slack_access_token: Option<String>,
}

impl User {
    pub fn create_or_udpate(new_user: &NewUser, conn: Connection) -> Result<User> {
        use crate::schema::users::dsl::*;
        let user_res: Result<User> = users
            .filter(slack_user_id.eq(&new_user.slack_user_id))
            .first(&conn)
            .map_err(|e| e.into());

        match user_res {
            Ok(user) => diesel::update(users.find(user.id))
                .set(slack_access_token.eq(&new_user.slack_access_token))
                .get_result(&conn),
            Err(_) => diesel::insert_into(users)
                .values(new_user)
                .get_result(&conn),
        }
        .map_err(|e| e.into())
    }

    pub fn find(find_id: i32, conn: Connection) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;

        match users.find(find_id).first(&conn) {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
