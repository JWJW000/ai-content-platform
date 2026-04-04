use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub platform: String, // "xiaohongshu" | "wechat"
    pub prompt: String,
    pub schedule: String, // cron expression
    pub status: String,   // "running" | "stopped"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub platform: String,
    pub prompt: String,
    pub schedule: String,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub name: String,
    pub platform: String,
    pub status: String,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            name: task.name,
            platform: task.platform,
            status: task.status,
        }
    }
}
