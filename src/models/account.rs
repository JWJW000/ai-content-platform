use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub platform: String, // "xiaohongshu" | "douyin"
    pub username: String,
    /// Cookie 文件路径（用于 rust_drission）
    pub cookie_path: Option<String>,
    /// 旧的 auth 字段，保留用于兼容
    pub auth: String, 
    pub status: String, // "active" | "inactive" | "pending_login"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub platform: String,
    pub username: String,
    /// 创建时可选提供 auth（cookie 路径或直接 cookie）
    pub auth: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub platform: String,
    pub username: String,
    pub status: String,
    pub cookie_path: Option<String>,
}

impl From<Account> for AccountResponse {
    fn from(a: Account) -> Self {
        Self {
            id: a.id,
            platform: a.platform,
            username: a.username,
            status: a.status,
            cookie_path: a.cookie_path,
        }
    }
}

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub account_id: Uuid,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub qrcode_base64: Option<String>,
}
