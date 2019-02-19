use actix_web::{error, AsyncResponder, Form, FutureResponse, HttpResponse, Json, State};
use futures::future;
use futures::future::Future;

use crate::github::{PRResult, PullRequest};
use crate::slack::{attachment, extract_links, SlackRequest};
use crate::utils::db::{FindPullRequest, NewPullRequest, PullRequestsByState};
use crate::AppConfig;

pub fn review(
  (form, state): (Form<SlackRequest>, State<AppConfig>),
) -> actix_web::Result<HttpResponse> {
  if form.text.trim().is_empty() {
    let response = state.slack.immediate_response(
            "Specify pull request For example: /code_review_bot http://github.com/facebook/react/pulls/123".to_string(),
        )?;
    return prepare_response(response);
  }

  let url = form.text.to_lowercase().to_string();
  let pull_request: PullRequest = url.parse()?;
  let pr_response = state
    .github
    .get_pr(&pull_request)
    .map_err(error::ErrorNotFound)?;

  let pr_files: String = state
    .github
    .get_files(&pr_response, &state.language_lookup)
    .map_err(error::ErrorBadRequest)?;

  state
    .slack
    .post_message(&pr_response, &pr_files, &form.channel_id)
    .map_err(error::ErrorNotFound)?;

  let message = state.slack.immediate_response(
    "To have these automatically posted for you see: \
     <https://github.com/vigetlabs/code_review_bot/blob/master/README.md#adding-a-webhook-recommended\
     |Find out more>".to_string()
  )?;
  prepare_response(message)
}

pub fn reviews(
  (form, state): (Form<SlackRequest>, State<AppConfig>),
) -> FutureResponse<HttpResponse> {
  state
    .db
    .send(PullRequestsByState {
      state: "open".to_string(),
    })
    .map_err(error::ErrorBadRequest)
    .and_then(move |res| match res {
      Ok(prs) => {
        let open_prs: Vec<String> = if prs.is_empty() {
          vec!["All PRs Reviewed! :partyparrot:".to_string()]
        } else {
          prs.iter().map(|pr| pr.display_text.to_string()).collect()
        };

        state
          .slack
          .reviews_response(&open_prs.join("\n"), &form.channel_id)
          .map_err(error::ErrorNotFound)
      }
      Err(e) => Err(error::ErrorNotFound(e)),
    })
    .and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body("")))
    .responder()
}

fn prepare_response(body: String) -> actix_web::Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(body),
  )
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SlackEventWrapper {
  UrlVerification {
    token: String,
    challenge: String,
  },
  EventCallback {
    token: String,
    team_id: String,
    api_app_id: String,
    event: SlackEvent,
    event_id: String,
    event_time: u32,
  },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SlackEvent {
  Message {
    channel: String,
    user: Option<String>,
    subtype: Option<String>,
    attachments: Option<Vec<attachment::Attachment>>,
    text: String,
    ts: String,
  },
}

#[derive(Serialize, Debug)]
pub struct UrlVerification {
  challenge: String,
}

pub fn message(
  (json, state): (Json<SlackEventWrapper>, State<AppConfig>),
) -> FutureResponse<HttpResponse> {
  let Json(event_wrapper) = json;

  match event_wrapper {
    SlackEventWrapper::UrlVerification { challenge, .. } => {
      future::Either::A(handle_url_verification(challenge))
    }
    SlackEventWrapper::EventCallback { event, .. } => future::Either::B(handle_event(event, state)),
  }
  .responder()
}

fn handle_url_verification(
  challenge: String,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
  future::ok(0)
    .and_then(|_| serde_json::to_string(&UrlVerification { challenge }))
    .map_err(actix_web::error::ErrorBadRequest)
    .and_then(prepare_response)
}

fn handle_event(
  event: SlackEvent,
  state: State<AppConfig>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
  let SlackEvent::Message {
    mut text,
    channel,
    ts,
    subtype,
    attachments,
    ..
  } = event;
  if let Some(atts) = attachments {
    text = format!(
      "{}{}",
      text,
      atts
        .iter()
        .map(|att| att.fallback.clone())
        .collect::<Vec<String>>()
        .join("")
    );
  }

  let handle_slack_message = future::ok(0)
    .and_then(move |_| {
      extract_links(&text)
        .iter()
        .filter_map(|url| url.parse::<PullRequest>().ok())
        .filter_map(|pr| state.github.get_pr(&pr).map(|res| (pr, res)).ok())
        .nth(0)
        .ok_or_else(|| "No PR".to_string())
        .map(|(pr, res)| (pr, res, state))
    })
    .then(move |res| {
      let (pr, res, state) = res?;
      if res.open() {
        Ok((pr, res, state))
      } else {
        Err("PR Already Closed".to_string())
      }
    })
    .and_then(move |(pr, res, state)| {
      state
        .db
        .send(FindPullRequest {
          github_id: github_id(&res),
        })
        .map_err(|e| format!("{}", e))
        .and_then(|db_res| match db_res {
          Ok(_) => Err("PR Already Created".to_string()),
          Err(_) => Ok((pr, res, state)),
        })
    })
    .and_then(move |(pr, res, state)| {
      state
        .db
        .send(NewPullRequest {
          github_id: github_id(&res),
          state: "open".to_string(),
          slack_message_id: ts,
          channel,
          display_text: format!("{}", res),
        })
        .map(|_| (pr, state))
        .map_err(|e| format!("{}", e))
    })
    .and_then(move |(pr, state)| state.github.create_webhook(&pr, &state.webhook_url))
    .then(|_| prepare_response("".to_string()));

  if subtype.is_none() || subtype.unwrap_or_else(|| "".to_string()) == "bot_message" {
    future::Either::A(handle_slack_message)
  } else {
    future::Either::B(
      future::ok(0).and_then(|_| Ok(HttpResponse::Ok().content_type("application/json").body(""))),
    )
  }
}

fn github_id(pr: &PRResult) -> String {
  format!("{}-{}", pr.base.repo.full_name, pr.number)
}
