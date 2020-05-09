use actix_session::Session;
use actix_web::web::{Data, Form, Path, Query};
use actix_web::HttpResponse;

use crate::db::DBExecutor;
use crate::error::{Error, Result};
use crate::github;
use crate::models::{NewWebhook, Webhook};
use crate::utils::{helpers::get_current_user, paginated_resource, prepare_response};
use crate::AppData;

#[derive(Debug, Serialize)]
struct ReposResponse<'a> {
    repos: Vec<Repo<'a>>,
    pagination: &'a paginated_resource::PaginatedResource<github::Repo>,
}

#[derive(Debug, Serialize)]
struct Repo<'a> {
    repo: &'a github::Repo,
    webhook: Option<&'a Webhook>,
}

pub async fn repos(
    state: AppData,
    db: Data<DBExecutor>,
    session: Session,
    params: Query<paginated_resource::PaginationParams>,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&db, &session)?.ok_or(Error::NotAuthedError)?;
    let access_token = current_user
        .github_access_token
        .ok_or(Error::NotAuthedError)?;

    let github_repos = state.github.get_repos(&access_token, &*params).await?;

    let webhooks = Webhook::for_repos(&github_repos.resources, &db)?;
    let repos = github_repos
        .resources
        .iter()
        .map(|repo| Repo {
            repo,
            webhook: webhooks
                .iter()
                .find(|w| w.owner == repo.owner.login && w.name == repo.name),
        })
        .collect();

    let res = serde_json::to_string(&ReposResponse {
        repos,
        pagination: &github_repos,
    })?;

    Ok(prepare_response(&res))
}

#[derive(Deserialize)]
pub struct WebhookParams {
    owner: String,
    name: String,
}

pub async fn create_webhook(
    form: Form<WebhookParams>,
    state: AppData,
    db: Data<DBExecutor>,
    session: Session,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&db, &session)?.ok_or(Error::NotAuthedError)?;
    let access_token = current_user
        .github_access_token
        .ok_or(Error::NotAuthedError)?;

    let result = state
        .github
        .create_webhook(
            &github::ReviewRequest {
                owner: form.owner.clone(),
                name: form.name.clone(),
                id: "".to_owned(),
            },
            &state.webhook_url(),
            &access_token,
        )
        .await
        .and_then(|webhook| {
            Webhook::create(
                &NewWebhook {
                    hook_id: format!("{}", webhook.id),
                    name: form.name.clone(),
                    owner: form.owner.clone(),
                },
                &db,
            )
        })?;

    let body = serde_json::to_string(&result)?;

    Ok(prepare_response(&body))
}

pub async fn delete_webhook(
    state: AppData,
    db: Data<DBExecutor>,
    session: Session,
    path: Path<(i32,)>,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&db, &session)?.ok_or(Error::NotAuthedError)?;
    let access_token = current_user
        .github_access_token
        .ok_or(Error::NotAuthedError)?;

    match Webhook::find(path.0, &db) {
        Ok(webhook) => {
            state.github.delete_webhook(&webhook, &access_token).await?;
            webhook.delete(&db)
        }
        Err(err) => Err(err),
    }?;

    Ok(prepare_response("{ \"response\": \"ok\" }"))
}
