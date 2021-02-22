use serde::{Deserialize, Serializer};
use std::str::FromStr;

use crate::error::{Result, UrlParseError};

#[derive(Debug, Serialize)]
pub struct PaginatedResource<T> {
    pub resources: Vec<T>,
    #[serde(serialize_with = "url_to_string")]
    pub next: Option<url::Url>,
    #[serde(serialize_with = "url_to_string")]
    pub prev: Option<url::Url>,
    #[serde(serialize_with = "url_to_string")]
    pub first: Option<url::Url>,
    #[serde(serialize_with = "url_to_string")]
    pub last: Option<url::Url>,
}

fn url_to_string<S>(url: &Option<url::Url>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(url) = url {
        serializer.serialize_str(url.as_str())
    } else {
        serializer.serialize_none()
    }
}

impl<T> PaginatedResource<T> {
    pub fn new(resources: Vec<T>, link_values: &[hyperx::header::LinkValue]) -> Result<Self> {
        let mut resource = Self {
            resources,
            next: None,
            prev: None,
            first: None,
            last: None,
        };

        for value in link_values {
            if let Some(rels) = value.rel() {
                let url = url::Url::from_str(value.link()).map_err(UrlParseError::Parse)?;

                for rel in rels.iter() {
                    match rel {
                        hyperx::header::RelationType::Next => resource.next.replace(url.clone()),
                        hyperx::header::RelationType::Prev => resource.prev.replace(url.clone()),
                        hyperx::header::RelationType::First => resource.first.replace(url.clone()),
                        hyperx::header::RelationType::Last => resource.last.replace(url.clone()),
                        _ => None,
                    };
                }
            }
        }

        Ok(resource)
    }

    pub fn next(&self) -> Option<String> {
        self.next
            .as_ref()
            .and_then(|url| url.query())
            .map(|query| format!("?{}", query))
    }

    pub fn prev(&self) -> Option<String> {
        self.prev
            .as_ref()
            .and_then(|url| url.query())
            .map(|query| format!("?{}", query))
    }

    pub fn first(&self) -> Option<String> {
        self.first
            .as_ref()
            .and_then(|url| url.query())
            .map(|query| format!("?{}", query))
    }

    pub fn last(&self) -> Option<String> {
        self.last
            .as_ref()
            .and_then(|url| url.query())
            .map(|query| format!("?{}", query))
    }
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<String>,
}
