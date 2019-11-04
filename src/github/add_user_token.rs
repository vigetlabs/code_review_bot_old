pub trait AddUserToken {
    fn maybe_add_token(self, token: Option<String>) -> Self;
}

impl AddUserToken for reqwest::RequestBuilder {
    fn maybe_add_token(self, token: Option<String>) -> Self {
        if let Some(token) = token {
            self.header(reqwest::header::AUTHORIZATION, format!("token {}", token))
        } else {
            self
        }
    }
}
