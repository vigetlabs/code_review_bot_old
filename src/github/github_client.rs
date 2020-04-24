use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::error::{Result, Error};
use crate::models;
use crate::utils::paginated_resource::{PaginatedResource, PaginationParams};

use super::{
    add_user_token::AddUserToken, FileResult, NewWebhook, PRResult, Repo, ReviewRequest, User,
    Webhook,
};

#[derive(Clone)]
pub struct GithubClient {
    url: String,
    client: reqwest::Client,
}

impl Default for GithubClient {
    fn default() -> Self {
        let client = reqwest::Client::new();

        Self {
            url: "https://api.github.com".to_owned(),
            client,
        }
    }
}

impl GithubClient {
    pub async fn get_pr(&self, pull_request: &ReviewRequest, token: &str) -> Result<PRResult> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/pulls/{id}",
            url = self.url,
            owner = pull_request.owner,
            repo = pull_request.name,
            id = pull_request.id
        );

        self.get_json(&request_url, token).await
    }

    pub async fn get_files(&self, pull_request: &PRResult, token: &str) -> Result<Vec<FileResult>> {
        let request_url = format!("{}/files", pull_request.url);

        self.get_json(&request_url, token).await
    }

    pub async fn create_webhook(
        &self,
        pull_request: &ReviewRequest,
        webhook_url: &str,
        token: &str,
    ) -> Result<Webhook> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/hooks",
            url = self.url,
            owner = pull_request.owner,
            repo = pull_request.name,
        );

        let hooks: Vec<Webhook> = self.get_json(&request_url, token).await?;

        if let Some(hook) = hooks
            .iter()
            .find(|hook| hook.config.url.contains("github_event"))
            .cloned() {
            Ok(hook)
        } else {
            let body = serde_json::to_string(&NewWebhook::new(webhook_url)).unwrap();
            self.post_json(&request_url, &body, token).await
        }
    }

    pub async fn delete_webhook(&self, hook: &models::Webhook, token: &str) -> Result<()> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/hooks/{hook_id}",
            url = self.url,
            owner = hook.owner,
            repo = hook.name,
            hook_id = hook.hook_id,
        );

        self.delete(&request_url, token).await.map(|_| ())
    }

    pub async fn get_user(&self, access_token: &str) -> Result<User> {
        let request_url = format!("{url}/user", url = self.url,);

        self.get_json(&request_url, access_token).await
    }

    pub async fn get_repos(
        &self,
        access_token: &str,
        params: &PaginationParams,
    ) -> Result<PaginatedResource<Repo>> {
        let request_url = format!(
            "{url}/user/repos?sort={sort}&page={page}",
            url = self.url,
            sort = "updated",
            page = params.page.as_ref().unwrap_or(&"1".to_owned()),
        );

        let res = self.get(&request_url, access_token).await?;

        let link_header = res.headers().get(reqwest::header::LINK).cloned();
        let resources: Vec<Repo> = res.json().await?;

        if let Some(link_str) = link_header {
            let link = hyperx::header::Link::from_str(
                link_str.to_str().map_err(|_| Error::ServerError("header error".to_string()))?
            )?;
            PaginatedResource::new(resources, link.values())
        } else {
            PaginatedResource::new(resources, &[])
        }
    }

    async fn get_json<T>(&self, url: &str, token: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(url, token).await?.json().await.map_err(|e| e.into())
    }

    async fn get(&self, url: &str, token: &str) -> Result<reqwest::Response> {
        self.client
            .get(url)
            .add_token(token)
            .send().await?
            .error_for_status()
            .map_err(|e| e.into())
    }

    async fn delete(&self, url: &str, token: &str) -> Result<reqwest::Response> {
        self.client
            .delete(url)
            .add_token(token)
            .send().await?
            .error_for_status()
            .map_err(|e| e.into())
    }

    async fn post_json<T>(&self, url: &str, body: &str, token: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .add_token(token)
            .body(body.to_owned())
            .send().await?
            .error_for_status()?
            .json().await
            .map_err(|e| e.into())
    }
}
