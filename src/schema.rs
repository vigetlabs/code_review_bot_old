table! {
    pull_requests (id) {
        id -> Int4,
        github_id -> Varchar,
        state -> Varchar,
        slack_message_id -> Varchar,
    }
}
