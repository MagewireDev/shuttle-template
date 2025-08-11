use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::health::health;

pub fn create_router() -> Router {
    let health_router = Router::new()
        .route("/",get(health));
    
    Router::new()
        .nest("/health", health_router)
        .fallback_service(ServeDir::new("frontend/dist"))
}