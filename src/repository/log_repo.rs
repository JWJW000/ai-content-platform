use crate::db::postgres::get_pool;
use crate::models::Log;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_all_logs() -> Result<Vec<Log>> {
    let pool = get_pool();
    let logs = sqlx::query_as::<_, Log>(
        "SELECT id, task_id, level, message, created_at FROM logs ORDER BY created_at DESC LIMIT 100"
    )
    .fetch_all(pool)
    .await?;
    Ok(logs)
}

pub async fn get_logs_by_task_id(task_id: Uuid) -> Result<Vec<Log>> {
    let pool = get_pool();
    let logs = sqlx::query_as::<_, Log>(
        "SELECT id, task_id, level, message, created_at FROM logs WHERE task_id = $1 ORDER BY created_at DESC LIMIT 100"
    )
    .bind(task_id)
    .fetch_all(pool)
    .await?;
    Ok(logs)
}

pub async fn insert_log(log: &Log) -> Result<()> {
    let pool = get_pool();
    sqlx::query(
        "INSERT INTO logs (id, task_id, level, message, created_at) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(log.id)
    .bind(log.task_id)
    .bind(&log.level)
    .bind(&log.message)
    .bind(log.created_at)
    .execute(pool)
    .await?;
    Ok(())
}
