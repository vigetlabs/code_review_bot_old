use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use juniper::{
    graphql_object, Context, EmptyMutation, EmptySubscription, FieldResult, GraphQLInputObject,
    GraphQLObject, GraphQLUnion, RootNode,
};

use crate::db;

impl Context for db::DBExecutor {}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<db::DBExecutor>, EmptySubscription<db::DBExecutor>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<db::DBExecutor>::new(),
        EmptySubscription::<db::DBExecutor>::new(),
    )
}

#[derive(GraphQLObject)]
struct PageInfo {
    page: i32,
    per_page: i32,
    count: i32,
}

#[derive(GraphQLUnion)]
#[graphql(context=db::DBExecutor)]
enum PaginatedItem {
    PullRequests(PullRequestType),
    Reviews(ReviewType),
}

#[derive(GraphQLObject)]
#[graphql(context=db::DBExecutor)]
struct PaginatedItems {
    items: Vec<PaginatedItem>,
    page_info: PageInfo,
}

#[derive(GraphQLInputObject)]
struct PullRequestSearchInput {
    start_at: Option<NaiveDateTime>,
    end_at: Option<NaiveDateTime>,
}

#[derive(Queryable)]
struct UserType {
    id: i32,
    username: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[graphql_object(context=db::DBExecutor)]
impl UserType {
    fn id(&self) -> i32 {
        self.id
    }

    fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    #[graphql(arguments(page(default = 1), per_page(default = 25)))]
    fn pull_requests(
        &self,
        db: &db::DBExecutor,
        page: i32,
        per_page: i32,
        search: Option<PullRequestSearchInput>,
    ) -> FieldResult<PaginatedItems> {
        paginated_pull_requests(db, page, per_page, search, Some(self.id))
    }

    #[graphql(arguments(page(default = 1), per_page(default = 25)))]
    fn reviews(
        &self,
        db: &db::DBExecutor,
        page: i32,
        per_page: i32,
    ) -> FieldResult<PaginatedItems> {
        use crate::schema::github_users::dsl::{github_users, user_id};
        use crate::schema::reviews::dsl::*;
        let conn = db.0.get()?;

        let query = reviews.inner_join(github_users).filter(user_id.eq(self.id));
        let count: i64 = query.clone().count().first(&conn)?;

        let revs = query
            .select((id, state, user_id, created_at))
            .order(created_at.desc())
            .limit(per_page as i64)
            .offset((page as i64 - 1) * per_page as i64)
            .load(&conn)?;

        Ok(PaginatedItems {
            items: revs.into_iter().map(PaginatedItem::Reviews).collect(),
            page_info: PageInfo {
                page,
                per_page,
                count: count as i32,
            },
        })
    }
}

#[derive(Queryable)]
struct PullRequestType {
    id: i32,
    state: String,
    channel: String,
    user_id: Option<i32>,
    display_text: String,
}

#[graphql_object(context = db::DBExecutor)]
impl PullRequestType {
    fn state(&self) -> &str {
        self.state.as_str()
    }

    fn channel(&self) -> &str {
        self.channel.as_str()
    }

    fn display_text(&self) -> &str {
        self.display_text.as_str()
    }

    fn user(&self, db: &db::DBExecutor) -> FieldResult<Option<UserType>> {
        if let Some(current_user_id) = self.user_id {
            use crate::schema::github_users::{self, dsl::*};
            use crate::schema::users::dsl::{created_at, id, updated_at, users};
            let conn = db.0.get()?;

            return Ok(Some(
                users
                    .left_join(github_users::table.on(github_users::user_id.eq(id.nullable())))
                    .select((id, login.nullable(), created_at, updated_at))
                    .filter(id.eq(current_user_id))
                    .first(&conn)?,
            ));
        }
        Ok(None)
    }

