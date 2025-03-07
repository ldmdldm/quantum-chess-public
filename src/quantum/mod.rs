use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use uuid::Uuid;

/// Quantum state representing a superposition of chess piece positions
pub struct QuantumState {
    /// Quantum amplitude vector in complex Hilbert space
    amplitudes: Array1<Complex64>,
    /// Mapping from basis state indices to chess positions
    basis_states: Vec<ChessPosition>,
    /// Quantum entanglement information with other pieces
    entanglements: HashMap<Uuid, EntanglementType>,
}

/// A chess position in standard algebraic notation (e.g., "e4")
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChessPosition {
    pub notation: String,
    pub row: u8,
    pub col: u8,
}

/// Types of quantum entanglement between chess pieces
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntanglementType {
    /// Bell-state entanglement (maximum entanglement)
    Bell,
    /// W-state entanglement (distributed entanglement)
    WState,
    /// GHZ-state entanglement (multi-piece entanglement)
    GHZ,
    /// Custom entanglement with specific correlation probability
    Custom { correlation: f64 },
}

/// Probability zone for quantum moves based on stake amount
#[derive(Clone, Debug, PartialEq)]
pub enum ProbabilityZone {
    /// High probability zone (75-95%)
    High,
    /// Medium probability zone (40-75%)
    Medium,
    /// Low probability zone (10-40%)
    Low,
    /// Quantum tunneling zone (1-10%)
    QuantumTunneling,
    /// Custom probability zone with min and max values
    Custom { min: f64, max: f64 },
}

impl QuantumState {
    /// Create a new quantum state with a single definite position
    pub fn new(position: ChessPosition) -> Self {
        let mut amplitudes = Array1::zeros(1);
        amplitudes[0] = Complex64::new(1.0, 0.0);
        
        Self {
            amplitudes,
            basis_states: vec![position],
            entanglements: HashMap::new(),
        }
    }

    /// Create a quantum state in a superposition of multiple positions
    pub fn superposition(positions: Vec<ChessPosition>, probabilities: Vec<f64>) -> Result<Self, String> {
        if positions.is_empty() {
            return Err("Cannot create superposition with empty positions".into());
        }
        
        if positions.len() != probabilities.len() {
            return Err("Positions and probabilities must have the same length".into());
        }
        
        let sum: f64 = probabilities.iter().sum();
        if (sum - 1.0).abs() > 1e-6 {
            return Err("Probabilities must sum to 1.0".into());
        }
        
        let mut amplitudes = Array1::zeros(positions.len());
        for (i, &prob) in probabilities.iter().enumerate() {
            // Convert probability to amplitude
            amplitudes[i] = Complex64::new(prob.sqrt(), 0.0);
        }
        
        Ok(Self {
            amplitudes,
            basis_states: positions,
            entanglements: HashMap::new(),
        })
    }

    /// Entangle this quantum state with another piece
    pub fn entangle(&mut self, piece_id: Uuid, entanglement_type: EntanglementType) {
        self.entanglements.insert(piece_id, entanglement_type);
    }

    /// Remove entanglement with a specific piece
    pub fn disentangle(&mut self, piece_id: &Uuid) -> bool {
        self.entanglements.remove(piece_id).is_some()
    }

    /// Check if this piece is entangled with another specific piece
    pub fn is_entangled_with(&self, piece_id: &Uuid) -> bool {
        self.entanglements.contains_key(piece_id)
    }

    /// Get all pieces this piece is entangled with
    pub fn entangled_pieces(&self) -> Vec<Uuid> {
        self.entanglements.keys().cloned().collect()
    }

    /// Get the type of entanglement with a specific piece
    pub fn entanglement_type(&self, piece_id: &Uuid) -> Option<&EntanglementType> {
        self.entanglements.get(piece_id)
    }

    /// Get the probabilities of measuring each position
    pub fn probabilities(&self) -> Vec<(ChessPosition, f64)> {
        self.basis_states
            .iter()
            .zip(self.amplitudes.iter())
            .map(|(pos, amp)| (pos.clone(), amp.norm_sqr()))
            .collect()
    }

    /// Quantum measurement - collapses the superposition to a single position
    /// Returns the measured position
    pub fn measure(&mut self) -> ChessPosition {
        let probs: Vec<f64> = self.amplitudes.iter().map(|amp| amp.norm_sqr()).collect();
        
        // Create a distribution based on probabilities
        let dist = rand::distributions::WeightedIndex::new(&probs).unwrap();
        let mut rng = rand::thread_rng();
        
        // Sample from the distribution
        let idx = dist.sample(&mut rng);
        let measured_position = self.basis_states[idx].clone();
        
        // Collapse the state to the measured position
        let mut new_amplitudes = Array1::zeros(1);
        new_amplitudes[0] = Complex64::new(1.0, 0.0);
        
        self.amplitudes = new_amplitudes;
        self.basis_states = vec![measured_position.clone()];
        
        measured_position
    }

