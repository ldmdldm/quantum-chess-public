use std::collections::HashSet;

use crate::game::{
    board::{Board, Position, Square},
    pieces::{Piece, PieceType},
    quantum::{QuantumState, Superposition, Entanglement, MeasurementOutcome},
};

/// Rules module for Quantum Chess
/// 
/// This module defines the rules of Quantum Chess, including how pieces move,
/// how quantum mechanics principles are applied, and win conditions.

/// Represents different quantum effects that can be applied to chess pieces
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumEffect {
    /// Superposition effect allows a piece to exist in multiple states simultaneously
    Superposition {
        /// Probability of the primary position (0.0-1.0)
        probability: f32,
        /// Whether the superposition has been observed/collapsed
        observed: bool,
    },
    /// Entanglement effect creates quantum correlation between two pieces
    Entanglement {
        /// ID of the entangled pair
        pair_id: usize,
        /// Whether this entanglement has been observed
        observed: bool,
    },
    /// No quantum effect (classical piece)
    None,
}

/// Defines rules for superposition in quantum chess
#[derive(Debug, Clone)]
pub struct SuperpositionRule {
    /// Maximum number of superpositions allowed per player
    pub max_per_player: usize,
    /// Minimum probability threshold for primary position
    pub min_probability: f32,
    /// Whether kings can be placed in superposition
    pub allow_king_superposition: bool,
    /// Whether pieces in check can enter superposition
    pub allow_while_in_check: bool,
}

/// Defines rules for entanglement in quantum chess
#[derive(Debug, Clone)]
pub struct EntanglementRule {
    /// Maximum number of entangled pairs allowed per player
    pub max_pairs_per_player: usize,
    /// Piece types that can be entangled
    pub allowed_piece_types: Vec<PieceType>,
    /// Whether pieces of different types can be entangled
    pub allow_different_piece_types: bool,
    /// Whether opponent pieces can be entangled
    pub allow_opponent_entanglement: bool,
}

/// Main rules configuration for Quantum Chess
#[derive(Debug, Clone)]
pub struct QuantumRules {
    /// Rules for superposition
    pub superposition: SuperpositionRule,
    /// Rules for entanglement
    pub entanglement: EntanglementRule,
    /// Probability of random quantum effect during capture
    pub capture_measurement_probability: f32,
    /// Whether to allow quantum moves while in check
    pub allow_quantum_moves_in_check: bool,
}

impl Default for QuantumRules {
    fn default() -> Self {
        Self {
            superposition: SuperpositionRule {
                max_per_player: MAX_SUPERPOSITIONS,
                min_probability: 0.2,
                allow_king_superposition: false,
                allow_while_in_check: false,
            },
            entanglement: EntanglementRule {
                max_pairs_per_player: 3,
                allowed_piece_types: vec![
                    PieceType::Knight, 
                    PieceType::Bishop, 
                    PieceType::Queen
                ],
                allow_different_piece_types: true,
                allow_opponent_entanglement: false,
            },
            capture_measurement_probability: 0.5,
            allow_quantum_moves_in_check: false,
        }
    }
}

/// Maximum number of superpositions allowed per player
pub const MAX_SUPERPOSITIONS: usize = 4;

/// Represents different types of quantum moves available in Quantum Chess
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumMove {
    /// Classical move with deterministic outcome (like traditional chess)
    Classical {
        piece: Piece,
        from: Position,
        to: Position,
    },
    /// Split move creates superposition of a piece in two locations
    Split {
        piece: Piece,
        from: Position,
        to_primary: Position,
        to_secondary: Position,
        probability: f32, // Probability of primary outcome (0.0-1.0)
    },
    /// Merge move collapses a superposition into a single position
    Merge {
        piece: Piece,
        from_primary: Position,
        from_secondary: Position,
        to: Position,
    },
    /// Entanglement move creates quantum correlation between two pieces
    Entangle {
        piece1: Piece,
        position1: Position,
        piece2: Piece,
        position2: Position,
    },
    /// Measurement forces collapse of quantum states
    Measure {
        positions: Vec<Position>,
    },
}

/// Defines results of a quantum move
#[derive(Debug, Clone)]
pub enum MoveResult {
    Success,
    InvalidMove(String),
    GameOver(GameResult),
}

/// Defines possible game results
#[derive(Debug, Clone, PartialEq)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Draw,
    InProgress,
}

/// Main rules implementation for Quantum Chess
pub struct Rules {
    /// Tracks which positions are in superposition
    superpositions: HashSet<Superposition>,
    /// Tracks entangled pieces
    entanglements: Vec<Entanglement>,
    /// Current game state
    game_result: GameResult,
    /// Quantum rules configuration
    quantum_rules: QuantumRules,
}

