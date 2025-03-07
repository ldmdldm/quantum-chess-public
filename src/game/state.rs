use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chess::{Board as ChessBoard, ChessMove, Color, Piece as ChessPiece};

use crate::blockchain::WalletAddress;

/// Represents the current status of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    /// Game is waiting for players to join
    Waiting,
    /// Game is in progress
    InProgress,
    /// Game has ended with a winner
    Completed,
    /// Game ended in a draw
    Draw,
    /// Game was abandoned
    Abandoned,
}

/// Represents a player in the quantum chess game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    /// Blockchain wallet address of the player
    pub wallet_address: WalletAddress,
    /// Color assigned to the player (White or Black)
    pub color: Color,
    /// Current stake amount in Core tokens
    pub stake_amount: u64,
    /// Timestamp when player joined the game
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

/// Additional player information tracked during the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    /// Reference to the player
    pub player: Player,
    /// Number of pieces captured
    pub captures: u8,
    /// Number of quantum moves performed
    pub quantum_moves: u8,
    /// Number of classical moves performed
    pub classical_moves: u8,
    /// Current probability bonus (based on stake and game performance)
    pub probability_bonus: f64,
    /// Pieces in superposition state
    pub superpositions: u8,
    /// Pieces in entanglement
    pub entanglements: u8,
}

/// Represents the quantum state of a piece on the board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumPieceState {
    /// The classical piece type
    pub piece: ChessPiece,
    /// Primary position on the board (where the piece is physically displayed)
    pub primary_position: chess::Square,
    /// Secondary positions with associated probabilities
    pub superpositions: HashMap<chess::Square, f64>,
    /// Entangled pieces (pieces whose states are linked)
    pub entangled_with: Vec<chess::Square>,
    /// Probability of measurement collapsing to primary position
    pub measurement_probability: f64,
}

/// Tracks the history of quantum operations in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumOperationHistory {
    /// Type of quantum operation performed
    pub operation_type: String,
    /// Affected positions
    pub positions: Vec<chess::Square>,
    /// Player who performed the operation
    pub player: WalletAddress,
    /// Timestamp of the operation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Blockchain transaction ID for verification
    pub transaction_id: String,
}

/// Main game state structure holding all aspects of a quantum chess game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Unique identifier for the game
    pub id: Uuid,
    
    /// Current classic chess board state
    pub board: ChessBoard,
    
    /// Current status of the game
    pub status: GameStatus,
    
    /// Player information
    pub white_player: Option<PlayerInfo>,
    pub black_player: Option<PlayerInfo>,
    
    /// Which player's turn it is (true for white, false for black)
    pub white_to_move: bool,
    
    /// Total stake locked in the game (in Core tokens)
    pub total_stake: u64,
    
    /// Quantum states of pieces on the board
    pub quantum_states: HashMap<chess::Square, QuantumPieceState>,
    
    /// History of moves played
    pub move_history: Vec<ChessMove>,
    
    /// History of quantum operations performed
    pub quantum_history: Vec<QuantumOperationHistory>,
    
    /// Blockchain contract address for this game
    pub contract_address: Option<String>,
    
    /// Game creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Last move timestamp
    pub last_move_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Game result description (if game is completed)
    pub result_description: Option<String>,
}

