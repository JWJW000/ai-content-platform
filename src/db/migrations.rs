use crate::db::postgres::get_pool;
use anyhow::Result;

pub async fn run_migrations() -> Result<()> {
    let pool = get_pool();
    
    // 创建 tasks 表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id UUID PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            platform VARCHAR(50) NOT NULL,
            prompt TEXT NOT NULL,
            schedule VARCHAR(100) NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'stopped',
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // 创建 contents 表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS contents (
            id UUID PRIMARY KEY,
            task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            title VARCHAR(500) NOT NULL,
            body TEXT NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'pending_review',
            score FLOAT8,
            review_note TEXT,
            created_at TIMESTAMPTZ NOT NULL,
            reviewed_at TIMESTAMPTZ
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // 创建 accounts 表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id UUID PRIMARY KEY,
            platform VARCHAR(50) NOT NULL,
            username VARCHAR(255) NOT NULL,
            auth TEXT NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // 创建 logs 表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS logs (
            id UUID PRIMARY KEY,
            task_id UUID REFERENCES tasks(id) ON DELETE SET NULL,
            level VARCHAR(20) NOT NULL,
            message TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // 创建 executions 表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS executions (
            id UUID PRIMARY KEY,
            task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            status VARCHAR(20) NOT NULL,
            started_at TIMESTAMPTZ NOT NULL,
            finished_at TIMESTAMPTZ
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    tracing::info!("Database migrations completed");
    Ok(())
}
