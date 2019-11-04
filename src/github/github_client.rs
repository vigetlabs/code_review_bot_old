use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::error::Result;
use crate::models;
use crate::utils::paginated_resource::{PaginatedResource, PaginationParams};
use crate::utils::Languages;

use super::{
    add_user_token::AddUserToken, FileResult, NewWebhook, PRResult, Repo, ReviewRequest, User,
    Webhook,
};

#[derive(Clone)]
pub struct GithubClient {
    url: String,
    client: reqwest::Client,
}

impl GithubClient {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(url: String, token: &str) -> Result<GithubClient> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("bearer {}", token)).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(GithubClient { url, client })
    }

    pub fn get_pr(&self, pull_request: &ReviewRequest) -> Result<PRResult> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/pulls/{id}",
            url = self.url,
            owner = pull_request.owner,
            repo = pull_request.name,
            id = pull_request.id
        );

        self.get_json(&request_url, None)
    }

    pub fn get_files(&self, pull_request: &PRResult, lookup: &Languages) -> Result<String> {
        let request_url = format!("{}/files", pull_request.url);

        let res: Vec<FileResult> = self.get_json(&request_url, None)?;

        let mut file_extensions: Vec<String> = res
            .iter()
            .filter_map(|file_res| file_res.extension())
            .filter_map(|ext| lookup.get(&ext))
            .map(|icon| icon.to_string())
            .collect();

        file_extensions.sort();
        file_extensions.dedup();

        Ok(file_extensions.join(" "))
    }

    pub fn create_webhook(
        &self,
        pull_request: &ReviewRequest,
        webhook_url: &str,
        token: Option<String>,
    ) -> Result<Webhook> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/hooks",
            url = self.url,
            owner = pull_request.owner,
            repo = pull_request.name,
        );

        let hooks: Vec<Webhook> = self.get_json(&request_url, token.clone())?;

        hooks
            .iter()
            .find(|hook| hook.config.url.contains("github_event"))
            .cloned()
            .map_or_else(
                || {
                    let body = serde_json::to_string(&NewWebhook::new(webhook_url)).unwrap();
                    self.post_json(&request_url, &body, token)
                },
                Ok,
            )
    }

    pub fn delete_webhook(&self, hook: &models::Webhook, token: Option<String>) -> Result<()> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/hooks/{hook_id}",
            url = self.url,
            owner = hook.owner,
            repo = hook.name,
            hook_id = hook.hook_id,
        );

        self.delete(&request_url, token).map(|_| ())
    }

    pub fn get_user(&self, access_token: &str) -> Result<User> {
        let request_url = format!("{url}/user", url = self.url,);

        self.get_json(&request_url, Some(access_token.to_owned()))
    }

    pub fn get_repos(
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

        self.get(&request_url, Some(access_token.to_owned()))
            .and_then(|mut res| {
                let resources: Vec<Repo> = res.json()?;
                let link_header = res.headers().get(reqwest::header::LINK);

                if let Some(link_str) = link_header {
                    let link = hyperx::header::Link::from_str(link_str.to_str()?)?;
                    PaginatedResource::new(resources, link.values())
                } else {
                    PaginatedResource::new(resources, &[])
                }
            })
    }

    fn get_json<T>(&self, url: &str, token: Option<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get(url, token)?.json().map_err(|e| e.into())
    }

    fn get(&self, url: &str, token: Option<String>) -> Result<reqwest::Response> {
        self.client
            .get(url)
            .maybe_add_token(token)
            .send()?
            .error_for_status()
            .map_err(|e| e.into())
    }

    fn delete(&self, url: &str, token: Option<String>) -> Result<reqwest::Response> {
        self.client
            .delete(url)
            .maybe_add_token(token)
            .send()?
            .error_for_status()
            .map_err(|e| e.into())
    }

    fn post_json<T>(&self, url: &str, body: &str, token: Option<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .maybe_add_token(token)
            .body(body.to_owned())
            .send()?
            .error_for_status()?
            .json()
            .map_err(|e| e.into())
    }
}
