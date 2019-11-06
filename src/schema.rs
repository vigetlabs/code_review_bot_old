table! {
    configs (key) {
        key -> Varchar,
        value -> Varchar,
    }
}

table! {
    file_extensions (id) {
        id -> Int4,
        extension -> Varchar,
        icon_mapping_id -> Nullable<Int4>,
    }
}

table! {
    file_names (id) {
        id -> Int4,
        name -> Varchar,
        icon_mapping_id -> Nullable<Int4>,
    }
}

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
    icon_mappings (id) {
        id -> Int4,
        file_type -> Varchar,
        image_file -> Varchar,
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

table! {
    webhooks (id) {
        id -> Int4,
        hook_id -> Varchar,
        name -> Varchar,
        owner -> Varchar,
    }
}

joinable!(file_extensions -> icon_mappings (icon_mapping_id));
joinable!(file_names -> icon_mappings (icon_mapping_id));

allow_tables_to_appear_in_same_query!(
    configs,
    file_extensions,
    file_names,
    github_users,
    icon_mappings,
    pull_requests,
    reviews,
    users,
    webhooks,
);
