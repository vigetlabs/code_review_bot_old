use crate::error::{Result, UrlParseError};
use crate::utils::Languages;
use std::{fmt, path::Path, str::FromStr};
use url::{self, Url};

#[derive(Deserialize, Debug)]
pub struct PullRequestEvent {
    pub number: u32,
    pub action: PRAction,
    pub pull_request: PRResult,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PRAction {
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    Labeled,
    Unlabled,
    Opened,
    ReadyForReview,
    Reopened,
    Closed,
    Edited,
    Synchronize,
}

#[derive(Deserialize, Debug)]
pub struct ReviewEvent {
    pub action: ReviewAction,
    pub pull_request: ReviewPR,
    pub review: PRReview,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReviewAction {
    Submitted,
    Editied,
    Dismissed,
}

#[derive(Deserialize, Debug)]
pub struct PRReview {
    pub state: PRReviewState,
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PRReviewState {
    ChangesRequested,
    Approved,
    Commented,
}

#[derive(Deserialize, Debug)]
pub struct ReviewPR {
    pub url: String,
    pub html_url: String,
    pub title: String,
    pub body: String,
    pub state: PRState,
    pub number: u32,
    #[serde(default)]
    pub draft: bool,

    pub user: User,
    pub base: Base,
}

#[derive(Clone, Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Repo {
    pub full_name: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Base {
    pub repo: Repo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PRResult {
    pub url: String,
    pub html_url: String,
    pub title: String,
    pub body: String,
    pub state: PRState,
    pub merged: bool,
    pub review_comments: u32,
    pub additions: u32,
    pub deletions: u32,
    pub number: u32,
    #[serde(default)]
    pub draft: bool,

    pub user: User,
    pub base: Base,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PRState {
    Open,
    Closed,
}

#[derive(Deserialize)]
struct FileResult {
    filename: String,
}

impl fmt::Display for PRResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = format!("{}: {}", self.base.repo.full_name, self.title);
        write!(
            f,
            "(+{additions} -{deletions}) <{url}|{title}> by {user}",
            additions = self.additions,
            deletions = self.deletions,
            url = self.html_url,
            title = title,
            user = self.user.login
        )
    }
}

impl PRResult {
    pub fn color(&self) -> String {
        if let PRState::Open = self.state {
            "#34d058".to_string()
        } else if self.merged {
            "#6f42c1".to_string()
        } else {
            "#cb2431".to_string()
        }
    }

    pub fn open(&self) -> bool {
        if let PRState::Open = self.state {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PullRequest {
    pub owner: String,
    pub name: String,
    pub id: String,
}

impl FromStr for PullRequest {
    type Err = UrlParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let repository_url = Url::parse(s).map_err(UrlParseError::Parse)?;
        let mut path = repository_url
            .path_segments()
            .ok_or(UrlParseError::MissingSegment)?;

        Ok(Self {
            owner: path
                .nth(0)
                .ok_or(UrlParseError::MissingSegment)?
                .to_string(),
            name: path
                .nth(0)
                .ok_or(UrlParseError::MissingSegment)?
                .to_string(),
            id: path
                .nth(1)
                .ok_or(UrlParseError::MissingSegment)?
                .to_string(),
        })
    }
}

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

    pub fn get_pr(&self, pull_request: &PullRequest) -> Result<PRResult> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/pulls/{id}",
            url = self.url,
            owner = pull_request.owner,
            repo = pull_request.name,
            id = pull_request.id
        );

        self.client
            .get(&request_url)
            .send()?
            .error_for_status()?
            .json()
            .map_err(|e| e.into())
    }

    pub fn get_files(&self, pull_request: &PRResult, lookup: &Languages) -> Result<String> {
        let request_url = format!("{}/files", pull_request.url);

        let res: Vec<FileResult> = self
            .client
            .get(&request_url)
            .send()?
            .error_for_status()?
            .json()?;

        let mut file_extensions: Vec<String> = res
            .iter()
            .filter_map(|file_res| Path::new(&file_res.filename).extension())
            .filter_map(|os_str| os_str.to_str())
            .map(|string| format!(".{}", string).to_string())
            .filter_map(|ext| lookup.get(&ext))
            .map(|icon| icon.to_string())
            .collect();

        file_extensions.sort();
        file_extensions.dedup();

        Ok(file_extensions.join(" "))
    }

    pub fn create_webhook(&self, pull_request: &PullRequest, webhook_url: &str) -> Result<()> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/hooks",
            url = self.url,
            owner = pull_request.owner,
            repo = pull_request.name,
        );

        let hooks: Vec<WebHook> = self
            .client
            .get(&request_url)
            .send()?
            .error_for_status()?
            .json()?;

        if !hooks
            .iter()
            .any(|hook| hook.config.url.contains("github_event"))
        {
            let body = serde_json::to_string(&WebHook::new(webhook_url)).unwrap();
            self.client
                .post(&request_url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(body)
                .send()?
                .error_for_status()?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct WebHook {
    config: WebHookConfig,
    events: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebHookConfig {
    url: String,
    content_type: ContentType,
    secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContentType {
    Json,
    Form,
}
impl WebHook {
    fn new(webhook_url: &str) -> Self {
        Self {
            events: vec![
                "pull_request".to_string(),
                "pull_request_review".to_string(),
            ],
            config: WebHookConfig {
                url: webhook_url.to_string(),
                content_type: ContentType::Json,
                secret: Some("update-only".to_string()),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_url_sucess() {
        let repo: PullRequest = "http://github.com/facebook/react/pulls/1234"
            .parse()
            .expect("Can't parse url");
        assert_eq!(repo.id, "1234");
        assert_eq!(repo.owner, "facebook");
        assert_eq!(repo.name, "react");
    }

    #[test]
    fn test_parse_url_failure() {
        let repo = "totally invalid url".parse::<PullRequest>();
        assert!(repo.is_err(), "Should not parse")
    }
}
