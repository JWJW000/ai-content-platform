use crate::models::{Content, ReviewContentRequest};
use crate::repository::content_repo;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_contents(task_id: Option<Uuid>) -> Result<Vec<Content>> {
    match task_id {
        Some(id) => content_repo::get_contents_by_task_id(id).await,
        None => content_repo::get_all_contents().await,
    }
}

pub async fn get_content_by_id(id: Uuid) -> Result<Option<Content>> {
    content_repo::get_content_by_id(id).await
}

pub async fn update_content_status(id: Uuid, status: &str) -> Result<()> {
    content_repo::update_content_status(id, status).await
}

pub async fn review_content(id: Uuid, request: ReviewContentRequest) -> Result<()> {
    content_repo::review_content(id, request.approved, request.note).await
}

pub async fn get_pending_review_contents() -> Result<Vec<Content>> {
    let all = content_repo::get_all_contents().await?;
    Ok(all.into_iter().filter(|c| c.status == "pending_review").collect())
}

pub async fn get_approved_contents() -> Result<Vec<Content>> {
    let all = content_repo::get_all_contents().await?;
    Ok(all.into_iter().filter(|c| c.status == "approved").collect())
}
