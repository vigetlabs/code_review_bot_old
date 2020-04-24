use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::db::DBExecutor;
use crate::error::{Error, Result};
use crate::github::{
    PRAction, PRFiles, PRReviewState, PullRequestEvent, ReviewAction, ReviewEvent,
};
use crate::models::{GithubUser, IconMapping, NewPullRequest, PullRequest, Review, User};
use crate::slack::Reaction;
use crate::utils::prepare_response;
use crate::AppData;

async fn handle_pull_request_opened(
    state: AppData,
    db: Data<DBExecutor>,
    json: PullRequestEvent,
) -> Result<HttpResponse> {
    if json.pull_request.draft {
        return Err(Error::GuardError("Ignoring Draft PR"));
    }

    let requester = GithubUser::find_or_create(&json.pull_request.user, &db, None)?;
    let user = requester
        .user_id
        .and_then(|id| User::find(id, &db).ok())
        .and_then(|inner| inner);

    let pr_files = PRFiles::new(
        &json.pull_request,
        &state.github,
        user.clone().and_then(|u| u.github_access_token),
    ).await;
    let mappings = IconMapping::from(pr_files.filenames, pr_files.extensions, &db)?;

    let result = state.slack.post_message(
        &json.pull_request,
        mappings,
        &state.slack.channel,
        &state.app_url,
        user,
    ).await?;

    PullRequest::create(
        &NewPullRequest {
            github_id: github_id(
                &json.pull_request.base.repo.full_name,
                json.pull_request.number,
            ),
            state: "open".to_string(),
            slack_message_id: result.ts.unwrap_or_else(|| "".to_string()),
            channel: result.channel.unwrap_or_else(|| "".to_string()),
            display_text: format!("{}", json.pull_request),
            github_user_id: requester.github_id,
        },
        &db,
    )?;

    Ok(prepare_response(""))
}

async fn handle_pull_request_closed(
    state: AppData,
    db: Data<DBExecutor>,
    json: PullRequestEvent,
) -> Result<HttpResponse> {
    let db_pr = PullRequest::find(
        &github_id(
            &json.pull_request.base.repo.full_name,
            json.pull_request.number,
        ),
        &db,
    )?
    .update("closed", &db)?;
    let user = db_pr.user(&db)?;

    let pr_files = PRFiles::new(
        &json.pull_request,
        &state.github,
        user.clone().and_then(|u| u.github_access_token),
    ).await;
    let mappings = IconMapping::from(pr_files.filenames, pr_files.extensions, &db)?;

    state.slack.update_message(
        &json.pull_request,
        mappings,
        &db_pr.slack_message_id,
        &db_pr.channel,
        &state.app_url,
        user,
    ).await?;
    Ok(prepare_response(""))
}

pub async fn pull_request(
    json: Json<PullRequestEvent>,
    state: AppData,
    db: Data<DBExecutor>,
) -> Result<HttpResponse> {
    match json.action {
        PRAction::Opened | PRAction::ReadyForReview => {
            handle_pull_request_opened(state, db, json.0).await
        }
        PRAction::Closed => handle_pull_request_closed(state, db, json.0).await,
        _ => Err(Error::GithubError(format!(
            "Unhandled PR Action: {:?}",
            json.action
        ))),
    }
}

pub async fn review(
    json: Json<ReviewEvent>,
    state: AppData,
    db: Data<DBExecutor>,
) -> Result<HttpResponse> {
    match json.action {
        ReviewAction::Submitted => handle_review_submitted(state, db, json.0).await,
        _ => Ok(prepare_response("")),
    }
}

async fn handle_review_submitted(
    state: AppData,
    db: Data<DBExecutor>,
    json: ReviewEvent,
) -> Result<HttpResponse> {
    if json.review.user.login == json.pull_request.user.login {
        return Err(Error::GuardError("Reviewer same as opened pull request"));
    }
    let mut reaction = Reaction::Comment;
    let mut approved = false;
    if let PRReviewState::Approved = json.review.state {
        reaction = Reaction::Approve;
        approved = true;
    }

    let reviewer = GithubUser::find_or_create(&json.review.user, &db, None)?;
    let reviewer_user = reviewer.user(&db)?;
    let mut db_pr = PullRequest::find(
        &github_id(
            &json.pull_request.base.repo.full_name,
            json.pull_request.number,
        ),
        &db,
    )?;
    db_pr = db_pr.update(&next_state(&db_pr.state, approved), &db)?;
    Review::create_or_update(&reviewer, &db_pr, &json.review.state.to_string(), &db)?;

    state.slack.add_reaction(
        &reaction,
        &db_pr.slack_message_id,
        &db_pr.channel,
        reviewer_user,
    ).await?;

    Ok(prepare_response(""))
}

fn github_id(repo: &str, number: u32) -> String {
    format!("{}-{}", repo, number)
}

fn next_state(state: &str, approved: bool) -> String {
    if state == "open" && approved {
        "approved".to_string()
    } else {
        state.to_string()
    }
}
