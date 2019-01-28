use crate::schema::pull_requests;

#[derive(Debug, Insertable)]
#[table_name = "pull_requests"]
pub struct NewPullRequest<'a> {
  pub github_id: &'a str,
  pub state: &'a str,
  pub slack_message_id: &'a str,
}

#[derive(Debug, Queryable)]
pub struct PullRequest {
  pub id: i32,
  pub github_id: String,
  pub state: String,
  pub slack_message_id: String,
}
