use::axum::response::Json;
use serde::Serialize;


#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn health() -> Json<HealthResponse>{
    Json(HealthResponse {
        status: "Server is healthy"
    })
}
