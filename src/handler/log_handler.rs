use crate::models::LogResponse;
use crate::service::log_service;
use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use axum::response::IntoResponse;
use uuid::Uuid;

pub async fn list_logs() -> impl IntoResponse {
    match log_service::get_all_logs().await {
        Ok(logs) => {
            let response: Vec<LogResponse> = logs.into_iter().map(|l| l.into()).collect();
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

pub async fn get_logs_by_task(
    Path(task_id): Path<Uuid>,
) -> impl IntoResponse {
    match log_service::get_logs_by_task_id(task_id).await {
        Ok(logs) => {
            let response: Vec<LogResponse> = logs.into_iter().map(|l| l.into()).collect();
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
