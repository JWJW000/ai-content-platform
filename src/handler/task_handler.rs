use crate::models::{CreateTaskRequest, TaskResponse};
use crate::service::task_service;
use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use axum::response::IntoResponse;
use uuid::Uuid;

pub async fn list_tasks() -> impl IntoResponse {
    match task_service::get_all_tasks().await {
        Ok(tasks) => {
            let response: Vec<TaskResponse> = tasks.into_iter().map(|t| t.into()).collect();
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

pub async fn create_task(
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    match task_service::create_task(payload).await {
        Ok(task) => (StatusCode::CREATED, Json(serde_json::json!({
            "code": 0,
            "data": TaskResponse::from(task),
            "message": "success"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

pub async fn start_task(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match task_service::start_task(id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": null,
            "message": "Task started"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

pub async fn stop_task(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match task_service::stop_task(id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": null,
            "message": "Task stopped"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

pub async fn delete_task(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match task_service::delete_task(id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": null,
            "message": "Task deleted"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}
