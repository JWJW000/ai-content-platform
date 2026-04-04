use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Execution {
    pub id: Uuid,
    pub task_id: Uuid,
    pub status: String, // "running" | "success" | "failed"
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ExecutionResponse {
    pub id: Uuid,
    pub task_id: Uuid,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl From<Execution> for ExecutionResponse {
    fn from(e: Execution) -> Self {
        Self {
            id: e.id,
            task_id: e.task_id,
            status: e.status,
            started_at: e.started_at,
            finished_at: e.finished_at,
        }
    }
}
