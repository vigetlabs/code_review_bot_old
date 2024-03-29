use actix_web::{
    web::{Data, Form, Json},
    HttpResponse,
};

use crate::db::DBExecutor;
use crate::error::Result;
use crate::models::{IconMapping, PullRequest as PullRequestModel, User};
use crate::slack::{attachment, SlackRequest};
use crate::utils::prepare_response;
use crate::AppData;

pub async fn review(
    form: Form<SlackRequest>,
    state: AppData,
    db: Data<DBExecutor>,
) -> Result<HttpResponse> {
    let access_token = if let Some(token) =
        User::find_by_slack_id(&form.user_id, &db)?.and_then(|user| user.github_access_token)
    {
        token
    } else {
        let res = state.slack.immediate_response(
            format!("To submit a pull request you must first sign in and connect your account to github here {}.", state.app_url),
        )?;
        return Ok(prepare_response(&res));
    };
    if form.text.trim().is_empty() {
        let res = state.slack.immediate_response(
                "Specify pull request For example: /code_review_bot http://github.com/facebook/react/pulls/123".to_string(),
            )?;
        return Ok(prepare_response(&res));
    }

    let pull_request = form.text.to_lowercase().parse()?;
    let pr_response = state.github.get_pr(&pull_request, &access_token).await?;
    let (filenames, extensions): (Vec<_>, Vec<_>) = state
        .github
        .get_files(&pr_response, &access_token)
        .await
        .map(|files| {
            files
                .into_iter()
                .map(|file| (file.filename(), file.extension()))
        })?
        .unzip();
    let filenames: Vec<String> = filenames.into_iter().filter_map(|o| o).collect();
    let mut extensions: Vec<String> = extensions.into_iter().filter_map(|o| o).collect();
    extensions.dedup();

    let mappings = IconMapping::from(filenames, extensions, &db)?;

    state
        .slack
        .post_message(
            &pr_response,
            mappings,
            &form.channel_id,
            &state.app_url,
            None,
        )
        .await?;

    Ok(prepare_response(""))
}

pub async fn reviews(
    form: Form<SlackRequest>,
    state: AppData,
    db: Data<DBExecutor>,
) -> Result<HttpResponse> {
    let prs = PullRequestModel::by_state("open", &db)?;

    let open_prs: Vec<String> = if prs.is_empty() {
        vec!["All PRs Reviewed! :partyparrot:".to_string()]
    } else {
        prs.iter().map(|pr| pr.display_text.to_string()).collect()
    };

    state
        .slack
        .reviews_response(&open_prs.join("\n"), &form.channel_id)
        .await?;
    Ok(prepare_response(""))
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SlackEventWrapper {
    UrlVerification { token: String, challenge: String },
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

pub async fn message(json: Json<SlackEventWrapper>) -> Result<HttpResponse> {
    let Json(event_wrapper) = json;

    match event_wrapper {
        SlackEventWrapper::UrlVerification { challenge, .. } => handle_url_verification(challenge),
    }
}

fn handle_url_verification(challenge: String) -> Result<HttpResponse> {
    let res = serde_json::to_string(&UrlVerification { challenge })?;

    Ok(prepare_response(&res))
}
