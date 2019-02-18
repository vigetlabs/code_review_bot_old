use url;

use self::url::Url;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

use crate::utils::Languages;

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
    ReviewRequsted,
    ReviewRequestRemoved,
    Labeled,
    Unlabled,
    Opened,
    Reopened,
    Closed,
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
}

#[derive(Debug)]
pub struct PullRequest {
    pub owner: String,
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub enum ParseError {
    MissingSegment,
    Parse(url::ParseError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "malformed repo url")
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        "malformed repo url"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl FromStr for PullRequest {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let repository_url = Url::parse(s).map_err(ParseError::Parse)?;
        let mut path = repository_url
            .path_segments()
            .ok_or(ParseError::MissingSegment)?;

        Ok(Self {
            owner: path.nth(0).ok_or(ParseError::MissingSegment)?.to_string(),
            name: path.nth(0).ok_or(ParseError::MissingSegment)?.to_string(),
            id: path.nth(1).ok_or(ParseError::MissingSegment)?.to_string(),
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
    pub fn new(url: String, token: &str) -> Result<GithubClient, &'static str> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("bearer {}", token))
                .map_err(|_| "Invalid header value")?,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| "Cannot build client")?;

        Ok(GithubClient { url, client })
    }

    pub fn get_pr(&self, pull_request: &PullRequest) -> reqwest::Result<PRResult> {
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
    }

    pub fn get_files(
        &self,
        pull_request: &PRResult,
        lookup: &Languages,
    ) -> reqwest::Result<String> {
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

    pub fn create_webhook(&self, pull_request: &PullRequest) -> Result<(), String> {
        let request_url = format!(
            "{url}/repos/{owner}/{repo}/hooks",
            url = self.url,
            owner = pull_request.owner,
            repo = pull_request.name,
        );

        let hooks: Vec<WebHook> = self
            .client
            .get(&request_url)
            .send()
            .map_err(|e| format!("{}", e))?
            .error_for_status()
            .map_err(|e| format!("{}", e))?
            .json()
            .map_err(|e| format!("{}", e))?;

        match hooks
            .iter()
            .find(|hook| hook.config.url.contains("github_event"))
        {
            Some(_) => None,
            None => Some(()),
        }
        .ok_or_else(|| "Hook exists".to_string())
        .and_then(|_| {
            // TODO: Fix error handling here
            let body = serde_json::to_string(&WebHook::new()).unwrap();
            self.client
                .post(&request_url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(body)
                .send()
                .map_err(|e| format!("{}", e))?
                .error_for_status()
                .map_err(|e| format!("{}", e))
        })?;

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
    fn new() -> Self {
        Self {
            events: vec![
                "pull_request".to_string(),
                "pull_request_review".to_string(),
            ],
            config: WebHookConfig {
                url: "http://f95ae61b.ngrok.io/github_event".to_string(),
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
