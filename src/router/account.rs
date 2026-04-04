use axum::Router;
use crate::handler::account_handler;

pub fn routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(account_handler::list_accounts))
        .route("/", axum::routing::post(account_handler::create_account))
        .route("/{id}", axum::routing::delete(account_handler::delete_account))
}
