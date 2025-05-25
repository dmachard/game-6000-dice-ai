use crate::api::handlers;
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new().route("/status", get(handlers::status_handler))
}
