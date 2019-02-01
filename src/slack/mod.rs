mod attachment;
use crate::github;

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

#[derive(Deserialize, Debug)]
pub struct SlackMessagePostResponse {
    pub channel: String,
    pub ts: String,
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
    client: reqwest::Client,
}

impl SlackClient {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(url: String, token: &str) -> Result<SlackClient, &'static str> {
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

        Ok(SlackClient { url, client })
    }

    pub fn post_message(
        &self,
        pull_request: &github::PRResult,
        files: &str,
    ) -> Result<SlackMessagePostResponse, &'static str> {
        let message = serde_json::to_string(&SlackMessagePost {
            text: None,
            channel: "#code-review-bot-test".to_string(),
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
}