    /// Measure the position but with entanglement effects considered
    pub fn measure_with_entanglement(&mut self, entangled_states: &HashMap<Uuid, QuantumState>) -> ChessPosition {
        // If no entanglements, perform regular measurement
        if self.entanglements.is_empty() {
            return self.measure();
        }
        
        // Apply entanglement effects to measurement probabilities
        // This is a simplified model of quantum entanglement
        let mut modified_probs = self.amplitudes.iter().map(|amp| amp.norm_sqr()).collect::<Vec<_>>();
        
        for (piece_id, entanglement_type) in &self.entanglements {
            if let Some(entangled_state) = entangled_states.get(piece_id) {
                modify_probabilities(&mut modified_probs, entangled_state, entanglement_type);
            }
        }
        
        // Normalize probabilities
        let sum: f64 = modified_probs.iter().sum();
        if sum > 0.0 {
            for prob in &mut modified_probs {
                *prob /= sum;
            }
        }
        
        // Sample from modified distribution
        let dist = rand::distributions::WeightedIndex::new(&modified_probs).unwrap();
        let mut rng = rand::thread_rng();
        let idx = dist.sample(&mut rng);
        
        // Collapse the state
        let measured_position = self.basis_states[idx].clone();
        self.amplitudes = Array1::zeros(1);
        self.amplitudes[0] = Complex64::new(1.0, 0.0);
        self.basis_states = vec![measured_position.clone()];
        
        measured_position
    }

    /// Add a new possible position to the superposition
    pub fn add_position(&mut self, position: ChessPosition, amplitude: Complex64) -> Result<(), String> {
        // Check if position already exists
        if let Some(idx) = self.basis_states.iter().position(|pos| *pos == position) {
            // Update the amplitude
            self.amplitudes[idx] += amplitude;
        } else {
            // Add new position
            self.basis_states.push(position);
            
            // Extend amplitude vector
            let mut new_amplitudes = Array1::zeros(self.amplitudes.len() + 1);
            for (i, amp) in self.amplitudes.iter().enumerate() {
                new_amplitudes[i] = *amp;
            }
            new_amplitudes[self.amplitudes.len()] = amplitude;
            self.amplitudes = new_amplitudes;
        }
        
        // Normalize the state
        self.normalize();
        Ok(())
    }

    /// Normalize the quantum state
    fn normalize(&mut self) {
        let norm_squared: f64 = self.amplitudes.iter().map(|amp| amp.norm_sqr()).sum();
        let norm = norm_squared.sqrt();
        
        if norm > 0.0 {
            for amp in self.amplitudes.iter_mut() {
                *amp /= Complex64::new(norm, 0.0);
            }
        }
    }
}

/// Helper function to modify probabilities based on entanglement
fn modify_probabilities(
    probs: &mut Vec<f64>,
    entangled_state: &QuantumState,
    entanglement_type: &EntanglementType,
) {
    match entanglement_type {
        EntanglementType::Bell => {
            // Bell entanglement - maximum correlation
            let entangled_probs: Vec<f64> = entangled_state.amplitudes.iter().map(|amp| amp.norm_sqr()).collect();
            
            // For Bell states, we align probabilities
            for (i, prob) in probs.iter_mut().enumerate() {
                if i < entangled_probs.len() {
                    *prob = (*prob + entangled_probs[i]) / 2.0;
                }
            }
        },
        EntanglementType::WState => {
            // W-state entanglement - distributed correlation
            // Boost probabilities of states with similar positions
            let entangled_positions = &entangled_state.basis_states;
            for (i, pos) in entangled_positions.iter().enumerate() {
                if i < probs.len() {
                    probs[i] *= 1.2; // Boost probability
                }
            }
        },
        EntanglementType::GHZ => {
            // GHZ-state entanglement - multi-piece correlation
            // Either all pieces move or none move
            let mut rng = rand::thread_rng();
            let mut all_or_none = Uniform::from(0..2).sample(&mut rng);
            
            if all_or_none == 0 {
                // Boost first probability (original position)
                if !probs.is_empty() {
                    probs[0] *= 2.0;
                }
            } else {
                // Boost other probabilities
                for i in 1..probs.len() {
                    probs[i] *= 1.5;
                }
            }
        },
        EntanglementType::Custom { correlation } => {
            // Custom correlation
            let entangled_probs: Vec<f64> = entangled_state.amplitudes.iter().map(|amp| amp.norm_sqr()).collect();
            
            for (i, prob) in probs.iter_mut().enumerate() {
                if i < entangled_probs.len() {
                    *prob = *prob * (1.0 - correlation) + entangled_probs[i] * *correlation;
                }
            }
        }
    }
}

