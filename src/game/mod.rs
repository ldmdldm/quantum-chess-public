mod state;
mod board;
mod moves;
mod quantum;
mod rules;

pub use state::{GameState, GameStatus, Player, PlayerInfo};
pub use board::{Board, Position, Piece, PieceType};
pub use moves::{Move, MoveResult, MoveType, ProbabilityZone};
pub use quantum::{QuantumState, Superposition, Entanglement};
pub use rules::{QuantumRules, QuantumEffect, EntanglementRule, SuperpositionRule};

/// Game module error types
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Invalid move: {0}")]
    InvalidMove(String),
    
    #[error("Game already ended")]
    GameAlreadyEnded,
    
    #[error("Not player's turn")]
    NotPlayerTurn,
    
    #[error("Insufficient stake")]
    InsufficientStake,
    
    #[error("Invalid quantum operation: {0}")]
    InvalidQuantumOperation(String),
    
    #[error("Blockchain error: {0}")]
    BlockchainError(String),
}

pub type Result<T> = std::result::Result<T, GameError>;

