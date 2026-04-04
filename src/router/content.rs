use axum::Router;
use crate::handler::content_handler;

pub fn routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(content_handler::list_contents))
        .route("/{id}", axum::routing::get(content_handler::get_content))
}
