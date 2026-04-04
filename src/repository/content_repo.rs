use crate::db::postgres::get_pool;
use crate::models::Content;
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_all_contents() -> Result<Vec<Content>> {
    let pool = get_pool();
    let contents = sqlx::query_as::<_, Content>(
        "SELECT id, task_id, title, body, status, score, review_note, created_at, reviewed_at FROM contents ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(contents)
}

pub async fn get_contents_by_task_id(task_id: Uuid) -> Result<Vec<Content>> {
    let pool = get_pool();
    let contents = sqlx::query_as::<_, Content>(
        "SELECT id, task_id, title, body, status, score, review_note, created_at, reviewed_at FROM contents WHERE task_id = $1 ORDER BY created_at DESC"
    )
    .bind(task_id)
    .fetch_all(pool)
    .await?;
    Ok(contents)
}

pub async fn get_content_by_id(id: Uuid) -> Result<Option<Content>> {
    let pool = get_pool();
    let content = sqlx::query_as::<_, Content>(
        "SELECT id, task_id, title, body, status, score, review_note, created_at, reviewed_at FROM contents WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(content)
}

pub async fn insert_content(content: &Content) -> Result<()> {
    let pool = get_pool();
    sqlx::query(
        "INSERT INTO contents (id, task_id, title, body, status, score, review_note, created_at, reviewed_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(content.id)
    .bind(content.task_id)
    .bind(&content.title)
    .bind(&content.body)
    .bind(&content.status)
    .bind(content.score)
    .bind(&content.review_note)
    .bind(content.created_at)
    .bind(content.reviewed_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_content_status(id: Uuid, status: &str) -> Result<()> {
    let pool = get_pool();
    sqlx::query("UPDATE contents SET status = $1 WHERE id = $2")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn review_content(id: Uuid, approved: bool, note: Option<String>) -> Result<()> {
    let pool = get_pool();
    let status = if approved { "approved" } else { "rejected" };
    sqlx::query("UPDATE contents SET status = $1, review_note = $2, reviewed_at = NOW() WHERE id = $3")
        .bind(status)
        .bind(&note)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
