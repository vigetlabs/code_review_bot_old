extern crate serde_yaml;

pub mod app_config;
pub mod db;

use actix_web::{HttpResponse, State};
use futures::future::Future;
use std::collections::HashMap;
use std::fs;

use crate::error::Error;
use app_config::AppConfig;

type FileExtension = String;
type LanguageIcon = String;

pub type Languages = HashMap<FileExtension, LanguageIcon>;
pub type Response = Box<Future<Item = HttpResponse, Error = Error>>;

#[derive(Deserialize, Debug, Clone)]
struct Language {
    group: Option<String>,
    #[serde(default = "Vec::new")]
    extensions: Vec<String>,
}

fn language_lookup(name: &str) -> LanguageIcon {
    match name {
        "CSS" => ":css:",
        "Elixir" => ":elixir:",
        "HTML" => ":html:",
        "JavaScript" => ":js:",
        "Kotlin" => ":kotlin:",
        "Ruby" => ":ruby:",
        "Rust" => ":rust:",
        "SQL" => ":sql:",
        "Swift" => ":swift:",
        "TypeScript" => ":ts:",
        _ => "",
    }
    .to_string()
}

pub fn load_languages() -> Result<Languages, &'static str> {
    let yaml = fs::read_to_string("resources/languages.yml").map_err(|_| "Can't read file")?;
    let map: HashMap<String, Language> =
        serde_yaml::from_str(&yaml).map_err(|_| "Can't parse yaml")?;
    let mut result: Languages = HashMap::new();

    for (key, value) in map.iter() {
        for ext in value.extensions.iter() {
            if let Some(group) = &value.group {
                result.insert(ext.to_string(), language_lookup(&group));
            } else {
                result.insert(ext.to_string(), language_lookup(key));
            }
        }
    }

    Ok(result)
}

pub fn prepare_response(body: &str) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body.to_string())
}

pub struct RequestAction<T, U> {
    pub state: State<AppConfig>,
    pub json: T,
    pub value: U,
}

impl<T, U> RequestAction<T, U> {
    pub fn new(state: State<AppConfig>, json: T, value: U) -> Self {
        RequestAction { state, json, value }
    }

    pub fn with_value<V>(self, value: V) -> RequestAction<T, V> {
        let Self { state, json, .. } = self;
        RequestAction { state, json, value }
    }

    pub fn add_value<V>(self, value: V) -> RequestAction<T, (U, V)> {
        let Self {
            state,
            json,
            value: prev_value,
        } = self;
        RequestAction {
            state,
            json,
            value: (prev_value, value),
        }
    }
}
