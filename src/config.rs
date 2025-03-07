use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Main application configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    /// Server configuration
    pub server: ServerConfig,
    
    /// Blockchain configuration
    pub blockchain: BlockchainConfig,
    
    /// Game configuration
    pub game: GameConfig,
}

/// Server configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    /// Server host
    pub host: String,
    
    /// Server port
    pub port: u16,
    
    /// Number of worker threads
    pub workers: usize,
}

/// Blockchain configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockchainConfig {
    /// Core blockchain node URL
    pub node_url: String,
    
    /// Private key for blockchain transactions
    pub private_key: String,
    
    /// Contract address for the quantum chess smart contract
    pub contract_address: String,
    
    /// Chain ID for the Core blockchain
    pub chain_id: u64,
}

/// Game configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameConfig {
    /// Minimum stake amount in Core tokens
    pub min_stake: f64,
    
    /// Maximum stake amount in Core tokens
    pub max_stake: f64,
    
    /// Default game time limit in seconds
    pub default_time_limit: u64,
    
    /// Maximum number of pieces that can be in superposition
    pub max_superposition_pieces: u8,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Server configuration
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .context("Failed to parse SERVER_PORT environment variable")?;
        let workers = env::var("SERVER_WORKERS")
            .unwrap_or_else(|_| "4".to_string())
            .parse::<usize>()
            .context("Failed to parse SERVER_WORKERS environment variable")?;

        // Blockchain configuration
        let node_url = env::var("CORE_BLOCKCHAIN_URL")
            .context("CORE_BLOCKCHAIN_URL environment variable not set")?;
        let private_key = env::var("CORE_PRIVATE_KEY")
            .context("CORE_PRIVATE_KEY environment variable not set")?;
        let contract_address = env::var("CORE_CONTRACT_ADDRESS")
            .context("CORE_CONTRACT_ADDRESS environment variable not set")?;
        let chain_id = env::var("CORE_CHAIN_ID")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u64>()
            .context("Failed to parse CORE_CHAIN_ID environment variable")?;

        // Game configuration
        let min_stake = env::var("MIN_STAKE_AMOUNT")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<f64>()
            .context("Failed to parse MIN_STAKE_AMOUNT environment variable")?;
        let max_stake = env::var("MAX_STAKE_AMOUNT")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<f64>()
            .context("Failed to parse MAX_STAKE_AMOUNT environment variable")?;
        let default_time_limit = env::var("DEFAULT_TIME_LIMIT")
            .unwrap_or_else(|_| "1800".to_string())
            .parse::<u64>()
            .context("Failed to parse DEFAULT_TIME_LIMIT environment variable")?;
        let max_superposition_pieces = env::var("MAX_SUPERPOSITION_PIECES")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<u8>()
            .context("Failed to parse MAX_SUPERPOSITION_PIECES environment variable")?;

        Ok(Self {
            server: ServerConfig {
                host,
                port,
                workers,
            },
            blockchain: BlockchainConfig {
                node_url,
                private_key,
                contract_address,
                chain_id,
            },
            game: GameConfig {
                min_stake,
                max_stake,
                default_time_limit,
                max_superposition_pieces,
            },
        })
    }
}
