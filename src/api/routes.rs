use actix_web::{web, HttpResponse, Scope};

use crate::api::blockchain;
use crate::api::game;
use crate::api::quantum;

/// Configures and returns all API routes for the application
pub fn configure() -> Scope {
    web::scope("/api")
        .service(health_check)
        .service(web::scope("/games").configure(game::configure))
        .service(web::scope("/blockchain").configure(blockchain::configure))
        .service(web::scope("/quantum").configure(quantum::configure))
}

/// Simple health check endpoint
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Register all API routes with the application
pub fn register(config: &mut web::ServiceConfig) {
    config.service(configure());
}

