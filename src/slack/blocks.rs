use crate::github;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Block {
    Context(Context),
    Section(Section),
}

impl Block {
    pub fn from_pull_request(
        pull_request: &github::PRResult,
        files: Vec<crate::models::IconMapping>,
        additions: &str,
        url: &str,
    ) -> Vec<Block> {
        let text = format!(
            "<{}|{}> by {}",
            pull_request.html_url, pull_request.base.repo.full_name, pull_request.user.login,
        );

        let mut elements = vec![
            Elements::Image(Image::new(
                "pull request status".to_owned(),
                format!("{}{}", url, pull_request.image_path()),
            )),
            Elements::Text(Text {
                text_type: text_type(),
                text: additions.to_owned(),
            }),
        ];

        let mut images: Vec<Elements> = files
            .into_iter()
            .map(|icon| {
                Elements::Image(Image::new(
                    icon.file_type.clone(),
                    format!("{}{}", url, icon.image_path()),
                ))
            })
            .collect();

        elements.append(&mut images);
        elements.truncate(3);

        vec![
            Block::Section(Section {
                text: Text {
                    text_type: text_type(),
                    text: format!("*{}*\n{}", pull_request.title, text),
                },
            }),
            Block::Context(Context { elements }),
        ]
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Elements {
    Text(Text),
    Image(Image),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Context {
    elements: Vec<Elements>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Section {
    text: Text,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Text {
    #[serde(default = "text_type")]
    #[serde(rename = "type")]
    text_type: String,
    text: String,
}

fn text_type() -> String {
    "mrkdwn".to_owned()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    #[serde(default = "image_type")]
    #[serde(rename = "type")]
    image_type: String,
    image_url: String,
    alt_text: String,
}

impl Image {
    fn new(alt_text: String, image_url: String) -> Self {
        Self {
            image_type: image_type(),
            image_url,
            alt_text,
        }
    }
}

fn image_type() -> String {
    "image".to_owned()
}