/// Calculates move probability based on stake, quantum zone, and board state
pub fn calculate_move_probability(
    piece_position: &ChessPosition,
    target_position: &ChessPosition,
    stake_amount: f64,
    quantum_zone: &ProbabilityZone,
    is_in_check: bool,
) -> f64 {
    // Base probability from the quantum zone
    let (min_prob, max_prob) = match quantum_zone {
        ProbabilityZone::High => (0.75, 0.95),
        ProbabilityZone::Medium => (0.40, 0.75),
        ProbabilityZone::Low => (0.10, 0.40),
        ProbabilityZone::QuantumTunneling => (0.01, 0.10),
        ProbabilityZone::Custom { min, max } => (*min, *max),
    };
    
    // Calculate distance-based factor (Manhattan distance)
    let distance = ((piece_position.row as i8 - target_position.row as i8).abs() + 
                    (piece_position.col as i8 - target_position.col as i8).abs()) as f64;
    
    // Distance penalty: longer moves have lower probability
    let distance_factor = 1.0 / (1.0 + distance / 8.0);
    
    // Stake bonus: higher stakes provide better odds
    // Logarithmic scale to prevent excessive advantage from high stakes
    let stake_factor = if stake_amount > 0.0 {
        0.1 * (1.0 + stake_amount.ln() / 10.0)
    } else {
        0.0
    };
    
    // Check penalty: being in check reduces probability
    let check_factor = if is_in_check { -0.1 } else { 0.0 };
    
    // Calculate final probability within the zone's range
    let zone_range = max_prob - min_prob;
    let base_prob = min_prob + zone_range * 0.5; // Start in the middle of the range
    
    // Apply all factors
    let adjusted_prob = base_prob + distance_factor * zone_range / 2.0 + stake_factor + check_factor;
    
    // Ensure the result stays within the zone's limits
    adjusted_prob.max(min_prob).min(max_prob)
}

/// Creates a quantum teleportation circuit for moving a piece to a distant square
pub fn quantum_teleport(
    source_position: &ChessPosition,
    target_position: &ChessPosition,
    stake_amount: f64,
) -> Result<f64, String> {
    // Check if teleportation is allowed
    if stake_amount < 10.0 {
        return Err("Insufficient stake for quantum teleportation".into());
    }
    
    // Distance between source and target
    let distance = ((source_position.row as i8 - target_position.row as i8).abs() + 
                    (source_position.col as i8 - target_position.col as i8).abs()) as f64;
    
    // Calculate teleportation probability (inversely proportional to distance)
    let base_prob = 0.3 * (1.0 - distance / 16.0).max(0.1);
    
    // Stake boosts teleportation probability (logarithmic scale)
    let stake_boost = 0.2 * (stake_amount.ln() / ln(100.0));
    
    // Final probability capped at 50% (teleportation should remain uncertain)
    let final_prob = (base_prob + stake_boost).min(0.5);
    
    Ok(final_prob)
}

/// Helper function for natural logarithm
fn ln(x: f64) -> f64 {
    x.ln()
}

/// Apply quantum interference between two position amplitudes
pub fn apply_interference(
    state: &mut QuantumState,
    position1: &ChessPosition,
    position2: &ChessPosition,
    phase: f64,
) -> Result<(), String> {
    // Find indices of the two positions
    let idx1 = state.basis_states.iter().position(|pos| pos == position1);
    let idx2 = state.basis_states.iter().position(|pos| pos == position2);
    
    match (idx1, idx2) {
        (Some(i), Some(j)) => {
            // Create interference between the two positions
            let amp1 = state.amplitudes[i];
            let amp2 = state.amplitudes[j];
            
            // Apply phase rotation and mix amplitudes
            let phase_factor = Complex64::new(phase.cos(), phase.sin());
            state.amplitudes[i] = amp1 + phase_factor * amp2;
            state.amplitudes[j] = amp2 + phase_factor.conj() * amp1;
            
            // Normalize after interference
            state.normalize();
            Ok(())
        },
        _ => Err("One or both positions not found in quantum state".into()),
    }
}

/// Creates a superposition state for a piece at multiple positions
pub fn create_superposition(
    game_id: &Uuid, 
    piece_type: &str, 
    positions: Vec<ChessPosition>, 
    probabilities: Vec<f64>
) -> Result<QuantumState, String> {
    if positions.is_empty() {
        return Err("Cannot create superposition with empty positions".into());
    }
    
    if positions.len() != probabilities.len() {
        return Err("Positions and probabilities must have the same length".into());
    }
    
    // Create a quantum state in superposition
    QuantumState::superposition(positions, probabilities)
}

