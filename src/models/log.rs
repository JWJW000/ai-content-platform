use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Log {
    pub id: Uuid,
    pub task_id: Option<Uuid>,
    pub level: String, // "info" | "error" | "warn"
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LogResponse {
    pub id: Uuid,
    pub task_id: Option<Uuid>,
    pub level: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

impl From<Log> for LogResponse {
    fn from(l: Log) -> Self {
        Self {
            id: l.id,
            task_id: l.task_id,
            level: l.level,
            message: l.message,
            created_at: l.created_at,
        }
    }
}
