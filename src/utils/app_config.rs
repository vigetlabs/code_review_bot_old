use crate::db;
use crate::error::Result;
use crate::github::{GithubClient, GithubOauthClient};
use crate::slack::SlackClient;

#[derive(Clone)]
pub struct AppConfig {
    pub github: GithubClient,
    pub github_oauth: GithubOauthClient,
    pub slack: SlackClient,
    pub db: db::DBExecutor,
    pub webhook_url: String,
    pub app_url: String,
    pub app_secret: String,
}

impl AppConfig {
    // TODO: Builder pattern
    #[allow(clippy::new_ret_no_self, clippy::too_many_arguments)]
    pub fn new(
        github_token: &str,
        github_client_id: &str,
        github_client_secret: &str,
        slack_token: &str,
        channel: &str,
        client_id: &str,
        cient_secret: &str,
        db: db::DBExecutor,
        webhook_url: String,
        app_url: String,
        app_secret: String,
    ) -> Result<Self> {
        let github_url = "https://api.github.com".to_string();
        let slack_url = "https://slack.com/api/".to_string();

        Ok(AppConfig {
            github: GithubClient::new(github_url, &github_token)?,
            github_oauth: GithubOauthClient::new(&github_client_id, &github_client_secret),
            slack: SlackClient::new(
                slack_url,
                &slack_token,
                channel.to_string(),
                client_id.to_string(),
                cient_secret.to_string(),
            )?,
            db,
            webhook_url,
            app_url,
            app_secret,
        })
    }
}
