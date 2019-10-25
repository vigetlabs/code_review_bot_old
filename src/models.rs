use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::db::DBExecutor;
use crate::error::Result;
use crate::github;
use crate::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "pull_requests"]
pub struct NewPullRequest {
    pub github_id: String,
    pub state: String,
    pub slack_message_id: String,
    pub channel: String,
    pub display_text: String,
    pub github_user_id: i32,
}

#[derive(Clone, Debug, Queryable, Identifiable, Associations)]
#[table_name = "pull_requests"]
#[belongs_to(GithubUser)]
pub struct PullRequest {
    pub id: i32,
    pub github_id: String,
    pub state: String,
    pub slack_message_id: String,
    pub channel: String,
    pub display_text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub github_user_id: i32,
}

impl PullRequest {
    pub fn create(pr: &NewPullRequest, db: &DBExecutor) -> Result<PullRequest> {
        use crate::schema::pull_requests::dsl::*;
        let conn = db.0.get()?;

        diesel::insert_into(pull_requests)
            .values(pr)
            .get_result(&conn)
            .map_err(|e| e.into())
    }

    pub fn find(gh_id: &str, db: &DBExecutor) -> Result<PullRequest> {
        use crate::schema::pull_requests::dsl::*;
        let conn = db.0.get()?;

        pull_requests
            .filter(github_id.eq(gh_id))
            .first(&conn)
            .map_err(|e| e.into())
    }

    pub fn by_state(query_state: &str, db: &DBExecutor) -> Result<Vec<PullRequest>> {
        use crate::schema::pull_requests::dsl::*;
        let conn = db.0.get()?;

        pull_requests
            .filter(state.eq(query_state))
            .load::<PullRequest>(&conn)
            .map_err(|e| e.into())
    }

    pub fn update(&self, new_state: &str, db: &DBExecutor) -> Result<PullRequest> {
        use crate::schema::pull_requests::dsl::*;
        let conn = db.0.get()?;

        diesel::update(pull_requests.find(self.id))
            .set(state.eq(new_state))
            .get_result(&conn)
            .map_err(|e| e.into())
    }

    pub fn user(&self, db: &DBExecutor) -> Result<Option<User>> {
        use crate::schema::github_users::dsl::*;
        let conn = db.0.get()?;

        let gh_user = github_users
            .filter(github_id.eq(self.github_user_id))
            .first::<GithubUser>(&conn)?;

        gh_user.user(db)
    }
}

#[derive(Clone, Debug, Queryable, Identifiable, Associations)]
#[table_name = "github_users"]
#[belongs_to(User)]
pub struct GithubUser {
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
    pub github_id: i32,
    pub user_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl GithubUser {
    pub fn find_or_create(
        user: &github::User,
        db: &DBExecutor,
        u_id: Option<i32>,
    ) -> Result<GithubUser> {
        use crate::schema::github_users::dsl::*;
        let conn = db.0.get()?;
        let res = github_users
            .filter(github_id.eq(user.id))
            .first::<GithubUser>(&conn);

        let gh_user: Option<GithubUser> = match res {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(err) => Err(err),
        }?;

        match gh_user {
            Some(user) => Ok(user),
            None => match u_id {
                Some(u_id) => diesel::insert_into(github_users)
                    .values(vec![(
                        login.eq(&user.login),
                        avatar_url.eq(&user.avatar_url),
                        github_id.eq(user.id),
                        user_id.eq(u_id),
                    )])
                    .get_result(&conn)
                    .map_err(|e| e.into()),
                None => diesel::insert_into(github_users)
                    .values(vec![(
                        login.eq(&user.login),
                        avatar_url.eq(&user.avatar_url),
                        github_id.eq(user.id),
                    )])
                    .get_result(&conn)
                    .map_err(|e| e.into()),
            },
        }
    }

    pub fn user(&self, db: &DBExecutor) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;
        let conn = db.0.get()?;

        if let Some(u_id) = self.user_id {
            return users
                .find(u_id)
                .get_result::<User>(&conn)
                .map(Some)
                .map_err(|e| e.into());
        }
        Ok(None)
    }
}

