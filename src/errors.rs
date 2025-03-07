use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Service-level errors for API responses
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Internal Server Error: {0}")]
    InternalError(String),
}
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_message) = match self {
            ServiceError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ServiceError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ServiceError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ServiceError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let error_response = ErrorResponse {
            error: error_message.to_string(),
            code: status_code.as_u16(),
        };

        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
/// Enum representing all possible errors in our Quantum Chess application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Game error: {0}")]
    Game(#[from] GameError),

    #[error("Blockchain error: {0}")]
    Blockchain(#[from] BlockchainError),

    #[error("Quantum error: {0}")]
    Quantum(#[from] QuantumError),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
}
/// Game-related errors
#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid move: {0}")]
    InvalidMove(String),

    #[error("Game not found with ID: {0}")]
    GameNotFound(String),

    #[error("Not player's turn")]
    NotPlayerTurn,

    #[error("Game already ended")]
    GameAlreadyEnded,

    #[error("Invalid game state: {0}")]
    InvalidGameState(String),

    #[error("Insufficient stake amount: {0}")]
    InsufficientStake(String),
    
    #[error("Position out of bounds")]
    PositionOutOfBounds,
    
    #[error("Invalid piece selection")]
    InvalidPieceSelection,
}
/// Blockchain-related errors
#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Smart contract error: {0}")]
    SmartContractError(String),

    #[error("Wallet error: {0}")]
    WalletError(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    #[error("Block verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Nonce error: {0}")]
    NonceError(String),
    
    #[error("Gas estimation failed: {0}")]
    GasEstimationFailed(String),
}
/// Quantum mechanics simulation errors
#[derive(Error, Debug)]
pub enum QuantumError {
    #[error("Superposition error: {0}")]
    SuperpositionError(String),

    #[error("Entanglement error: {0}")]
    EntanglementError(String),

    #[error("Measurement error: {0}")]
    MeasurementError(String),

    #[error("Probability calculation error: {0}")]
    ProbabilityError(String),
    
    #[error("Quantum state collapse error: {0}")]
    StateCollapseError(String),
    
    #[error("Quantum decoherence error: {0}")]
    DecoherenceError(String),
}
/// API error response structure
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: self.to_string(),
            code: status_code.as_u16(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

/// Helper functions for creating errors
pub mod error_helpers {
    use super::*;

    pub fn unauthorized(reason: &str) -> AppError {
        AppError::Service(ServiceError::Unauthorized(reason.to_string()))
    }

    pub fn bad_request(reason: &str) -> AppError {
        AppError::Service(ServiceError::BadRequest(reason.to_string()))
    }

    pub fn not_found(reason: &str) -> AppError {
        AppError::Service(ServiceError::NotFound(reason.to_string()))
    }

    pub fn internal_error(reason: &str) -> AppError {
        AppError::Service(ServiceError::InternalError(reason.to_string()))
    }

    pub fn game_not_found(id: &str) -> AppError {
        AppError::Game(GameError::GameNotFound(id.to_string()))
    }

    pub fn invalid_move(reason: &str) -> AppError {
        AppError::Game(GameError::InvalidMove(reason.to_string()))
    }

    pub fn blockchain_connection_error(error: &str) -> AppError {
        AppError::Blockchain(BlockchainError::ConnectionError(error.to_string()))
    }

    pub fn transaction_failed(error: &str) -> AppError {
        AppError::Blockchain(BlockchainError::TransactionFailed(error.to_string()))
    }

    pub fn quantum_probability_error(error: &str) -> AppError {
        AppError::Quantum(QuantumError::ProbabilityError(error.to_string()))
    }
}

/// Implement conversion from other errors
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Server(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::Server(error.to_string())
    }
}

