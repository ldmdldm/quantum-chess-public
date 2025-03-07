mod core;
mod wallet;
mod contract;
mod transaction;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub use self::core::{CoreBlockchainClient, BlockchainConfig};
pub use self::wallet::{Wallet, KeyPair};
pub use self::contract::{SmartContract, ContractMethod};
pub use self::transaction::{Transaction, TransactionStatus};

/// Represents a stake in the Quantum Chess game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStake {
    /// The amount of tokens staked
    pub amount: u64,
    /// The account that placed the stake
    pub account: String,
    /// The player's wallet address (may be same as account)
    pub player: String,
    /// The game ID this stake is for
    pub game_id: String,
    /// The transaction ID on the blockchain
    pub transaction_id: String,
    /// The status of the stake (pending, confirmed, paid, etc.)
    pub status: String,
    /// Timestamp when stake was created
    pub created_at: u64,
    /// Timestamp when stake was last updated
    pub updated_at: Option<u64>,
}

/// Represents a move recorded on the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainMove {
    /// The game ID
    pub game_id: String,
    /// The player's account
    pub player: String,
    /// The move in algebraic notation
    pub move_notation: String,
    /// The starting position of the move
    pub from_position: String,
    /// The destination position of the move
    pub to_position: String,
    /// The quantum probability used
    pub probability: f64,
    /// The resulting position hash
    pub position_hash: String,
    /// The timestamp of the move
    pub timestamp: u64,
    /// The blockchain transaction ID (if available)
    pub transaction_id: Option<String>,
}

/// Interface for blockchain implementations
pub trait BlockchainClient {
    /// Connect to the blockchain network
    fn connect(&mut self) -> Result<()>;
    
    /// Check if connected to the blockchain
    fn is_connected(&self) -> bool;
    
    /// Get the current account balance
    fn get_balance(&self, address: &str) -> Result<u64>;
    
    /// Create a new game on the blockchain
    fn create_game(&self, stake_amount: u64) -> Result<String>;
    
    /// Join an existing game
    fn join_game(&self, game_id: &str, stake_amount: u64) -> Result<()>;
    
    /// Record a move on the blockchain
    fn record_move(&self, game_move: BlockchainMove) -> Result<String>;
    
    /// Verify a move's authenticity
    fn verify_move(&self, transaction_id: &str) -> Result<BlockchainMove>;
    
    /// Get all moves for a game
    fn get_game_moves(&self, game_id: &str) -> Result<Vec<BlockchainMove>>;
    
    /// Get stake information for a game
    fn get_game_stakes(&self, game_id: &str) -> Result<Vec<GameStake>>;
    
    /// Finalize a game and distribute rewards
    fn finalize_game(&self, game_id: &str, winner: &str) -> Result<String>;
    
    /// Initialize a wallet from a key file
    fn init_wallet(&mut self, key_path: &str) -> Result<()>;
    
    /// Get the wallet address
    fn get_address(&self) -> Result<String>;
    
    /// Initialize a smart contract with a specific address
    fn init_contract(&mut self, contract_address: &str) -> Result<()>;
    
    /// Deploy a smart contract with the given bytecode
    fn deploy_contract(&self, bytecode: &[u8], constructor_args: &[&str], value: u64) -> Result<String>;
    
    /// Estimate gas required for a transaction
    fn estimate_gas(&self, to: &str, data: &[u8], value: u64) -> Result<u64>;
    
    /// Send a transaction to the blockchain
    fn send_transaction(&self, to: &str, data: &[u8], value: u64) -> Result<String>;
    
    /// Wait for a transaction to be confirmed
    fn wait_for_transaction(&self, transaction_hash: &str, confirmations: u64) -> Result<bool>;
    
    /// Get transaction details
    fn get_transaction(&self, transaction_hash: &str) -> Result<Option<Transaction>>;
    
    /// Get the latest block number
    fn get_block_number(&self) -> Result<u64>;
    
    /// Call a read-only contract method
    fn call_contract_method(&self, method: &str, args: &[&str]) -> Result<String>;
}

