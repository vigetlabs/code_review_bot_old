use crate::error::Result;

#[derive(Serialize)]
pub struct GHTokenParams<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
}

#[derive(Deserialize)]
pub struct GHAuthResponse {
    pub access_token: String,
}

#[derive(Clone)]
pub struct GithubOauthClient {
    pub client_id: String,
    client_secret: String,
}

impl GithubOauthClient {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
        }
    }

    pub fn get_token(&self, code: &str) -> Result<GHAuthResponse> {
        let params = GHTokenParams {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            code,
        };

        reqwest::Client::new()
            .post("https://github.com/login/oauth/access_token")
            .header(reqwest::header::ACCEPT, "application/json")
            .form(&params)
            .send()?
            .error_for_status()?
            .json()
            .map_err(|e| e.into())
    }
}
