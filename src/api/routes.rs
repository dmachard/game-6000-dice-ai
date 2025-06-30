use crate::api::handlers::{self, GameStore};
use crate::config::Config;
use axum::{
    Router,
    routing::{post},
    middleware,
    http::HeaderValue,
    Extension,
};
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Mutex;


pub fn create_router(config: Arc<Config>) -> Router {
    let game_store: GameStore = Arc::new(Mutex::new(HashMap::new()));

    Router::new()
        .route("/api/game", post(handlers::create_game))
        .route("/api/game/{game_id}/roll", post(handlers::roll_dice_handler))
        .route("/api/game/{game_id}/bank", post(handlers::bank_points_handler))
        .route("/api/game/{game_id}/status", post(handlers::game_status_handler))
        .route("/api/game/{game_id}/next", post(handlers::next_player_handler))
        .with_state(game_store)
        .layer(Extension(config))
        .layer(middleware::from_fn(cors_middleware))
}


async fn cors_middleware(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    
    headers.insert(
        "Access-Control-Allow-Origin", 
        HeaderValue::from_static("*")
    );
    headers.insert(
        "Access-Control-Allow-Methods", 
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS")
    );
    headers.insert(
        "Access-Control-Allow-Headers", 
        HeaderValue::from_static("Content-Type, Authorization")
    );
    
    response
}