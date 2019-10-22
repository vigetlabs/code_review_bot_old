pub mod attachment;
mod helpers;

use base64::encode;
use reqwest;
use std::fmt;

use crate::error::{Error, Result};
use crate::github;
use crate::models;

pub use helpers::extract_links;

#[derive(Serialize, Debug)]
pub struct SlackMessageResponse {
    text: Option<String>,
    attachments: Option<Vec<attachment::Attachment>>,
    response_type: String,
    username: Option<String>,
    as_user: bool,
    channel: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SlackMessagePost {
    text: Option<String>,
    channel: String,
    attachments: Option<Vec<attachment::Attachment>>,
    username: Option<String>,
    as_user: bool,
    icon_url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SlackMessageUpdate {
    as_user: Option<bool>,
    attachments: Option<Vec<attachment::Attachment>>,
    channel: String,
    text: Option<String>,
    ts: String,
}

#[derive(Serialize, Debug)]
pub struct SlackCreateComment {
    timestamp: String,
    channel: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct SlackMessagePostResponse {
    pub channel: Option<String>,
    pub ts: Option<String>,
    pub ok: bool,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SlackMessageUpdateResponse {
    pub channel: Option<String>,
    pub ts: Option<String>,
    pub ok: bool,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SlackCreateCommentResponse {
    pub ok: bool,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SlackRequest {
    pub text: String,
    token: String,
    pub response_url: String,
    pub channel_id: String,
}

#[derive(Clone)]
pub struct SlackClient {
    url: String,
    pub channel: String,
    pub client_id: String,
    pub client_secret: String,
    client: reqwest::Client,
}

pub enum Reaction {
    Comment,
    Approve,
}

trait SlackResponse {
    fn ok(&self) -> bool;
    fn error(&self) -> String;
}

impl SlackResponse for SlackCreateCommentResponse {
    fn ok(&self) -> bool {
        self.ok
    }

    fn error(&self) -> String {
        self.error
            .clone()
            .unwrap_or_else(|| "Unkown Error".to_string())
    }
}

impl SlackResponse for SlackMessageUpdateResponse {
    fn ok(&self) -> bool {
        self.ok
    }

    fn error(&self) -> String {
        self.error
            .clone()
            .unwrap_or_else(|| "Unkown Error".to_string())
    }
}

impl SlackResponse for SlackMessagePostResponse {
    fn ok(&self) -> bool {
        self.ok
    }

    fn error(&self) -> String {
        self.error
            .clone()
            .unwrap_or_else(|| "Unkown Error".to_string())
    }
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Reaction::Approve => write!(f, "white_check_mark"),
            Reaction::Comment => write!(f, "eyes"),
        }
    }
}

impl SlackClient {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        url: String,
        token: &str,
        channel: String,
        client_id: String,
        client_secret: String,
    ) -> Result<SlackClient> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(SlackClient {
            url,
            client,
            channel,
            client_id,
            client_secret,
        })
    }

    pub fn post_message(
        &self,
        pull_request: &github::PRResult,
        files: &str,
        channel: &str,
        user: Option<models::User>,
    ) -> Result<SlackMessagePostResponse> {
        let additions = format!("(+{} -{})", pull_request.additions, pull_request.deletions);
        let title = format!("{} {}", additions, pull_request.title);
        let as_user = user.is_some();

        let message = serde_json::to_string(&SlackMessagePost {
            text: None,
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
                &title,
            )]),
            channel: channel.to_string(),
            username: None,
            icon_url: Some(pull_request.user.avatar_url.to_string()),
            as_user,
        })?;

        let mut request = self
            .client
            .post(&format!("{}/{}", self.url, "chat.postMessage"));

        if let Some(user) = user {
            request = request.header(
                reqwest::header::AUTHORIZATION,
                &format!("Bearer {}", user.slack_access_token),
            );
        }

