use axum::Router;
use crate::handler::log_handler;

pub fn routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(log_handler::list_logs))
        .route("/{task_id}", axum::routing::get(log_handler::get_logs_by_task))
}
