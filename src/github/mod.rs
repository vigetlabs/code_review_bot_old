extern crate url;

use self::url::Url;
use std::fmt;
use std::str::FromStr;

#[derive(Deserialize)]
struct Owner {
  login: String,
  html_url: String,
}

#[derive(Deserialize)]
pub struct SearchResult {
  items: Vec<Repository>,
}

#[derive(Deserialize)]
struct Repository {
  name: String,
  html_url: String,
  description: Option<String>,
  owner: Owner,
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

  pub fn search(&self, q: &str, per_page: u32) -> reqwest::Result<SearchResult> {
    let request_url = format!(
      "{url}/search/repositories?q={q}&per_page={per_page}",
      url = self.url,
      q = q,
      per_page = per_page
    );

    let mut request = match reqwest::get(&request_url)?.error_for_status() {
      Ok(res) => res,
      Err(e) => {
        println!("Error {}", e);
        return Err(e);
      }
    };

    request.json()
  }
}

impl fmt::Display for SearchResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let res = self
      .items
      .iter()
      .map(|repo| {
        format!(
          "<{0}|{1}> by <{2}|{3}>\n{4}\n----",
          repo.html_url,
          repo.name,
          repo.owner.html_url,
          repo.owner.login,
          repo.description.as_ref().unwrap_or(&String::new())
        )
      }).collect::<Vec<String>>()
      .join("\n");
    write!(f, "{}", res)
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
