mod attachment;
use crate::github;

mod error;
use error::SlackError;

use std::fmt;

#[derive(Serialize, Debug)]
pub struct SlackMessageResponse {
    text: Option<String>,
    attachments: Option<Vec<attachment::Attachment>>,
    response_type: String,
}

#[derive(Serialize, Debug)]
pub struct SlackMessagePost {
    text: Option<String>,
    channel: String,
    attachments: Option<Vec<attachment::Attachment>>,
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
}

#[derive(Clone)]
pub struct SlackClient {
    url: String,
    channel: String,
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
    pub fn new(url: String, token: &str, channel: String) -> Result<SlackClient, SlackError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(SlackError::Client)?;

        Ok(SlackClient {
            url,
            client,
            channel,
        })
    }

    pub fn post_message(
        &self,
        pull_request: &github::PRResult,
        files: &str,
    ) -> Result<SlackMessagePostResponse, SlackError> {
        let message = serde_json::to_string(&SlackMessagePost {
            text: None,
            channel: self.channel.to_string(),
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
            )]),
        })?;

        self.client
            .post(&format!("{}/{}", self.url, "chat.postMessage"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()?
            .error_for_status()?
            .json()
            .map_err(SlackError::Request)
            .and_then(handle_response)
    }

    pub fn update_message(
        &self,
        pull_request: &github::PRResult,
        files: &str,
        ts: &str,
        channel: &str,
    ) -> Result<SlackMessageUpdateResponse, SlackError> {
        let message = serde_json::to_string(&SlackMessageUpdate {
            as_user: Some(true),
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
            )]),
            channel: channel.to_string(),
            text: None,
            ts: ts.to_string(),
        })?;

        self.client
            .post(&format!("{}/{}", self.url, "chat.update"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()?
            .error_for_status()?
            .json()
            .map_err(SlackError::Request)
            .and_then(handle_response)
    }

    pub fn immediate_response(&self, text: String) -> Result<String, serde_json::Error> {
        serde_json::to_string(&SlackMessageResponse {
            text: Some(text),
            attachments: None,
            response_type: "ephemeral".to_string(),
        })
    }

    pub fn response(
        &self,
        pull_request: &github::PRResult,
        files: &str,
        response_url: &str,
    ) -> Result<(), SlackError> {
        let response = serde_json::to_string(&SlackMessageResponse {
            text: None,
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
            )]),
            response_type: "in_channel".to_string(),
        })?;

        self.client
            .post(response_url)
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
    ) -> Result<SlackCreateCommentResponse, SlackError> {
        let message = serde_json::to_string(&SlackCreateComment {
            timestamp: ts.to_string(),
            channel: channel.to_string(),
            name: format!("{}", reaction),
        })?;

        self.client
            .post(&format!("{}/{}", self.url, "reactions.add"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()?
            .error_for_status()?
            .json()
            .map_err(SlackError::Request)
            .and_then(handle_response)
    }
}

fn handle_response<T: SlackResponse>(resp: T) -> Result<T, SlackError> {
    if resp.ok() {
        Ok(resp)
    } else {
        Err(SlackError::Failed(resp.error()))
    }
}
