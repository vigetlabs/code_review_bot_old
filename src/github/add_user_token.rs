pub trait AddUserToken {
    fn add_token(self, token: &str) -> Self;
}

impl AddUserToken for reqwest::RequestBuilder {
    fn add_token(self, token: &str) -> Self {
        self.header(reqwest::header::AUTHORIZATION, format!("token {}", token))
    }
}
