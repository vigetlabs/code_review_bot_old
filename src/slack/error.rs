use std::error;
use std::fmt;

#[derive(Debug)]
pub enum SlackError {
  Client(reqwest::Error),
  Header(reqwest::header::InvalidHeaderValue),
  Request(reqwest::Error),
  Json(serde_json::Error),
  Failed(String),
}

impl From<reqwest::Error> for SlackError {
  fn from(err: reqwest::Error) -> Self {
    SlackError::Request(err)
  }
}

impl From<reqwest::header::InvalidHeaderValue> for SlackError {
  fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
    SlackError::Header(err)
  }
}

impl From<serde_json::Error> for SlackError {
  fn from(err: serde_json::Error) -> Self {
    SlackError::Json(err)
  }
}

impl fmt::Display for SlackError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      SlackError::Client(ref err) => write!(f, "Client Error: {}", err),
      SlackError::Header(ref err) => write!(f, "Header Error: {}", err),
      SlackError::Request(ref err) => write!(f, "Request Error: {}", err),
      SlackError::Json(ref err) => write!(f, "JSON Error: {}", err),
      SlackError::Failed(ref err) => write!(f, "Slack Message Failed: {}", err),
    }
  }
}

impl error::Error for SlackError {
  fn description(&self) -> &str {
    match *self {
      SlackError::Client(ref err) => err.description(),
      SlackError::Header(ref err) => err.description(),
      SlackError::Request(ref err) => err.description(),
      SlackError::Json(ref err) => err.description(),
      SlackError::Failed(ref err) => err,
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      SlackError::Client(ref err) => Some(err),
      SlackError::Header(ref err) => Some(err),
      SlackError::Request(ref err) => Some(err),
      SlackError::Json(ref err) => Some(err),
      SlackError::Failed(ref _err) => None,
    }
  }
}