    fn reviews(&self, db: &db::DBExecutor) -> FieldResult<Vec<ReviewType>> {
        use crate::schema::github_users::dsl::{github_users, user_id};
        use crate::schema::reviews::dsl::*;
        let conn = db.0.get()?;

        reviews
            .filter(pull_request_id.eq(self.id))
            .inner_join(github_users)
            .select((id, state, user_id, created_at))
            .load(&conn)
            .map_err(|e| e.into())
    }
}

#[derive(Queryable)]
struct ReviewType {
    id: i32,
    state: String,
    user_id: Option<i32>,
    created_at: NaiveDateTime,
}

#[graphql_object(context = db::DBExecutor)]
impl ReviewType {
    fn id(&self) -> i32 {
        self.id
    }

    fn state(&self) -> &str {
        self.state.as_str()
    }

    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    fn user(&self, db: &db::DBExecutor) -> FieldResult<Option<UserType>> {
        if let Some(current_user_id) = self.user_id {
            use crate::schema::github_users::{self, dsl::*};
            use crate::schema::users::dsl::{created_at, id, updated_at, users};
            let conn = db.0.get()?;

            return Ok(Some(
                users
                    .left_join(github_users::table.on(github_users::user_id.eq(id.nullable())))
                    .select((id, login.nullable(), created_at, updated_at))
                    .filter(id.eq(current_user_id))
                    .first(&conn)?,
            ));
        }
        Ok(None)
    }
}

pub struct Query;
#[graphql_object(context = db::DBExecutor)]
impl Query {
    fn api_version() -> String {
        "0.1".to_string()
    }

    #[graphql(arguments(uid(name = "id")))]
    fn user(db: &db::DBExecutor, uid: i32) -> FieldResult<UserType> {
        use crate::schema::github_users::dsl::*;
        use crate::schema::users::dsl::{created_at, id, updated_at, users};
        let conn = db.0.get()?;

        users
            .left_join(github_users)
            .select((id, login.nullable(), created_at, updated_at))
            .filter(id.eq(uid))
            .first(&conn)
            .map_err(|e| e.into())
    }

    fn users(db: &db::DBExecutor) -> FieldResult<Vec<UserType>> {
        use crate::schema::github_users::dsl::*;
        use crate::schema::users::dsl::{created_at, id, updated_at, users};
        let conn = db.0.get()?;

        users
            .left_join(github_users)
            .select((id, login.nullable(), created_at, updated_at))
            .load(&conn)
            .map_err(|e| e.into())
    }

    #[graphql(arguments(page(default = 1), per_page(default = 25)))]
    fn pull_requests(
        db: &db::DBExecutor,
        page: i32,
        per_page: i32,
        search: Option<PullRequestSearchInput>,
    ) -> FieldResult<PaginatedItems> {
        paginated_pull_requests(db, page, per_page, search, None)
    }
}

fn paginated_pull_requests(
    db: &db::DBExecutor,
    page: i32,
    per_page: i32,
    search: Option<PullRequestSearchInput>,
    user_id: Option<i32>,
) -> FieldResult<PaginatedItems> {
    use crate::schema::github_users::dsl::{github_users, user_id as gh_user_id};
    use crate::schema::pull_requests::dsl::*;
    let conn = db.0.get()?;

    let (start_at, end_at) = if let Some(PullRequestSearchInput { start_at, end_at }) = search {
        (
            start_at.unwrap_or_else(|| Utc::now().naive_utc() - Duration::days(7)),
            end_at.unwrap_or_else(|| Utc::now().naive_utc()),
        )
    } else {
        (
            Utc::now().naive_utc() - Duration::days(7),
            Utc::now().naive_utc(),
        )
    };

    let count: i64;
    let query = pull_requests
        .filter(created_at.between(start_at, end_at))
        .inner_join(github_users);

    let new_query = if let Some(user_id) = user_id {
        let q = query.filter(gh_user_id.eq(user_id));
        count = q.clone().count().first(&conn)?;
        q.into_boxed()
    } else {
        count = query.clone().count().first(&conn)?;
        query.into_boxed()
    };

    let prs = new_query
        .select((id, state, channel, gh_user_id, display_text))
        .order(created_at.desc())
        .limit(per_page as i64)
        .offset((page as i64 - 1) * per_page as i64)
        .load(&conn)?;

    Ok(PaginatedItems {
        items: prs.into_iter().map(PaginatedItem::PullRequests).collect(),
        page_info: PageInfo {
            page,
            per_page,
            count: count as i32,
        },
    })
}
