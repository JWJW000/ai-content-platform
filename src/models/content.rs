use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Content {
    pub id: Uuid,
    pub task_id: Uuid,
    pub title: String,
    pub body: String,
    pub status: String, // "generated" | "pending_review" | "approved" | "rejected" | "published" | "failed"
    pub score: Option<f64>,
    pub review_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ContentResponse {
    pub id: Uuid,
    pub task_id: Uuid,
    pub title: String,
    pub body: String,
    pub status: String,
    pub score: Option<f64>,
    pub review_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

impl From<Content> for ContentResponse {
    fn from(c: Content) -> Self {
        Self {
            id: c.id,
            task_id: c.task_id,
            title: c.title,
            body: c.body,
            status: c.status,
            score: c.score,
            review_note: c.review_note,
            created_at: c.created_at,
            reviewed_at: c.reviewed_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReviewContentRequest {
    pub approved: bool,
    pub note: Option<String>,
}
