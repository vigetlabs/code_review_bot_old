use crate::github;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Field {
    title: String,
    value: String,
    short: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Attachment {
    pub fallback: String,
    color: Option<String>,
    pretext: Option<String>,
    author_name: Option<String>,
    author_link: Option<String>,
    author_icon: Option<String>,
    title: Option<String>,
    title_link: Option<String>,
    text: String,
    fields: Option<Vec<Field>>,
    image_url: Option<String>,
    thumb_url: Option<String>,
    footer: Option<String>,
    footer_icon: Option<String>,
    ts: Option<u64>,
}

impl Attachment {
    pub fn from_pull_request(
        pull_request: &github::PRResult,
        files: Vec<String>,
        title: &str,
    ) -> Attachment {
        let color = pull_request.color();
        let text = format!(
            "<{}|{}> by {} {}",
            pull_request.html_url,
            pull_request.base.repo.full_name,
            pull_request.user.login,
            files.join(" ")
        );

        Attachment {
            fallback: format!("{}", pull_request),
            color: Some(color),
            pretext: None,
            author_name: None,
            author_link: None,
            author_icon: None,
            title: Some(title.to_owned()),
            title_link: None,
            text,
            fields: None,
            image_url: None,
            thumb_url: None,
            footer: None,
            footer_icon: None,
            ts: None,
        }
    }
}
