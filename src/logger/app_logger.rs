use crate::db::postgres::get_pool;
use crate::models::Log;
use anyhow::Result;
use uuid::Uuid;

/// 记录信息日志
pub async fn log_info(task_id: Option<Uuid>, message: &str) -> Result<()> {
    let log = Log {
        id: Uuid::new_v4(),
        task_id,
        level: "info".to_string(),
        message: message.to_string(),
        created_at: chrono::Utc::now(),
    };
    
    // 写入数据库
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
    
    // 同时输出到控制台
    tracing::info!("[{}] {}", task_id.map(|id| id.to_string()).unwrap_or_else(|| "SYSTEM".to_string()), message);
    
    Ok(())
}

/// 记录错误日志
pub async fn log_error(task_id: Option<Uuid>, message: &str) -> Result<()> {
    let log = Log {
        id: Uuid::new_v4(),
        task_id,
        level: "error".to_string(),
        message: message.to_string(),
        created_at: chrono::Utc::now(),
    };
    
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
    
    tracing::error!("[{}] {}", task_id.map(|id| id.to_string()).unwrap_or_else(|| "SYSTEM".to_string()), message);
    
    Ok(())
}

/// 记录警告日志
pub async fn log_warn(task_id: Option<Uuid>, message: &str) -> Result<()> {
    let log = Log {
        id: Uuid::new_v4(),
        task_id,
        level: "warn".to_string(),
        message: message.to_string(),
        created_at: chrono::Utc::now(),
    };
    
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
    
    tracing::warn!("[{}] {}", task_id.map(|id| id.to_string()).unwrap_or_else(|| "SYSTEM".to_string()), message);
    
    Ok(())
}
