use axum::{Router, routing::{get, post, delete}, extract::Path};
use crate::handler::{task_handler, content_handler, account_handler, log_handler};

pub fn get_routes() -> Router {
    Router::new()
        // Task routes
        .route("/api/tasks", get(task_handler::list_tasks))
        .route("/api/tasks", post(task_handler::create_task))
        .route("/api/tasks/:id/start", post(task_handler::start_task))
        .route("/api/tasks/:id/stop", post(task_handler::stop_task))
        .route("/api/tasks/:id", delete(task_handler::delete_task))
        // Content routes
        .route("/api/contents", get(content_handler::list_contents))
        .route("/api/contents/:id", get(content_handler::get_content))
        .route("/api/contents/:id/review", post(content_handler::review_content))
        // Account routes
        .route("/api/accounts", get(account_handler::list_accounts))
        .route("/api/accounts", post(account_handler::create_account))
        .route("/api/accounts/:id", delete(account_handler::delete_account))
        // Log routes
        .route("/api/logs", get(log_handler::list_logs))
        .route("/api/logs/:task_id", get(log_handler::get_logs_by_task))
}
