use crate::db::postgres::get_pool;
use crate::models::Task;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_all_tasks() -> Result<Vec<Task>> {
    let pool = get_pool();
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, name, platform, prompt, schedule, status, created_at, updated_at FROM tasks ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}

pub async fn get_task_by_id(id: Uuid) -> Result<Option<Task>> {
    let pool = get_pool();
    let task = sqlx::query_as::<_, Task>(
        "SELECT id, name, platform, prompt, schedule, status, created_at, updated_at FROM tasks WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(task)
}

pub async fn insert_task(task: &Task) -> Result<()> {
    let pool = get_pool();
    sqlx::query(
        "INSERT INTO tasks (id, name, platform, prompt, schedule, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(task.id)
    .bind(&task.name)
    .bind(&task.platform)
    .bind(&task.prompt)
    .bind(&task.schedule)
    .bind(&task.status)
    .bind(task.created_at)
    .bind(task.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_task_status(id: Uuid, status: &str) -> Result<()> {
    let pool = get_pool();
    sqlx::query("UPDATE tasks SET status = $1, updated_at = NOW() WHERE id = $2")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_task(id: Uuid) -> Result<()> {
    let pool = get_pool();
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_running_tasks() -> Result<Vec<Task>> {
    let pool = get_pool();
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, name, platform, prompt, schedule, status, created_at, updated_at FROM tasks WHERE status = 'running'"
    )
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}
