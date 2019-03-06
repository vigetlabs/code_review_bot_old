use crate::schema::pull_requests;

#[derive(Debug, Insertable)]
#[table_name = "pull_requests"]
pub struct NewPullRequest {
    pub github_id: String,
    pub state: String,
    pub slack_message_id: String,
    pub channel: String,
    pub display_text: String,
}

#[derive(Debug, Queryable)]
pub struct PullRequest {
    pub id: i32,
    pub github_id: String,
    pub state: String,
    pub slack_message_id: String,
    pub channel: String,
    pub display_text: String,
}
