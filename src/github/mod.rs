extern crate url;

use self::url::Url;
use std::fmt;
use std::str::FromStr;

#[derive(Deserialize)]
struct User {
  login: String,
}

#[derive(Deserialize)]
pub struct PRResult {
  url: String,
  title: String,
  body: String,
  state: PRState,
  merged: bool,
  review_comments: u32,
  additions: u32,
  deletions: u32,

  user: User,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PRState {
  Open,
  Closed,
}

impl fmt::Display for PRResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "(+{additions} -{deletions}) {url} by {user}",
      additions = self.additions,
      deletions = self.deletions,
      url = self.url,
      user = self.user.login
    )
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
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "malformed repo url")
  }
}

impl std::error::Error for ParseError {
  fn description(&self) -> &str {
    "malformed repo url"
  }

  fn cause(&self) -> Option<&std::error::Error> {
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

pub struct GithubClient {
  url: String,
}

impl GithubClient {
  pub fn new(url: String) -> GithubClient {
    GithubClient { url: url }
  }

  pub fn get_pr(&self, pull_request: &PullRequest) -> reqwest::Result<PRResult> {
    let request_url = format!(
      "{url}/repos/{owner}/{repo}/pulls/{id}",
      url = self.url,
      owner = pull_request.owner,
      repo = pull_request.name,
      id = pull_request.id
    );

    reqwest::get(&request_url)?.error_for_status()?.json()
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
