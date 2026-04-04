use crate::models::{ContentResponse, ReviewContentRequest};
use crate::service::content_service;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ListContentsQuery {
    task_id: Option<Uuid>,
    status: Option<String>,
}

pub async fn list_contents(
    Query(query): Query<ListContentsQuery>,
) -> impl IntoResponse {
    match content_service::get_contents(query.task_id).await {
        Ok(contents) => {
            let contents = if let Some(status) = query.status {
                contents.into_iter().filter(|c| c.status == status).collect()
            } else {
                contents
            };
            let response: Vec<ContentResponse> = contents.into_iter().map(|c| c.into()).collect();
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

pub async fn get_content(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match content_service::get_content_by_id(id).await {
        Ok(Some(content)) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": ContentResponse::from(content),
            "message": "success"
        }))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "code": 404,
            "data": null,
            "message": "Content not found"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}

pub async fn review_content(
    Path(id): Path<Uuid>,
    Json(payload): Json<ReviewContentRequest>,
) -> impl IntoResponse {
    match content_service::review_content(id, payload).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({
            "code": 0,
            "data": null,
            "message": "Content reviewed"
        }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "code": 500,
            "data": null,
            "message": e.to_string()
        }))).into_response(),
    }
}
