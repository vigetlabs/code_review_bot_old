use actix_web::{
    web::{Data, Form, Json},
    HttpResponse,
};

use crate::error::{Error, Result};
use crate::github::{PRResult, PullRequest};
use crate::models::{GithubUser, NewPullRequest, PullRequest as PullRequestModel};
use crate::slack::{attachment, extract_links, SlackRequest};
use crate::utils::prepare_response;
use crate::AppConfig;

pub fn review(form: Form<SlackRequest>, state: Data<AppConfig>) -> Result<HttpResponse> {
    if form.text.trim().is_empty() {
        let res = state.slack.immediate_response(
                "Specify pull request For example: /code_review_bot http://github.com/facebook/react/pulls/123".to_string(),
            )?;
        return Ok(prepare_response(&res));
    }

    let pull_request = form.text.to_lowercase().parse()?;
    let pr_response = state.github.get_pr(&pull_request)?;
    let pr_files = state
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

pub fn reviews(form: Form<SlackRequest>, state: Data<AppConfig>) -> Result<HttpResponse> {
    let prs = PullRequestModel::by_state("open", &state.db)?;

    let open_prs: Vec<String> = if prs.is_empty() {
        vec!["All PRs Reviewed! :partyparrot:".to_string()]
    } else {
        prs.iter().map(|pr| pr.display_text.to_string()).collect()
    };

    state
        .slack
        .reviews_response(&open_prs.join("\n"), &form.channel_id)?;
    Ok(prepare_response(""))
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

pub fn message((json, state): (Json<SlackEventWrapper>, Data<AppConfig>)) -> Result<HttpResponse> {
    let Json(event_wrapper) = json;

    match event_wrapper {
        SlackEventWrapper::UrlVerification { challenge, .. } => handle_url_verification(challenge),
        SlackEventWrapper::EventCallback { event, .. } => handle_event(event, state),
    }
}

fn handle_url_verification(challenge: String) -> Result<HttpResponse> {
    let res = serde_json::to_string(&UrlVerification { challenge })?;

    Ok(prepare_response(&res))
}

fn handle_event(event: SlackEvent, state: Data<AppConfig>) -> Result<HttpResponse> {
    let SlackEvent::Message {
        mut text,
        channel,
        ts,
        subtype,
        attachments,
        ..
    } = event.clone();

    if subtype.is_some() && subtype.unwrap_or_else(|| "".to_string()) != "bot_message" {
        return Ok(prepare_response(""));
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

    let result = extract_links(&text)
        .iter()
        .filter_map(|url| url.parse::<PullRequest>().ok())
        .filter_map(|pr| state.github.get_pr(&pr).map(|res| (pr, res)).ok())
        .nth(0)
        .ok_or_else(|| Error::GuardError("No PR"))?;

    let (pr, res) = result;

    if res.open() {
        return Err(Error::GuardError("PR Already Closed"));
    }

    let db_pr = PullRequestModel::find(&github_id(&res), &state.db);

    if db_pr.is_ok() {
        return Err(Error::GuardError("PR Already Closed"));
    }

    state.github.create_webhook(&pr, &state.webhook_url)?;

    let requester = GithubUser::find_or_create(&res.user, &state.db)?;
    PullRequestModel::create(
        &NewPullRequest {
            github_id: github_id(&res),
            state: "open".to_string(),
            slack_message_id: ts.to_string(),
            channel: channel.to_string(),
            display_text: format!("{}", res),
            github_user_id: requester.id,
        },
        &state.db,
    )?;

    Ok(prepare_response(""))
}

fn github_id(pr: &PRResult) -> String {
    format!("{}-{}", pr.base.repo.full_name, pr.number)
}
