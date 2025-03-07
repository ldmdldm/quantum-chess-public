use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use anyhow::{Context, Result};
use dotenv::dotenv;
use log::{info, error};
use std::sync::Arc;
use tokio::sync::Mutex;

// Module declarations
mod api;
mod blockchain;
mod game;
mod quantum;
mod config;
mod errors;
mod utils;

use blockchain::core::CoreBlockchain;
use config::AppConfig;
use game::state::GameState;

/// Application state accessible across all routes
pub struct AppState {
    config: AppConfig,
    blockchain: Arc<Mutex<CoreBlockchain>>,
    game_state: Arc<Mutex<GameState>>,
}

/// The main entry point for the Quantum Chess application
#[actix_web::main]
async fn main() -> Result<()> {
    // Initialize environment variables from .env file
    dotenv().ok();
    
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    info!("Starting Quantum Chess application");
    
    // Load configuration
    let config = AppConfig::from_env().context("Failed to load configuration")?;
    info!("Configuration loaded successfully");
    
    // Initialize Core blockchain connection
    let blockchain = match CoreBlockchain::new(&config.blockchain).await {
        Ok(blockchain) => {
            info!("Successfully connected to Core blockchain at {}", config.blockchain.node_url);
            Arc::new(Mutex::new(blockchain))
        },
        Err(e) => {
            error!("Failed to connect to Core blockchain: {}", e);
            return Err(anyhow::anyhow!("Blockchain connection failed").context(e));
        }
    };
    
    // Initialize game state
    let game_state = Arc::new(Mutex::new(GameState::new()));
    info!("Game state initialized");
    
    // Create shared application state
    let app_state = web::Data::new(AppState {
        config: config.clone(),
        blockchain: blockchain.clone(),
        game_state: game_state.clone(),
    });
    
    // Start the HTTP server
    info!("Starting web server on {}:{}", config.server.host, config.server.port);
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            // Register application state
            .app_data(app_state.clone())
            // Add middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            // Register API routes
            .service(
                web::scope("/api")
                    // Game routes
                    .service(api::game::configure())
                    // Blockchain routes
                    .service(api::blockchain::configure())
                    // Quantum simulation routes
                    .service(api::quantum::configure())
            )
            // Health check endpoint
            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("Quantum Chess is running") }))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .workers(config.server.workers)
    .run()
    .await
    .context("Server error")?;
    
    info!("Quantum Chess application stopped");
    Ok(())
}
