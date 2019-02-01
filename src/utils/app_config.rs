use crate::db;
use crate::github::GithubClient;
use crate::slack::SlackClient;
use crate::utils::Languages;

use actix::Addr;

#[derive(Clone)]
pub struct AppConfig {
  pub github: GithubClient,
  pub slack: SlackClient,
  pub language_lookup: Languages,
  pub db: Addr<db::DBExecutor>,
}

impl AppConfig {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(
    github_token: &str,
    slack_token: &str,
    language_lookup: Languages,
    db: Addr<db::DBExecutor>,
  ) -> Result<Self, &'static str> {
    let github_url = "https://api.github.com".to_string();
    let slack_url = "https://slack.com/api/".to_string();

    Ok(AppConfig {
      github: GithubClient::new(github_url, &github_token)?,
      slack: SlackClient::new(slack_url, &slack_token)?,
      language_lookup,
      db,
    })
  }
}
