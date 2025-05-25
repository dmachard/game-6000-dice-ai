use crate::api::models::Status;
use axum::Json;

pub async fn status_handler() -> Json<Status> {
    Json(Status {
        status: "ok".to_string(),
    })
}