#[derive(Clone, Debug, Queryable, Identifiable, Associations)]
#[belongs_to(GithubUser)]
#[belongs_to(PullRequest)]
pub struct Review {
    pub id: i32,
    pub github_user_id: i32,
    pub pull_request_id: i32,
    pub state: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Review {
    pub fn create_or_update(
        reviewer: &GithubUser,
        pull_request: &PullRequest,
        review_state: &str,
        db: &DBExecutor,
    ) -> Result<Review> {
        use crate::schema::reviews::dsl::*;
        let conn = db.0.get()?;

        let review: Result<Review> = reviews
            .filter(github_user_id.eq(&reviewer.id))
            .filter(pull_request_id.eq(&pull_request.id))
            .first(&conn)
            .map_err(|e| e.into());

        match review {
            Ok(review) => diesel::update(reviews.find(review.id))
                .set(state.eq(review_state))
                .get_result(&conn),
            Err(_) => diesel::insert_into(reviews)
                .values(vec![(
                    github_user_id.eq(reviewer.id),
                    pull_request_id.eq(pull_request.id),
                    state.eq(review_state),
                )])
                .get_result(&conn),
        }
        .map_err(|e| e.into())
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub slack_user_id: String,
    pub slack_access_token: String,
}

#[derive(Clone, Debug, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub slack_user_id: String,
    pub slack_access_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub github_access_token: Option<String>,
}

impl User {
    pub fn create_or_udpate(new_user: &NewUser, db: &DBExecutor) -> Result<User> {
        use crate::schema::users::dsl::*;
        let conn = db.0.get()?;

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

    pub fn find(find_id: i32, db: &DBExecutor) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;
        let conn = db.0.get()?;

        match users.find(find_id).first(&conn) {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
    pub fn connect_to_github_user(
        &self,
        access_token: &str,
        github_user: &github::User,
        db: &DBExecutor,
    ) -> Result<User> {
        use crate::schema::users::dsl::*;
        let conn = db.0.get()?;
        GithubUser::find_or_create(github_user, db, Some(self.id))?;
        diesel::update(users.find(self.id))
            .set(github_access_token.eq(access_token))
            .get_result(&conn)
            .map_err(|e| e.into())
    }

    pub fn is_gh_authed(&self) -> bool {
        self.github_access_token.is_some()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "webhooks"]
pub struct NewWebhook {
    pub hook_id: String,
    pub name: String,
    pub owner: String,
}

#[derive(Clone, Debug, Queryable, QueryableByName, Identifiable)]
#[table_name = "webhooks"]
pub struct Webhook {
    pub id: i32,
    pub hook_id: String,
    pub name: String,
    pub owner: String,
}

impl Webhook {
    pub fn create(new_webhook: &NewWebhook, db: &DBExecutor) -> Result<Webhook> {
        use crate::schema::webhooks::dsl::*;
        let conn = db.0.get()?;

        diesel::insert_into(webhooks)
            .values(new_webhook)
            .get_result(&conn)
            .map_err(|e| e.into())
    }

    pub fn for_repos(repos: &[github::Repo], db: &DBExecutor) -> Result<Vec<Webhook>> {
        let conn = db.0.get()?;

        let in_query = repos.to_query();

        diesel::sql_query(format!(
            "SELECT * FROM webhooks WHERE (owner, name) IN ({})",
            in_query,
        ))
        .load(&conn)
        .map_err(|e| e.into())
    }
}

trait ToQuery {
    fn to_query(&self) -> String;
}

impl<T> ToQuery for &[T]
where
    T: ToQuery,
{
    fn to_query(&self) -> String {
        self.iter()
            .map(|item| item.to_query())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl ToQuery for github::Repo {
    fn to_query(&self) -> String {
        format!("('{}','{}')", self.owner.login, self.name)
    }
}