/// Creates quantum entanglement between two pieces
pub fn create_entanglement(
    game_id: &Uuid,
    piece1_id: &Uuid,
    piece2_id: &Uuid,
    entanglement_type: EntanglementType,
    states: &mut HashMap<Uuid, QuantumState>,
) -> Result<(), String> {
    // Check if both pieces exist
    if !states.contains_key(piece1_id) || !states.contains_key(piece2_id) {
        return Err("One or both pieces not found".into());
    }
    
    // Apply entanglement to both pieces
    if let Some(state1) = states.get_mut(piece1_id) {
        state1.entangle(*piece2_id, entanglement_type.clone());
    }
    
    if let Some(state2) = states.get_mut(piece2_id) {
        state2.entangle(*piece1_id, entanglement_type);
    }
    
    Ok(())
}

/// Collapses the quantum state of a piece through measurement
pub fn collapse_state(
    game_id: &Uuid,
    piece_id: &Uuid,
    states: &mut HashMap<Uuid, QuantumState>,
) -> Result<ChessPosition, String> {
    // Get the quantum state of the piece
    let state = states.get_mut(piece_id)
        .ok_or_else(|| "Piece not found".to_string())?;
    
    // Check if the piece is entangled with other pieces
    let entangled_pieces = state.entangled_pieces();
    
    // If entangled, use entanglement-aware measurement
    if !entangled_pieces.is_empty() {
        let measured_position = state.measure_with_entanglement(states);
        
        // Update entangled pieces' states according to the measurement result
        for other_piece_id in entangled_pieces {
            if let Some(other_state) = states.get_mut(&other_piece_id) {
                // Depending on the entanglement type, update the other piece's state
                if let Some(entanglement_type) = state.entanglement_type(&other_piece_id) {
                    update_entangled_state(other_state, entanglement_type, &measured_position);
                }
                
                // Remove entanglement after measurement
                other_state.disentangle(piece_id);
            }
        }
        
        // Clear entanglements after measurement
        for other_piece_id in &entangled_pieces {
            state.disentangle(other_piece_id);
        }
        
        Ok(measured_position)
    } else {
        // Regular measurement if not entangled
        let measured_position = state.measure();
        Ok(measured_position)
    }
}

/// Update an entangled piece's state based on measurement of the other piece
fn update_entangled_state(
    state: &mut QuantumState,
    entanglement_type: &EntanglementType,
    measured_position: &ChessPosition,
) {
    match entanglement_type {
        EntanglementType::Bell => {
            // In Bell entanglement, force collapse to correlated state
            if state.basis_states.len() > 1 {
                // Find the most correlated basis state
                let idx = 0; // Simplified - in a real implementation this would be more complex
                let correlated_position = state.basis_states[idx].clone();
                
                // Collapse to the correlated position
                state.amplitudes = Array1::zeros(1);
                state.amplitudes[0] = Complex64::new(1.0, 0.0);
                state.basis_states = vec![correlated_position];
            }
        },
        EntanglementType::WState | EntanglementType::GHZ => {
            // Partially collapse, but maintain some superposition
            // This is a simplified implementation
            state.normalize();
        },
        EntanglementType::Custom { correlation } => {
            // Apply custom correlation effect
            // This is a simplified implementation
            state.normalize();
        }
    }
}

/// Get all quantum states for a specific game
pub fn get_game_quantum_states(
    game_id: &Uuid,
    db_connection: &mut dyn DatabaseConnection,
) -> Result<HashMap<Uuid, QuantumState>, String> {
    // In a real implementation, this would query the database for all quantum states
    // For now, we're returning a placeholder implementation
    
    // This would typically fetch quantum states from the database
    // db_connection.query_quantum_states(game_id)
    
    // For demonstration purposes, we'll create a dummy response
    let mut states = HashMap::new();
    
    // Example: Create a few quantum states for demonstration
    let rook_id = Uuid::new_v4();
    let queen_id = Uuid::new_v4();
    
    // Rook in superposition between a1 and a8
    let rook_positions = vec![
        ChessPosition { notation: "a1".to_string(), row: 0, col: 0 },
        ChessPosition { notation: "a8".to_string(), row: 7, col: 0 },
    ];
    let rook_probabilities = vec![0.7, 0.3];
    if let Ok(rook_state) = QuantumState::superposition(rook_positions, rook_probabilities) {
        states.insert(rook_id, rook_state);
    }
    
    // Queen in a definite position
    let queen_position = ChessPosition { notation: "d1".to_string(), row: 0, col: 3 };
    states.insert(queen_id, QuantumState::new(queen_position));
    
    Ok(states)
}

/// Database connection trait for interfacing with the quantum state storage
pub trait DatabaseConnection {
    fn query_quantum_states(&self, game_id: &Uuid) -> Result<HashMap<Uuid, QuantumState>, String>;
    fn save_quantum_state(&mut self, game_id: &Uuid, piece_id: &Uuid, state: &QuantumState) -> Result<(), String>;
}
