use actix_web::{AsyncResponder, Form, HttpResponse, Json, State};
use futures::future;
use futures::future::Future;

use crate::error::{Error, Result};
use crate::github::{PRResult, PullRequest};
use crate::slack::{attachment, extract_links, SlackRequest};
use crate::utils::db::{FindPullRequest, NewPullRequest, PullRequestsByState};
use crate::utils::{prepare_response, RequestAction, Response};
use crate::AppConfig;

pub fn review((form, state): (Form<SlackRequest>, State<AppConfig>)) -> Result<HttpResponse> {
    if form.text.trim().is_empty() {
        let response = state.slack.immediate_response(
            "Specify pull request For example: /code_review_bot http://github.com/facebook/react/pulls/123".to_string(),
        )?;
        return Ok(prepare_response(&response));
    }

    let url = form.text.to_lowercase().to_string();
    let pull_request: PullRequest = url.parse()?;
    let pr_response = state.github.get_pr(&pull_request)?;

    let pr_files: String = state
        .github
        .get_files(&pr_response, &state.language_lookup)?;

    state
        .slack
        .post_message(&pr_response, &pr_files, &form.channel_id)?;

    let message = state.slack.immediate_response(
        "To have these automatically posted for you see: \
        <https://github.com/vigetlabs/code_review_bot/blob/master/README.md#adding-a-webhook-recommended\
        |Find out more>".to_string()
    )?;
    Ok(prepare_response(&message))
}

pub fn reviews((form, state): (Form<SlackRequest>, State<AppConfig>)) -> Response {
    state
        .db
        .send(PullRequestsByState {
            state: "open".to_string(),
        })
        .map_err(|e| e.into())
        .and_then(|res| res)
        .and_then(move |prs| {
            let open_prs: Vec<String> = if prs.is_empty() {
                vec!["All PRs Reviewed! :partyparrot:".to_string()]
            } else {
                prs.iter().map(|pr| pr.display_text.to_string()).collect()
            };

            state
                .slack
                .reviews_response(&open_prs.join("\n"), &form.channel_id)
        })
        .map(|_| prepare_response(""))
        .responder()
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

#[derive(Deserialize, Debug, Clone)]
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

pub fn message((json, state): (Json<SlackEventWrapper>, State<AppConfig>)) -> Response {
    let Json(event_wrapper) = json;

    match event_wrapper {
        SlackEventWrapper::UrlVerification { challenge, .. } => handle_url_verification(challenge),
        SlackEventWrapper::EventCallback { event, .. } => handle_event(event, state),
    }
    .responder()
}

fn handle_url_verification(challenge: String) -> Response {
    future::result(serde_json::to_string(&UrlVerification { challenge }))
        .map_err(|e| e.into())
        .map(|res| prepare_response(&res))
        .responder()
}

fn handle_event(event: SlackEvent, state: State<AppConfig>) -> Response {
    let SlackEvent::Message {
        mut text,
        channel,
        ts,
        subtype,
        attachments,
        ..
    } = event.clone();

    if subtype.is_some() && subtype.unwrap_or_else(|| "".to_string()) != "bot_message" {
        return future::ok(prepare_response("")).responder();
    }

    if let Some(atts) = attachments {
        text = format!(
            "{}{}",
            text,
            atts.iter()
                .map(|att| att.fallback.clone())
                .collect::<Vec<String>>()
                .join("")
        );
    }

    future::result(
        extract_links(&text)
            .iter()
            .filter_map(|url| url.parse::<PullRequest>().ok())
            .filter_map(|pr| state.github.get_pr(&pr).map(|res| (pr, res)).ok())
            .nth(0)
            .ok_or_else(|| Error::GuardError("No PR"))
            .map(|val| RequestAction::new(state, event, val)),
    )
    .then(|res| {
        let req_action = res?;
        let RequestAction {
            value: (_, res), ..
        } = &req_action;

        if res.open() {
            Ok(req_action)
        } else {
            Err(Error::GuardError("PR Already Closed"))
        }
    })
    .and_then(|req_action| {
        let RequestAction {
            state,
            value: (_, res),
            ..
        } = &req_action;
        state
            .db
            .send(FindPullRequest {
                github_id: github_id(&res),
            })
            .map_err(|e| e.into())
            .and_then(|db_res| match db_res {
                Ok(_) => Err(Error::GuardError("PR Already Created")),
                Err(_) => Ok(req_action),
            })
    })
    .and_then(|req_action| {
        let RequestAction {
            state,
            value: (pr, _),
            ..
        } = &req_action;
        state
            .github
            .create_webhook(&pr, &state.webhook_url)
            .map(|_| req_action)
    })
    .and_then(move |req_action| {
        let RequestAction {
            state,
            value: (_, res),
            ..
        } = &req_action;
        state
            .db
            .send(NewPullRequest {
                github_id: github_id(&res),
                state: "open".to_string(),
                slack_message_id: ts.to_string(),
                channel: channel.to_string(),
                display_text: format!("{}", res),
            })
            .map_err(|e| e.into())
    })
    .map(|_| prepare_response(""))
    .responder()
}

fn github_id(pr: &PRResult) -> String {
    format!("{}-{}", pr.base.repo.full_name, pr.number)
}
