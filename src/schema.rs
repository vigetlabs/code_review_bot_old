table! {
    pull_requests (id) {
        id -> Int4,
        github_id -> Varchar,
        state -> Varchar,
        slack_message_id -> Varchar,
        channel -> Varchar,
        display_text -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        slack_user_id -> Varchar,
        slack_access_token -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    pull_requests,
    users,
);
