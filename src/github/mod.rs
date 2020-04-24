use std::fmt;
use std::path::Path;

mod add_user_token;
mod github_client;
mod github_oauth_client;
mod review_request;
pub use github_client::GithubClient;
pub use github_oauth_client::GithubOauthClient;
pub use review_request::ReviewRequest;

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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PRReviewState {
    ChangesRequested,
    Approved,
    Commented,
}

impl fmt::Display for PRReviewState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self)
                .map_err(|_| fmt::Error)?
                .replace("\"", "")
        )
    }
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
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Repo {
    pub id: i32,
    pub owner: User,
    pub name: String,
    pub full_name: String,
    #[serde(default)]
    pub permissions: RepoPermissions,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct RepoPermissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
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
pub struct FileResult {
    filename: String,
}

impl FileResult {
    pub fn extension(&self) -> Option<String> {
        Path::new(&self.filename)
            .extension()
            .and_then(|os_str| os_str.to_str())
            .map(|string| format!(".{}", string))
    }

    pub fn filename(&self) -> Option<String> {
        Path::new(&self.filename)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|string| string.to_owned())
    }
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
    pub fn image_path(&self) -> String {
        if let PRState::Open = self.state {
            "/public/images/open.png".to_string()
        } else if self.merged {
            "/public/images/merged.png".to_string()
        } else {
            "/public/images/closed.png".to_string()
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

#[derive(Debug, Serialize)]
struct NewWebhook {
    config: WebhookConfig,
    events: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Webhook {
    pub id: i32,
    pub config: WebhookConfig,
    pub events: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookConfig {
    url: String,
    content_type: ContentType,
    secret: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Json,
    Form,
}

impl NewWebhook {
    fn new(webhook_url: &str) -> Self {
        Self {
            events: vec![
                "pull_request".to_string(),
                "pull_request_review".to_string(),
            ],
            config: WebhookConfig {
                url: webhook_url.to_string(),
                content_type: ContentType::Json,
                secret: Some("update-only".to_string()),
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PRFiles {
    pub extensions: Vec<String>,
    pub filenames: Vec<String>,
}

impl PRFiles {
    pub async fn new(pull_request: &PRResult, client: &GithubClient, token: Option<String>) -> Self {
        if let Some(token) = token {
            let files = client.get_files(pull_request, &token).await;
            files
                .map(|files| {
                    files.into_iter()
                        .map(|file| (file.filename(), file.extension()))
                })
                .map(|file_info| file_info.unzip())
                .map(|(filenames, extensions): (Vec<_>, Vec<_>)| {
                    let filenames: Vec<String> = filenames.into_iter().filter_map(|o| o).collect();
                    let mut extensions: Vec<String> =
                        extensions.into_iter().filter_map(|o| o).collect();
                    extensions.dedup();

                    PRFiles {
                        extensions,
                        filenames,
                    }
                }).unwrap_or_default()
        } else  {
            Self::default()
        }
    }
}