impl Rules {
    /// Creates a new rules instance
    pub fn new() -> Self {
        Self {
            superpositions: HashSet::new(),
            entanglements: Vec::new(),
            game_result: GameResult::InProgress,
            quantum_rules: QuantumRules::default(),
        }
    }
    
    /// Creates a new rules instance with custom quantum rules
    pub fn with_quantum_rules(quantum_rules: QuantumRules) -> Self {
        Self {
            superpositions: HashSet::new(),
            entanglements: Vec::new(),
            game_result: GameResult::InProgress,
            quantum_rules,
        }
    }
    
    /// Gets a reference to the current quantum rules
    pub fn quantum_rules(&self) -> &QuantumRules {
        &self.quantum_rules
    }
    
    /// Updates the quantum rules
    pub fn set_quantum_rules(&mut self, quantum_rules: QuantumRules) {
        self.quantum_rules = quantum_rules;
    }

    /// Validates if a classical chess move is legal
    pub fn is_valid_classical_move(&self, board: &Board, piece: &Piece, from: &Position, to: &Position) -> bool {
        // Check if destination is occupied by same color piece
        if let Some(dest_piece) = board.get_piece(to) {
            if dest_piece.color == piece.color {
                return false;
            }
        }

        // Validate move based on piece type
        match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(board, piece, from, to),
            PieceType::Knight => self.is_valid_knight_move(board, piece, from, to),
            PieceType::Bishop => self.is_valid_bishop_move(board, piece, from, to),
            PieceType::Rook => self.is_valid_rook_move(board, piece, from, to),
            PieceType::Queen => self.is_valid_queen_move(board, piece, from, to),
            PieceType::King => self.is_valid_king_move(board, piece, from, to),
        }
    }

    // Implementations for specific piece move validation
    fn is_valid_pawn_move(&self, board: &Board, piece: &Piece, from: &Position, to: &Position) -> bool {
        // Normal pawn movement rules, including first move, capture, en passant, etc.
        // Simplified implementation for brevity
        true // Placeholder - would contain actual implementation
    }

    fn is_valid_knight_move(&self, board: &Board, piece: &Piece, from: &Position, to: &Position) -> bool {
        let dx = (to.x as i32 - from.x as i32).abs();
        let dy = (to.y as i32 - from.y as i32).abs();
        
        // Knight moves in an L-shape: 2 squares in one direction and 1 square perpendicular
        (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
    }

    fn is_valid_bishop_move(&self, board: &Board, piece: &Piece, from: &Position, to: &Position) -> bool {
        let dx = (to.x as i32 - from.x as i32).abs();
        let dy = (to.y as i32 - from.y as i32).abs();
        
        // Bishop moves diagonally
        if dx != dy {
            return false;
        }
        
        // Check if path is clear
        self.is_diagonal_path_clear(board, from, to)
    }

    fn is_valid_rook_move(&self, board: &Board, piece: &Piece, from: &Position, to: &Position) -> bool {
        // Rook moves horizontally or vertically
        if from.x != to.x && from.y != to.y {
            return false;
        }
        
        // Check if path is clear
        self.is_straight_path_clear(board, from, to)
    }

    fn is_valid_queen_move(&self, board: &Board, piece: &Piece, from: &Position, to: &Position) -> bool {
        // Queen can move like a rook or bishop
        self.is_valid_rook_move(board, piece, from, to) || self.is_valid_bishop_move(board, piece, from, to)
    }

    fn is_valid_king_move(&self, board: &Board, piece: &Piece, from: &Position, to: &Position) -> bool {
        let dx = (to.x as i32 - from.x as i32).abs();
        let dy = (to.y as i32 - from.y as i32).abs();
        
        // King moves one square in any direction
        dx <= 1 && dy <= 1
        
        // Castling rules would be implemented here
    }

    // Helper methods for path checking
    fn is_diagonal_path_clear(&self, board: &Board, from: &Position, to: &Position) -> bool {
        let dx = (to.x as i32 - from.x as i32).signum();
        let dy = (to.y as i32 - from.y as i32).signum();
        let mut x = from.x as i32 + dx;
        let mut y = from.y as i32 + dy;
        
        while x != to.x as i32 || y != to.y as i32 {
            if board.get_piece(&Position { x: x as u8, y: y as u8 }).is_some() {
                return false;
            }
            x += dx;
            y += dy;
        }
        
        true
    }

    fn is_straight_path_clear(&self, board: &Board, from: &Position, to: &Position) -> bool {
        if from.x == to.x {
            // Vertical movement
            let start = from.y.min(to.y) + 1;
            let end = from.y.max(to.y);
            
            for y in start..end {
                if board.get_piece(&Position { x: from.x, y }).is_some() {
                    return false;
                }
            }
        } else {
            // Horizontal movement
            let start = from.x.min(to.x) + 1;
            let end = from.x.max(to.x);
            
            for x in start..end {
                if board.get_piece(&Position { x, y: from.y }).is_some() {
                    return false;
                }
            }
        }
        
        true
    }

    /// Validates a quantum split move
    pub fn is_valid_split_move(
        &self, 
        board: &Board, 
        piece: &Piece, 
        from: &Position, 
        to_primary: &Position, 
        to_secondary: &Position
    ) -> bool {
        // Check if player has reached maximum superpositions
        let player_superpositions = self.superpositions.iter()
            .filter(|s| board.get_piece(&s.position1).map_or(false, |p| p.color == piece.color) || 
                    board.get_piece(&s.position2).map_or(false, |p| p.color == piece.color))
            .count();
            
        if player_superpositions >= self.quantum_rules.superposition.max_per_player {
            return false;
        }
        
        // Check if piece is a king and king superposition is not allowed
        if piece.piece_type == PieceType::King && !self.quantum_rules.superposition.allow_king_superposition {
            return false;
        }

        // Both target positions must be valid classical moves
        self.is_valid_classical_move(board, piece, from, to_primary) && 
        self.is_valid_classical_move(board, piece, from, to_secondary)
    }

    /// Validates a quantum merge move
    pub fn is_valid_merge_move(
        &self, 
        board: &Board, 
        piece: &Piece, 
        from_primary: &Position, 
        from_secondary: &Position, 
        to: &Position
    ) -> bool {
        // Check if the two source positions are in superposition
        let superposition_exists = self.superpositions.iter().any(|s| 
            (s.position1 == *from_primary && s.position2 == *from_secondary) || 
            (s.position1 == *from_secondary && s.position2 == *from_primary)
        );
        
        if !superposition_exists {
            return false;
        }
        
        // The target position must be a valid move from both source positions
        self.is_valid_classical_move(board, piece, from_primary, to) && 
        self.is_valid_classical_move(board, piece, from_secondary, to)
    }

    /// Validates a quantum entanglement move
    pub fn is_valid_entangle_move(
        &self, 
        board: &Board, 
        piece1: &Piece, 
        position1: &Position, 
        piece2: &Piece, 
        position2: &Position
    ) -> bool {
        // Check if pieces have different colors and opponent entanglement is not allowed
        if piece1.color != piece2.color && !self.quantum_rules.entanglement.allow_opponent_entanglement {
            return false;
        }
        
        // Check if pieces have different types and that's not allowed
        if piece1.piece_type != piece2.piece_type && !self.quantum_rules.entanglement.allow_different_piece_types {
            return false;
        }
        
        // Check if both piece types are allowed to be entangled
        let allowed_types = &self.quantum_rules.entanglement.allowed_piece_types;
        allowed_types.contains(&piece1.piece_type) && allowed_types.contains(&piece2.piece_type)
    }

    /// Executes a quantum move and returns the result
    pub fn execute_move(&mut self, board: &mut Board, quantum_move: QuantumMove) -> MoveResult {
        match quantum_move {
            QuantumMove::Classical { piece, from, to } => {
                if !self.is_valid_classical_move(board, &piece, &from, &to) {
                    return MoveResult::InvalidMove("Invalid classical move".to_string());
                }
                
                // Execute classical move
                board.remove_piece(&from);
                board.set_piece(&to, piece);
                
                // Check for check, checkmate, etc.
                self.update_game_state(board);
                
                MoveResult::Success
            },
            
            QuantumMove::Split { piece, from, to_primary, to_secondary, probability } => {
                if !self.is_valid_split_move(board, &piece, &from, &to_primary, &to_secondary) {
                    return MoveResult::InvalidMove("Invalid split move".to_string());
                }
                
                // Create superposition
                let superposition = Superposition {
                    position1: to_primary,
                    position2: to_secondary,
                    probability,
                };
                
                // Remove piece from original position
                board.remove_piece(&from);
                
                // Add superposition state
                self.superpositions.insert(superposition);
                
                // Set quantum state for both positions
                board.set_quantum_state(&to_primary, QuantumState::Superposition);
                board.set_quantum_state(&to_secondary, QuantumState::Superposition);
                
                MoveResult::Success
            },
            
            QuantumMove::Merge { piece, from_primary, from_secondary, to } => {
                if !self.is_valid_merge_move(board, &piece, &from_primary, &from_secondary, &to) {
                    return MoveResult::InvalidMove("Invalid merge move".to_string());
                }
                
                // Find and remove the superposition
                let superposition = self.superpositions.iter()
                    .find(|s| (s.position1 == from_primary && s.position2 == from_secondary) || 
                              (s.position1 == from_secondary && s.position2 == from_primary))
                    .cloned()
                    .unwrap();
                
                self.superpositions.remove(&superposition);
                
                // Clear quantum states
                board.clear_quantum_state(&from_primary);
                board.clear_quantum_state(&from_secondary);
                
                // Set piece at target position
                board.set_piece(&to, piece);
                
                MoveResult::Success
            },
            
            QuantumMove::Entangle { piece1, position1, piece2, position2 } => {
                if !self.is_valid_entangle_move(board, &piece1, &position1, &piece2, &position2) {
                    return MoveResult::InvalidMove("Invalid entanglement move".to_string());
                }
                
                // Create entanglement
                let entanglement = Entanglement {
                    position1,
                    position2,
                };
                
                self.entanglements.push(entanglement);
                
                // Set quantum states
                board.set_quantum_state(&position1, QuantumState::Entangled);
                board.set_quantum_state(&position2, QuantumState::Entangled);
                
                MoveResult::Success
            },
            
            QuantumMove::Measure { positions } => {
                // Force collapse of quantum states at the specified positions
                for position in positions {
                    self.collapse_quantum_state(board, &position);
                }
                
                // Check for win conditions after measurement
                self.update_game_state(board);
                
                MoveResult::Success
            },
        }
    }

    /// Updates the game state after a move, checking for check, checkmate, or stalemate conditions
    pub fn update_game_state(&mut self, board: &Board) -> MoveResult {
        // Check if either king is in check
        let white_in_check = self.is_king_in_check(board, true);
        let black_in_check = self.is_king_in_check(board, false);
        
        // Check for checkmate or stalemate
        if white_in_check && self.is_checkmate(board, true) {
            self.game_result = GameResult::BlackWins;
            return MoveResult::GameOver(GameResult::BlackWins);
        }
        
        if black_in_check && self.is_checkmate(board, false) {
            self.game_result = GameResult::WhiteWins;
            return MoveResult::GameOver(GameResult::WhiteWins);
        }
        
        // Check for stalemate (not in check but no legal moves)
        if (!white_in_check && self.is_stalemate(board, true)) || 
           (!black_in_check && self.is_stalemate(board, false)) {
            self.game_result = GameResult::Draw;
            return MoveResult::GameOver(GameResult::Draw);
        }
        
        // Check other draw conditions (insufficient material, 50-move rule, etc.)
        if self.is_draw_by_insufficient_material(board) {
            self.game_result = GameResult::Draw;
            return MoveResult::GameOver(GameResult::Draw);
        }
        
        // Game continues
        MoveResult::Success
    }
    
    /// Checks if the king of the specified color is in check
    fn is_king_in_check(&self, board: &Board, is_white: bool) -> bool {
        // Find the king's position
        let mut king_position = None;
        for y in 0..8 {
            for x in 0..8 {
                let pos = Position { x, y };
                if let Some(piece) = board.get_piece(&pos) {
                    if piece.piece_type == PieceType::King && piece.color == is_white {
                        king_position = Some(pos);
                        break;
                    }
                }
            }
            if king_position.is_some() {
                break;
            }
        }
        
        // If king is in superposition, check both positions
        let king_positions = if let Some(pos) = king_position {
            let mut positions = vec![pos];
            
            // Add superposition positions if the king is in superposition
            for superposition in &self.superpositions {
                if board.get_piece(&superposition.position1).map_or(false, |p| 
                    p.piece_type == PieceType::King && p.color == is_white) {
                    positions.push(superposition.position1);
                    positions.push(superposition.position2);
                }
                
                if board.get_piece(&superposition.position2).map_or(false, |p| 
                    p.piece_type == PieceType::King && p.color == is_white) {
                    positions.push(superposition.position1);
                    positions.push(superposition.position2);
                }
            }
            
            positions.dedup(); // Remove duplicates
            positions
        } else {
            // No king found (shouldn't happen in a valid game)
            return false;
        };
        
        // Check if any opponent piece can attack the king
        for king_pos in king_positions {
            for y in 0..8 {
                for x in 0..8 {
                    let pos = Position { x, y };
                    if let Some(piece) = board.get_piece(&pos) {
                        if piece.color != is_white {
                            // Check if this opponent piece can capture the king
                            if self.is_valid_classical_move(board, &piece, &pos, &king_pos) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Checks if the player of the specified color is in checkmate
    fn is_checkmate(&self, board: &Board, is_white: bool) -> bool {
        // If the king is not in check, it can't be checkmate
        if !self.is_king_in_check(board, is_white) {
            return false;
        }
        
        // Check if any move can get the player out of check
        // In quantum chess, this includes both classical and quantum moves
        !self.has_any_legal_move(board, is_white)
    }
    
    /// Checks if the player of the specified color is in stalemate
    fn is_stalemate(&self, board: &Board, is_white: bool) -> bool {
        // If the king is in check, it's not stalemate
        if self.is_king_in_check(board, is_white) {
            return false;
        }
        
        // Check if the player has any legal moves
        !self.has_any_legal_move(board, is_white)
    }
    
    /// Checks if the player of the specified color has any legal moves
    fn has_any_legal_move(&self, board: &Board, is_white: bool) -> bool {
        // Check all pieces of the specified color
        for y in 0..8 {
            for x in 0..8 {
                let pos = Position { x, y };
                if let Some(piece) = board.get_piece(&pos) {
                    if piece.color == is_white {
                        // Check all possible destination squares
                        for dest_y in 0..8 {
                            for dest_x in 0..8 {
                                let dest = Position { x: dest_x, y: dest_y };
                                
                                // Check if a classical move is legal
                                if self.is_valid_classical_move(board, &piece, &pos, &dest) {
                                    // Create a temporary board to simulate the move
                                    let mut temp_board = board.clone();
                                    temp_board.remove_piece(&pos);
                                    temp_board.set_piece(&dest, piece);
                                    
                                    // Check if the move puts the king out of check
                                    if !self.is_king_in_check(&temp_board, is_white) {
                                        return true;
                                    }
                                }
                                
                                // For quantum chess, also check quantum moves
                                // (Split, Entangle, etc. if allowed while in check)
                                if self.quantum_rules.allow_quantum_moves_in_check {
                                    // Check quantum split moves, entanglements, etc.
                                    // This would be more complex and depends on the specific quantum rules
                                    // Simplified for this implementation
                                }
                            }
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Checks if the game is a draw due to insufficient material
    fn is_draw_by_insufficient_material(&self, board: &Board) -> bool {
        // Count pieces
        let mut white_knights = 0;
        let mut white_bishops = 0;
        let mut black_knights = 0;
        let mut black_bishops = 0;
        let mut other_pieces = 0;
        
        for y in 0..8 {
            for x in 0..8 {
                let pos = Position { x, y };
                if let Some(piece) = board.get_piece(&pos) {
                    match piece.piece_type {
                        PieceType::King => {},
                        PieceType::Knight => {
                            if piece.color {
                                white_knights += 1;
                            } else {
                                black_knights += 1;
                            }
                        },
                        PieceType::Bishop => {
                            if piece.color {
                                white_bishops += 1;
                            } else {
                                black_bishops += 1;
                            }
                        },
                        _ => {
                            other_pieces += 1;
                        }
                    }
                }
            }
        }
        
        // King vs King
        if other_pieces == 0 && white_knights == 0 && white_bishops == 0 &&
           black_knights == 0 && black_bishops == 0 {
            return true;
        }
        
        // King + Bishop vs King or King + Knight vs King
        if other_pieces == 0 && 
           ((white_knights == 0 && white_bishops == 1 && black_knights == 0 && black_bishops == 0) ||
            (white_knights == 1 && white_bishops == 0 && black_knights == 0 && black_bishops == 0) ||
            (black_knights == 0 && black_bishops == 1 && white_knights == 0 && white_bishops == 0) ||
            (black_knights == 1 && black_bishops == 0 && white_knights == 0 && white_bishops == 0)) {
            return true;
        }
        
        // King + Bishop vs King + Bishop (same color bishops)
        if other_pieces == 0 && white_knights == 0 && black_knights == 0 &&
           white_bishops == 1 && black_bishops == 1 {
            // Ideally we would check if the bishops are on the same color squares
            // For simplicity, we'll assume they might be on different colors
            return false;
        }
        
        false
    }

    /// Collapses a quantum state based on probabilities
    fn collapse_quantum_state(&mut self, board: &mut Board, position: &Position) {
        // Check if position is in superposition
        for superposition in self.superpositions.clone() {
            if superposition.position1 == *position || superposition.position2 == *position {
                // Determine outcome based on probability
                let outcome = if rand::random::<f32>() < superposition.probability {
                    MeasurementOutcome::Primary
                } else {
                    MeasurementOutcome::Secondary
                };
                
                // Apply outcome
                match outcome {
                    MeasurementOutcome::Primary => {
                

