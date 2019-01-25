use crate::github;

#[derive(Serialize, Debug)]
pub struct Field {
    title: String,
    value: String,
    short: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct Attachment {
    fallback: String,
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
    pub fn from_pull_request(pull_request: github::PRResult, files: &str) -> Attachment {
        let color = pull_request.color();
        let additions = format!("(+{} -{})", pull_request.additions, pull_request.deletions);
        let title = format!(
            "{}: {}",
            pull_request.base.repo.full_name, pull_request.title
        );

        Attachment {
            fallback: format!("{}", pull_request),
            color: Some(color),
            pretext: None,
            author_name: Some(pull_request.user.login),
            author_link: Some(pull_request.user.html_url),
            author_icon: Some(pull_request.user.avatar_url),
            title: Some(title),
            title_link: Some(pull_request.html_url),
            text: format!("{} {}", files, additions),
            fields: None,
            image_url: None,
            thumb_url: None,
            footer: None,
            footer_icon: None,
            ts: None,
        }
    }
}
