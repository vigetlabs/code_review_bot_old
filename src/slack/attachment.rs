use github;

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
  pub fn from_repository(pull_request: github::PRResult) -> Attachment {
    let color = pull_request.color();

    Attachment {
      fallback: format!("{}", pull_request),
      color: Some(color),
      pretext: None,
      author_name: Some(pull_request.user.login),
      author_link: Some(pull_request.user.html_url),
      author_icon: Some(pull_request.user.avatar_url),
      title: Some(pull_request.title),
      title_link: Some(pull_request.html_url),
      text: "".to_string(),
      fields: Some(vec![
        Field {
          title: "Size".to_string(),
          value: format!("(+{} -{})", pull_request.additions, pull_request.deletions),
          short: Some(true),
        },
        Field {
          title: "Files".to_string(),
          value: "".to_string(),
          short: Some(true),
        },
      ]),
      image_url: None,
      thumb_url: None,
      footer: None,
      footer_icon: None,
      ts: None,
    }
  }
}
