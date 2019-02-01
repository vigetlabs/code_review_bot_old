mod attachment;
use crate::github;

use std::fmt;

#[derive(Serialize, Debug)]
pub struct SlackResponse {
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
    pub channel: String,
    pub ts: String,
}

#[derive(Deserialize, Debug)]
pub struct SlackMessageUpdateResponse {
    pub channel: String,
    pub ts: String,
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
    pub fn new(url: String, token: &str, channel: String) -> Result<SlackClient, &'static str> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|_| "Invalid header value")?,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| "Cannot build client")?;

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
    ) -> Result<SlackMessagePostResponse, &'static str> {
        let message = serde_json::to_string(&SlackMessagePost {
            text: None,
            channel: self.channel.to_string(),
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
            )]),
        })
        .map_err(|_| "Json serialize error")?;

        self.client
            .post(&format!("{}/{}", self.url, "chat.postMessage"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()
            .map_err(|_| "Slack post error")?
            .error_for_status()
            .map_err(|_| "Slack post error")?
            .json()
            .map_err(|_| "Json parse error")
    }

    pub fn update_message(
        &self,
        pull_request: &github::PRResult,
        files: &str,
        ts: &str,
        channel: &str,
    ) -> Result<SlackMessageUpdateResponse, &'static str> {
        let message = serde_json::to_string(&SlackMessageUpdate {
            as_user: Some(true),
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
            )]),
            channel: channel.to_string(),
            text: None,
            ts: ts.to_string(),
        })
        .map_err(|_| "Json serialize error")?;

        self.client
            .post(&format!("{}/{}", self.url, "chat.update"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()
            .map_err(|_| "Slack post error")?
            .error_for_status()
            .map_err(|_| "Slack post error")?
            .json()
            .map_err(|e| {
                println!("{:?}", e);
                "Json parse error"
            })
    }

    pub fn immediate_response(&self, text: String) -> Result<String, serde_json::Error> {
        serde_json::to_string(&SlackResponse {
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
    ) -> Result<(), &'static str> {
        let response = serde_json::to_string(&SlackResponse {
            text: None,
            attachments: Some(vec![attachment::Attachment::from_pull_request(
                pull_request,
                files,
            )]),
            response_type: "in_channel".to_string(),
        })
        .map_err(|_| "Json serialize error")?;

        self.client
            .post(response_url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(response)
            .send()
            .map_err(|_| "Slack send error")?;

        Ok(())
    }

    pub fn add_reaction(
        &self,
        reaction: &Reaction,
        ts: &str,
        channel: &str,
    ) -> Result<SlackCreateCommentResponse, &'static str> {
        let message = serde_json::to_string(&SlackCreateComment {
            timestamp: ts.to_string(),
            channel: channel.to_string(),
            name: format!("{}", reaction),
        })
        .map_err(|_| "Json serialize error")?;

        println!("{:?}", message);
        self.client
            .post(&format!("{}/{}", self.url, "reactions.add"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(message)
            .send()
            .map_err(|_| "Slack post error")?
            .error_for_status()
            .map_err(|_| "Slack post error")?
            .json()
            .map_err(|_| "Json parse error")
    }
}
