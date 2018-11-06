use std::fmt;

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
