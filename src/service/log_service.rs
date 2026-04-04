use crate::models::Log;
use crate::repository::log_repo;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_all_logs() -> Result<Vec<Log>> {
    log_repo::get_all_logs().await
}

pub async fn get_logs_by_task_id(task_id: Uuid) -> Result<Vec<Log>> {
    log_repo::get_logs_by_task_id(task_id).await
}
