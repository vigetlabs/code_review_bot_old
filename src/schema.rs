table! {
    github_users (id) {
        id -> Int4,
        login -> Varchar,
        avatar_url -> Varchar,
        github_id -> Int4,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    pull_requests (id) {
        id -> Int4,
        github_id -> Varchar,
        state -> Varchar,
        slack_message_id -> Varchar,
        channel -> Varchar,
        display_text -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        github_user_id -> Int4,
    }
}

table! {
    reviews (id) {
        id -> Int4,
        pull_request_id -> Int4,
        github_user_id -> Int4,
        state -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        slack_user_id -> Varchar,
        slack_access_token -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        github_access_token -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    github_users,
    pull_requests,
    reviews,
    users,
);
