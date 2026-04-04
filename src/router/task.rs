use axum::Router;
use crate::handler::task_handler;

pub fn routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(task_handler::list_tasks))
        .route("/", axum::routing::post(task_handler::create_task))
        .route("/{id}/start", axum::routing::post(task_handler::start_task))
        .route("/{id}/stop", axum::routing::post(task_handler::stop_task))
        .route("/{id}", axum::routing::delete(task_handler::delete_task))
}
