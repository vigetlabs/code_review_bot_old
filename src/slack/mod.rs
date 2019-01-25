mod attachment;
use crate::github;

#[derive(Serialize, Debug)]
pub struct SlackResponse {
    text: Option<String>,
    attachments: Option<Vec<attachment::Attachment>>,
    response_type: String,
}

#[derive(Deserialize, Debug)]
pub struct SlackRequest {
    pub text: String,
    token: String,
    pub response_url: String,
}

pub struct SlackClient {
    client: reqwest::Client,
}

impl SlackClient {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<SlackClient, &'static str> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(|_| "Cannot build client")?;

        Ok(SlackClient { client })
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
        pull_request: github::PRResult,
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
