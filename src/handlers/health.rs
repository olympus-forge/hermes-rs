use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}