        request
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()?
            .error_for_status()?
            .json::<SlackMessagePostResponse>()
            .map_err(|e| e.into())
            .and_then(handle_response)
    }

    pub fn update_message(
        &self,
        pull_request: &github::PRResult,
        files: &str,
        ts: &str,
        channel: &str,
        user: Option<models::User>,
    ) -> Result<SlackMessageUpdateResponse> {
        let additions = format!("(+{} -{})", pull_request.additions, pull_request.deletions);
        let title = format!("{} {}", additions, pull_request.title);
        let as_user = user.is_some();

        let message = serde_json::to_string(&SlackMessageUpdate {
            text: None,
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
                &title,
            )]),
            channel: channel.to_string(),
            as_user: Some(as_user),
            ts: ts.to_string(),
        })?;

        let mut request = self.client.post(&format!("{}/{}", self.url, "chat.update"));
        if let Some(user) = user {
            request = request.header(
                reqwest::header::AUTHORIZATION,
                &format!("Bearer {}", user.slack_access_token),
            );
        }

        request
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()?
            .error_for_status()?
            .json::<SlackMessageUpdateResponse>()
            .map_err(|e| e.into())
            .and_then(handle_response)
    }

    pub fn immediate_response(&self, text: String) -> Result<String> {
        serde_json::to_string(&SlackMessageResponse {
            text: Some(text),
            attachments: None,
            response_type: "ephemeral".to_string(),
            username: None,
            as_user: true,
            channel: None,
        })
        .map_err(|e| e.into())
    }

    pub fn reviews_response(&self, text: &str, channel_id: &str) -> Result<()> {
        let response = serde_json::to_string(&SlackMessageResponse {
            text: Some(text.to_string()),
            attachments: None,
            response_type: "in_channel".to_string(),
            username: Some("Waiting for Review".to_string()),
            as_user: false,
            channel: Some(channel_id.to_string()),
        })?;

        self.client
            .post(&format!("{}/{}", self.url, "chat.postMessage"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(response)
            .send()?;

        Ok(())
    }

    pub fn add_reaction(
        &self,
        reaction: &Reaction,
        ts: &str,
        channel: &str,
        user: Option<models::User>,
    ) -> Result<SlackCreateCommentResponse> {
        let message = serde_json::to_string(&SlackCreateComment {
            timestamp: ts.to_string(),
            channel: channel.to_string(),
            name: format!("{}", reaction),
        })?;

        let mut request = self
            .client
            .post(&format!("{}/{}", self.url, "reactions.add"));

        if let Some(user) = user {
            request = request.header(
                reqwest::header::AUTHORIZATION,
                &format!("Bearer {}", user.slack_access_token),
            );
        }

        request
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()?
            .error_for_status()?
            .json()
            .map_err(|e| e.into())
            .and_then(handle_response)
    }

    pub fn get_token(&self, code: &str) -> Result<SlackAuthResponse> {
        let auth_code = encode(&format!("{}:{}", self.client_id, self.client_secret));
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Basic {}", auth_code)).unwrap(),
        );

        let client = reqwest::Client::new();
        client
            .post(&format!("{}/{}", self.url, "oauth.access"))
            .form(&[("code", code)])
            .headers(headers)
            .send()?
            .error_for_status()?
            .json()
            .map_err(|e| e.into())
            .and_then(handle_response)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SlackAuthResponse {
    pub ok: bool,
    pub scope: Option<String>,
    pub error: Option<String>,
    pub access_token: Option<String>,
    pub user: Option<SlackUserData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SlackUserData {
    pub name: String,
    pub id: String,
}

impl SlackResponse for SlackAuthResponse {
    fn ok(&self) -> bool {
        self.ok
    }

    fn error(&self) -> String {
        self.error
            .clone()
            .unwrap_or_else(|| "Unkown Error".to_string())
    }
}

fn handle_response<T: SlackResponse>(resp: T) -> Result<T> {
    if resp.ok() {
        Ok(resp)
    } else {
        Err(Error::SlackError(resp.error()))
    }
}
