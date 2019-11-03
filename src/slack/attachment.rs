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
