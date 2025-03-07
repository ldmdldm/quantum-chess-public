use actix_web::{web, HttpResponse, Scope, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashMap;

use crate::blockchain::core::{CoreBlockchain, BlockchainConfig, StakeReceipt, UnstakeReceipt, VerificationResult};
use crate::blockchain::{Transaction, TransactionStatus};
use crate::config::AppConfig;
use crate::errors::ServiceError;
use crate::game::state::GameState;
use std::sync::Arc;
use tokio::sync::RwLock;

// Request and response data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct StakeRequest {
    pub game_id: Uuid,
    pub amount: u64,
    pub player_address: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StakeResponse {
    pub transaction_id: String,
    pub status: String,
    pub game_id: Uuid,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveVerificationRequest {
    pub game_id: Uuid,
    pub move_notation: String,
    pub player_address: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTransactionRequest {
    pub transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub transaction_id: String,
    pub status: String,
    pub block_number: Option<u64>,
    pub timestamp: Option<u64>,
    pub confirmation_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStakeInfo {
    pub game_id: Uuid,
    pub total_stake: u64,
    pub white_stake: u64,
    pub black_stake: u64,
    pub contract_address: String,
}

#[derive(Debug, Serialize)]
pub struct BlockchainError {
    message: String,
}

impl BlockchainError {
    fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

/// Configure blockchain API routes
pub fn configure() -> Scope {
    web::scope("/blockchain")
        .route("/stake", web::post().to(stake_funds))
        .route("/unstake", web::post().to(unstake_funds))
        .route("/verify_move", web::post().to(verify_move_on_blockchain))
        .route("/verify-transaction", web::post().to(verify_transaction))
        .route("/transaction/{tx_id}", web::get().to(get_transaction_status))
        .route("/game_stakes/{game_id}", web::get().to(get_game_stake_info))
        .route("/status", web::get().to(get_blockchain_status))
}

/// Stake funds for a game
async fn stake_funds(
    config: web::Data<AppConfig>,
    blockchain: web::Data<CoreBlockchain>,
    stake_req: web::Json<StakeRequest>,
) -> Result<HttpResponse, ServiceError> {
    let stake_req = stake_req.into_inner();
    
    // Verify signature matches player_address
    let verification = blockchain.verify_signature(
        &stake_req.game_id.to_string(), 
        &stake_req.signature, 
        &stake_req.player_address
    ).await.map_err(|e| ServiceError::InternalError(format!("Signature verification failed: {}", e)))?;
    
    if !verification.is_valid {
        return Err(ServiceError::Unauthorized("Invalid signature".into()));
    }
    
    // Create stake transaction on the blockchain
    let result = blockchain.stake_funds(
        &stake_req.game_id.to_string(),
        stake_req.amount,
    ).await.map_err(|e| ServiceError::InternalError(format!("Stake transaction failed: {}", e)))?;
    
    // Return transaction details
    Ok(HttpResponse::Ok().json(StakeResponse {
        transaction_id: result.transaction_hash,
        status: "pending".to_string(),
        game_id: stake_req.game_id,
        amount: stake_req.amount,
    }))
}

/// Unstake funds from a game
async fn unstake_funds(
    config: web::Data<AppConfig>,
    blockchain: web::Data<CoreBlockchain>,
    stake_req: web::Json<StakeRequest>,
) -> Result<HttpResponse, ServiceError> {
    let stake_req = stake_req.into_inner();
    
    // Verify signature matches player_address
    let verification = blockchain.verify_signature(
        &stake_req.game_id.to_string(), 
        &stake_req.signature, 
        &stake_req.player_address
    ).await.map_err(|e| ServiceError::InternalError(format!("Signature verification failed: {}", e)))?;
    
    if !verification.is_valid {
        return Err(ServiceError::Unauthorized("Invalid signature".into()));
    }
    
    // Create unstake transaction on the blockchain
    let result = blockchain.unstake_funds(
        &stake_req.game_id.to_string(),
        stake_req.amount,
    ).await.map_err(|e| ServiceError::InternalError(format!("Unstake transaction failed: {}", e)))?;
    
    // Return transaction details
    Ok(HttpResponse::Ok().json(StakeResponse {
        transaction_id: result.transaction_hash,
        status: "pending".to_string(),
        game_id: stake_req.game_id,
        amount: stake_req.amount,
    }))
}

/// Verify a move on the blockchain
async fn verify_move_on_blockchain(
    config: web::Data<AppConfig>,
    blockchain: web::Data<CoreBlockchain>,
    game_state: web::Data<Arc<RwLock<HashMap<Uuid, GameState>>>>,
    req: web::Json<MoveVerificationRequest>,
) -> Result<HttpResponse, ServiceError> {
    let req = req.into_inner();
    
    // Verify signature matches player_address
    let verification = blockchain.verify_signature(
        &req.game_id.to_string(), 
        &req.signature, 
        &req.player_address
    ).await.map_err(|e| ServiceError::InternalError(format!("Signature verification failed: {}", e)))?;
    
    if !verification.is_valid {
        return Err(ServiceError::Unauthorized("Invalid signature".into()));
    }
    
    // Get the game from the state map
    let games = game_state.read().await;
    let game = games.get(&req.game_id)
        .ok_or_else(|| ServiceError::NotFound(format!("Game not found: {}", req.game_id)))?;
    
    // Check if the player is part of the game
    let is_white_player = match &game.white_player {
        Some(player_info) => player_info.player.wallet_address == req.player_address,
        None => false
    };
    
    let is_black_player = match &game.black_player {
        Some(player_info) => player_info.player.wallet_address == req.player_address,
        None => false
    };
    
    if !is_white_player && !is_black_player {
        return Err(ServiceError::Unauthorized("Player not part of this game".into()));
    }
    
    drop(games); // Release the read lock
    
    // Record move on blockchain - use 0.9 as a default probability if not calculating 
    let tx_id = blockchain.record_move(
        &req.game_id.to_string(),
        &req.move_notation,
        0.9, // Default probability
        "a1", // Default from position
        "a2"  // Default to position
    ).await.map_err(|e| ServiceError::InternalError(format!("Failed to record move: {}", e)))?;
    
    Ok(HttpResponse::Ok().json(TransactionResponse {
        transaction_id: tx_id,
        status: "pending".to_string(),
        block_number: None,
        timestamp: None,
        confirmation_count: 0,
    }))
}

/// Get transaction status from the blockchain
async fn get_transaction_status(
    blockchain: web::Data<CoreBlockchain>,
    path: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let tx_id = path.into_inner();
    
    // Get transaction status from blockchain
    let tx_details = blockchain.get_transaction_details(&tx_id).await
        .map_err(|e| ServiceError::InternalError(format!("Failed to get transaction: {}", e)))?;
    
    // Parse the transaction details to get the status and other info
    let block_number: Option<u64> = tx_details["block_number"].as_u64();
    let timestamp: Option<u64> = tx_details["timestamp"].as_u64();
    let status = tx_details["status"].as_str().unwrap_or("unknown").to_string();
    
    let response = TransactionResponse {
        transaction_id: tx_id,
        status,
        block_number,
        timestamp,
        confirmation_count: 1, // Default value since our implementation doesn't track this
    };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Get stake information for a game
async fn get_game_stake_info(
    blockchain: web::Data<CoreBlockchain>,
    game_state: web::Data<GameState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let game_id = path.into_inner();
    
    // Get game from database
    let game = game_state.get_game(game_id).await?;
    
    // Get stake information from blockchain
    let stake_info = blockchain.get_game_stake_info(game_id).await?;
    
    let response = GameStakeInfo {
        game_id,
        total_stake: stake_info.total_stake,
        white_stake: stake_info.white_stake,
        black_stake: stake_info.black_stake,
        contract_address: stake_info.contract_address,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Verify a transaction on the blockchain
async fn verify_transaction(
    blockchain: web::Data<CoreBlockchain>,
    req: web::Json<VerifyTransactionRequest>,
) -> Result<HttpResponse, ServiceError> {
    let req = req.into_inner();
    
    // Verify the transaction on the blockchain
    match blockchain.verify_transaction(&req.transaction_hash).await {
        Ok(is_verified) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "verified": is_verified,
                "transaction_hash": req.transaction_hash
            })))
        },
        Err(e) => {
            Err(ServiceError::InternalError(format!("Failed to verify transaction: {}", e)))
        }
    }
}

/// Get blockchain status
async fn get_blockchain_status(
    blockchain: web::Data<CoreBlockchain>,
) -> Result<HttpResponse, ServiceError> {
    match blockchain.get_blockchain_status().await {
        Ok(status) => {
            Ok(HttpResponse::Ok().json(status))
        },
        Err(e) => {
            Err(ServiceError::InternalError(format!("Failed to get blockchain status: {}", e)))
        }
    }
}
