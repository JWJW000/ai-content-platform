use crate::models::{AccountResponse, CreateAccountRequest, LoginResponse};
use crate::service::account_service::{self, CreateAccountResponse};
use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use axum::response::IntoResponse;
use uuid::Uuid;

pub async fn list_accounts() -> impl IntoResponse {
    match account_service::get_all_accounts().await {
        Ok(accounts) => {
            let response: Vec<AccountResponse> = accounts.into_iter().map(|a| a.into()).collect();
            (StatusCode::OK, Json(serde_json::json!({
                "code": 0,
                "data": response,
                "message": "success"
            }))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

pub async fn create_account(
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service::create_account(payload).await {
        Ok(result) => {
            let response = serde_json::json!({
                "account": AccountResponse::from(result.account),
                "login_triggered": result.login_triggered,
                "message": result.message
            });
            (StatusCode::CREATED, Json(serde_json::json!({
                "code": 0,
                "data": response,
                "message": "success"
            }))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

pub async fn delete_account(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match account_service::delete_account(id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": null,
            "message": "Account deleted"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

/// 登录账号（触发浏览器进行扫码登录）
pub async fn login_account(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match account_service::login_account(id).await {
        Ok(response) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": response,
            "message": "success"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

/// 验证账号 Cookie 是否有效
pub async fn verify_account(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match account_service::verify_account(id).await {
        Ok(valid) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": { "valid": valid },
            "message": if valid { "valid" } else { "invalid" }
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}
