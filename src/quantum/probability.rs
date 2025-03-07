use std::cmp;

/// Defines probability zones for quantum moves
#[derive(Debug, Clone, Copy)]
pub enum ProbabilityZone {
    VeryLow,   // 0-20%
    Low,       // 20-40%
    Medium,    // 40-60%
    High,      // 60-80%
    VeryHigh,  // 80-100%
}

/// Probability calculation parameters
pub struct ProbabilityParams {
    pub base_probability: f64,
    pub stake_modifier: f64,
    pub position_modifier: f64,
    pub entanglement_modifier: f64,
}

impl Default for ProbabilityParams {
    fn default() -> Self {
        Self {
            base_probability: 0.5,
            stake_modifier: 0.0,
            position_modifier: 0.0,
            entanglement_modifier: 0.0,
        }
    }
}

/// Constants for probability calculations
const MAX_STAKE_BONUS: f64 = 0.3;  // Maximum bonus from stakes
const MIN_PROBABILITY: f64 = 0.05; // Minimum probability regardless of factors
const MAX_PROBABILITY: f64 = 0.95; // Maximum probability regardless of factors

/// Calculate probability based on stake amount and other factors
pub fn calculate_probability(stake_amount: u64, position: &str, zone: ProbabilityZone) -> f64 {
    // Base probability determined by zone
    let base = match zone {
        ProbabilityZone::VeryLow => 0.1,
        ProbabilityZone::Low => 0.3,
        ProbabilityZone::Medium => 0.5,
        ProbabilityZone::High => 0.7,
        ProbabilityZone::VeryHigh => 0.9,
    };
    
    // Calculate stake modifier (more stake = slightly higher probability)
    let stake_modifier = calculate_stake_modifier(stake_amount);
    
    // Calculate position modifier (based on chess position value)
    let position_modifier = calculate_position_modifier(position);
    
    // Combine all modifiers
    let final_probability = (base + stake_modifier + position_modifier)
        .min(MAX_PROBABILITY)
        .max(MIN_PROBABILITY);
        
    final_probability
}

/// Calculate modifier based on stake amount
fn calculate_stake_modifier(stake_amount: u64) -> f64 {
    // Example implementation - higher stakes give better probability up to a limit
    let stake_normalized = cmp::min(stake_amount, 100) as f64 / 100.0;
    stake_normalized * MAX_STAKE_BONUS
}

/// Calculate modifier based on chess position
fn calculate_position_modifier(position: &str) -> f64 {
    // Example implementation - central positions might have better probability
    if position.len() < 2 {
        return 0.0;
    }
    
    // Get file (column) and rank (row)
    let file = position.chars().next().unwrap();
    let rank = position.chars().nth(1).unwrap();
    
    // Central positions (d4, d5, e4, e5) get a small bonus
    if (file == 'd' || file == 'e') && (rank == '4' || rank == '5') {
        return 0.05;
    }
    
    0.0
}

/// Determine probability zone based on various factors
pub fn determine_probability_zone(
    piece_type: &str,
    is_capture: bool,
    stake_amount: u64,
) -> ProbabilityZone {
    // Different piece types might have different base probabilities
    let base_zone = match piece_type {
        "pawn" => ProbabilityZone::High,
        "knight" | "bishop" => ProbabilityZone::Medium,
        "rook" => ProbabilityZone::Medium,
        "queen" => ProbabilityZone::Low,    // More valuable pieces have lower probability for balance
        "king" => ProbabilityZone::VeryLow, // King moves are very constrained
        _ => ProbabilityZone::Medium,
    };
    
    // Captures are generally more risky
    if is_capture {
        match base_zone {
            ProbabilityZone::VeryHigh => ProbabilityZone::High,
            ProbabilityZone::High => ProbabilityZone::Medium,
            ProbabilityZone::Medium => ProbabilityZone::Low,
            ProbabilityZone::Low => ProbabilityZone::VeryLow,
            ProbabilityZone::VeryLow => ProbabilityZone::VeryLow,
        }
    } else {
        // High stakes can improve probability zone by one level
        if stake_amount > 50 {
            match base_zone {
                ProbabilityZone::VeryLow => ProbabilityZone::Low,
                ProbabilityZone::Low => ProbabilityZone::Medium,
                ProbabilityZone::Medium => ProbabilityZone::High,
                ProbabilityZone::High => ProbabilityZone::VeryHigh,
                ProbabilityZone::VeryHigh => ProbabilityZone::VeryHigh,
            }
        } else {
            base_zone
        }
    }
}

/// Calculate success probability for a quantum move
pub fn calculate_move_probability(
    piece_type: &str,
    from_position: &str,
    to_position: &str,
    is_capture: bool,
    stake_amount: u64,
    is_entangled: bool,
) -> f64 {
    // Determine the base probability zone
    let zone = determine_probability_zone(piece_type, is_capture, stake_amount);
    
    // Calculate the base probability
    let mut probability = calculate_probability(stake_amount, to_position, zone);
    
    // Apply entanglement modifier if needed
    if is_entangled {
        probability *= 0.8; // Entangled pieces have reduced probability
    }
    
    probability
}

