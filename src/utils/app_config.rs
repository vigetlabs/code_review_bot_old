use crate::db;
use crate::error::Result;
use crate::github::GithubClient;
use crate::slack::SlackClient;
use crate::utils::Languages;

#[derive(Clone)]
pub struct AppConfig {
    pub github: GithubClient,
    pub slack: SlackClient,
    pub language_lookup: Languages,
    pub db: db::DBExecutor,
    pub webhook_url: String,
}

impl AppConfig {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        github_token: &str,
        slack_token: &str,
        channel: &str,
        language_lookup: Languages,
        db: db::DBExecutor,
        webhook_url: String,
    ) -> Result<Self> {
        let github_url = "https://api.github.com".to_string();
        let slack_url = "https://slack.com/api/".to_string();

        Ok(AppConfig {
            github: GithubClient::new(github_url, &github_token)?,
            slack: SlackClient::new(slack_url, &slack_token, channel.to_string())?,
            language_lookup,
            db,
            webhook_url,
        })
    }
}
