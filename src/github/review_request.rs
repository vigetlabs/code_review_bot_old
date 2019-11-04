use std::str::FromStr;
use url::{self, Url};

use crate::error::UrlParseError;

#[derive(Debug)]
pub struct ReviewRequest {
    pub owner: String,
    pub name: String,
    pub id: String,
}

impl FromStr for ReviewRequest {
    type Err = UrlParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let repository_url = Url::parse(s).map_err(UrlParseError::Parse)?;
        let mut path = repository_url
            .path_segments()
            .ok_or(UrlParseError::MissingSegment)?;

        Ok(ReviewRequest {
            owner: path
                .nth(0)
                .ok_or(UrlParseError::MissingSegment)?
                .to_string(),
            name: path
                .nth(0)
                .ok_or(UrlParseError::MissingSegment)?
                .to_string(),
            id: path
                .nth(1)
                .ok_or(UrlParseError::MissingSegment)?
                .to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_url_sucess() {
        let repo: ReviewRequest = "http://github.com/facebook/react/pulls/1234"
            .parse()
            .expect("Can't parse url");
        assert_eq!(repo.id, "1234");
        assert_eq!(repo.owner, "facebook");
        assert_eq!(repo.name, "react");
    }

    #[test]
    fn test_parse_url_failure() {
        let repo = "totally invalid url".parse::<ReviewRequest>();
        assert!(repo.is_err(), "Should not parse")
    }
}