impl GameState {
    /// Create a new game with default settings
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            board: ChessBoard::default(),
            status: GameStatus::Waiting,
            white_player: None,
            black_player: None,
            white_to_move: true,
            total_stake: 0,
            quantum_states: HashMap::new(),
            move_history: Vec::new(),
            quantum_history: Vec::new(),
            contract_address: None,
            created_at: chrono::Utc::now(),
            last_move_at: None,
            result_description: None,
        }
    }

    /// Add a player to the game
    pub fn add_player(&mut self, wallet_address: WalletAddress, stake_amount: u64, color_preference: Option<Color>) -> Result<Color, String> {
        let color = match (color_preference, self.white_player.is_none(), self.black_player.is_none()) {
            // Honor color preference if available
            (Some(Color::White), true, _) => Color::White,
            (Some(Color::Black), _, true) => Color::Black,
            // If preference unavailable, assign an open color
            (Some(Color::White), false, true) => {
                return Err("White player already exists".to_string());
            },
            (Some(Color::Black), true, false) => {
                return Err("Black player already exists".to_string());
            },
            // If no preference, assign first available
            (None, true, _) => Color::White,
            (None, false, true) => Color::Black,
            // No slots available
            _ => return Err("Game is full".to_string()),
        };

        let player = Player {
            wallet_address,
            color,
            stake_amount,
            joined_at: chrono::Utc::now(),
        };

        let player_info = PlayerInfo {
            player: player.clone(),
            captures: 0,
            quantum_moves: 0,
            classical_moves: 0,
            probability_bonus: 0.0,
            superpositions: 0,
            entanglements: 0,
        };

        match color {
            Color::White => self.white_player = Some(player_info),
            Color::Black => self.black_player = Some(player_info),
        }

        self.total_stake += stake_amount;

        // If both players are now in, start the game
        if self.white_player.is_some() && self.black_player.is_some() {
            self.status = GameStatus::InProgress;
        }

        Ok(color)
    }

    /// Initialize all quantum states for pieces on the board
    pub fn initialize_quantum_states(&mut self) {
        // Clear existing quantum states
        self.quantum_states.clear();
        
        // Iterate through all squares on the board
        for rank in 0..8 {
            for file in 0..8 {
                let square = chess::Square::make_square(
                    chess::Rank::from_index(rank),
                    chess::File::from_index(file),
                );
                
                // If there's a piece on this square, create a quantum state for it
                if let Some(piece) = self.board.piece_on(square) {
                    let quantum_state = QuantumPieceState {
                        piece,
                        primary_position: square,
                        superpositions: HashMap::new(),
                        entangled_with: Vec::new(),
                        measurement_probability: 1.0, // Start with 100% probability in primary position
                    };
                    
                    self.quantum_states.insert(square, quantum_state);
                }
            }
        }
    }
    
    /// Get current player based on turn
    pub fn current_player(&self) -> Option<&PlayerInfo> {
        if self.white_to_move {
            self.white_player.as_ref()
        } else {
            self.black_player.as_ref()
        }
    }
    
    /// Get current player as mutable reference
    pub fn current_player_mut(&mut self) -> Option<&mut PlayerInfo> {
        if self.white_to_move {
            self.white_player.as_mut()
        } else {
            self.black_player.as_mut()
        }
    }
    
    /// Check if the game is over
    pub fn is_game_over(&self) -> bool {
        matches!(self.status, GameStatus::Completed | GameStatus::Draw | GameStatus::Abandoned)
    }
    
    /// Record a move in the game history
    pub fn record_move(&mut self, chess_move: ChessMove) {
        self.move_history.push(chess_move);
        self.last_move_at = Some(chrono::Utc::now());
    }
    
    /// Record a quantum operation
    pub fn record_quantum_operation(
        &mut self, 
        operation_type: &str, 
        positions: Vec<chess::Square>, 
        player: WalletAddress, 
        transaction_id: String
    ) {
        let operation = QuantumOperationHistory {
            operation_type: operation_type.to_string(),
            positions,
            player,
            timestamp: chrono::Utc::now(),
            transaction_id,
        };
        
        self.quantum_history.push(operation);
    }
    
    /// Calculate probability of a move succeeding based on quantum state
    pub fn calculate_move_probability(&self, from: chess::Square, to: chess::Square) -> f64 {
        // Get quantum state of the piece
        if let Some(quantum_state) = self.quantum_states.get(&from) {
            // Base probability from the quantum state
            let base_probability = quantum_state.measurement_probability;
            
            // Get player probability bonus
            let player_bonus = if let Some(player) = self.current_player() {
                player.probability_bonus
            } else {
                0.0
            };
            
            // Adjust based on piece's quantum states and entanglements
            let quantum_factor = match quantum_state.superpositions.len() {
                0 => 1.0, // No superposition means full probability
                1 => 0.8, // One superposition reduces probability
                2 => 0.7, // Two superpositions reduces further
                _ => 0.6, // More than two superpositions gives lowest probability
            };
            
            // Entanglement reduces probability
            let entanglement_factor = match quantum_state.entangled_with.len() {
                0 => 1.0,   // No entanglement
                1 => 0.9,   // One entanglement
                _ => 0.8,   // Multiple entanglements
            };
            
            // Combine all factors
            let total_probability = base_probability * quantum_factor * entanglement_factor * (1.0 + player_bonus);
            
            // Ensure probability is between 0 and 1
            total_probability.max(0.0).min(1.0)
        } else {
            // If no quantum state exists, assume classical probabilistic behavior
            0.9 // Default high probability for classical moves
        }
    }
    
    /// End the game with a result
    pub fn end_game(&mut self, winner: Option<Color>, description: &str) {
        self.status = match winner {
            Some(_) => GameStatus::Completed,
            None => GameStatus::Draw,
        };
        
        self.result_description = Some(description.to_string());
    }
}

