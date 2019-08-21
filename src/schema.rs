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
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        slack_user_id -> Varchar,
        slack_access_token -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    pull_requests,
    users,
);
