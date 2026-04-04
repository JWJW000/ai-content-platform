use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub platform: String, // "xiaohongshu" | "wechat"
    pub username: String,
    pub auth: String, // cookie/token (should be encrypted in production)
    pub status: String, // "active" | "inactive"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub platform: String,
    pub username: String,
    pub auth: String,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub platform: String,
    pub username: String,
    pub status: String,
}

impl From<Account> for AccountResponse {
    fn from(a: Account) -> Self {
        Self {
            id: a.id,
            platform: a.platform,
            username: a.username,
            status: a.status,
        }
    }
}
