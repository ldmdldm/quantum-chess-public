// Quantum Chess - Main library file
// Exports all modules required for the application

// Re-export blockchain module and submodules
pub mod blockchain {
    pub mod core;
    pub mod wallet;
    pub mod contract;
    pub mod transaction;

    pub use self::core::{CoreBlockchainClient, BlockchainClient};
    pub use self::wallet::{Wallet, KeyPair, WalletAddress};
    pub use self::contract::{SmartContract, ContractMethod};
    pub use self::transaction::{Transaction, TransactionStatus};

    // Re-export public structs for blockchain operations
    pub use self::core::{BlockchainConfig, BlockchainConnection};
    
    // Data structures
    pub struct BlockchainMove {
        pub game_id: String,
        pub player: String,
        pub piece_type: String,
        pub from_position: String,
        pub to_position: String,
        pub probability: f64,
        pub timestamp: u64,
        pub transaction_id: Option<String>,
    }

    pub struct GameStake {
        pub game_id: String,
        pub player: String,
        pub amount: u64,
        pub transaction_id: String,
        pub status: String,
        pub timestamp: u64,
    }
}

// Re-export game module
pub mod game {
    pub mod state;
    pub mod board;
    pub mod moves;
    pub mod quantum;

    pub use self::state::GameState;
}

// Re-export API module
pub mod api {
    pub mod routes;
    pub mod game;
    pub mod blockchain;
    pub mod quantum;

    pub use self::routes::configure_routes;
}

// Re-export quantum module
pub mod quantum {
    mod core;
    mod probability;
    
    pub use self::core::{QuantumState, Superposition, Entanglement};
}

// Re-export database module
pub mod db {
    pub mod models;
    pub mod schema;

    pub use self::models::{Game, Player, GameMove, QuantumState, GameStake};
}

// Re-export utility modules
pub mod config;
pub mod errors;
pub mod utils;

// Re-export common types
pub use crate::errors::{AppError, ServiceError, Result};

// Re-export blockchain client for easy access
pub use crate::blockchain::{BlockchainClient, CoreBlockchainClient, WalletAddress};

