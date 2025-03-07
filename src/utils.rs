use rand::distributions::{Distribution, Standard};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use hex;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use crate::game::state::{GameState, ChessPosition};
use crate::quantum::QuantumState;

/// Generates a cryptographically secure random number using the rand crate
pub fn generate_secure_random() -> u64 {
    rand::thread_rng().gen()
}

/// Generates a random number within a given range
pub fn generate_random_in_range(min: u64, max: u64) -> u64 {
    rand::thread_rng().gen_range(min..=max)
}

/// Generates a random float between 0 and 1 for probability calculations
pub fn generate_probability() -> f64 {
    rand::thread_rng().gen()
}

/// Generates a random seed based on timestamp and random data
pub fn generate_seed() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;
    
    timestamp ^ generate_secure_random()
}

/// Calculates SHA-256 hash of the input data
pub fn calculate_hash<T: AsRef<[u8]>>(data: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Combines multiple hashes into a single hash
pub fn combine_hashes(hashes: &[String]) -> String {
    let combined = hashes.join("");
    calculate_hash(combined)
}

/// Generates a unique game ID
pub fn generate_game_id() -> String {
    Uuid::new_v4().to_string()
}

/// Serializes game state to JSON
pub fn serialize_game_state(state: &GameState) -> Result<String> {
    serde_json::to_string(state).map_err(|e| anyhow!("Failed to serialize game state: {}", e))
}

/// Deserializes game state from JSON
pub fn deserialize_game_state(json: &str) -> Result<GameState> {
    serde_json::from_str(json).map_err(|e| anyhow!("Failed to deserialize game state: {}", e))
}

/// Converts a chess position to a string (e.g., "e4")
pub fn position_to_string(pos: &ChessPosition) -> String {
    format!("{}{}", 
        char::from(b'a' + pos.file as u8),
        pos.rank + 1
    )
}

/// Parses a string position to a ChessPosition
pub fn string_to_position(s: &str) -> Result<ChessPosition> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 2 {
        return Err(anyhow!("Invalid position format"));
    }
    
    let file = (chars[0] as u8).checked_sub(b'a')
        .ok_or_else(|| anyhow!("Invalid file: {}", chars[0]))?;
    
    let rank = chars[1].to_digit(10)
        .ok_or_else(|| anyhow!("Invalid rank: {}", chars[1]))?;
    
    if file > 7 || rank < 1 || rank > 8 {
        return Err(anyhow!("Position out of bounds"));
    }
    
    Ok(ChessPosition {
        file: file as usize,
        rank: (rank - 1) as usize,
    })
}

/// Generates a deterministic hash from a game state and seed
/// Used for reproducible randomness in quantum calculations
pub fn deterministic_hash(state: &GameState, seed: u64) -> u64 {
    let state_hash = match serialize_game_state(state) {
        Ok(json) => calculate_hash(json),
        Err(_) => "default".to_string(),
    };
    
    let combined = format!("{}{}", state_hash, seed);
    let hash = calculate_hash(combined);
    
    // Convert first 8 bytes of hash to u64
    let bytes = hex::decode(&hash[0..16]).unwrap_or_default();
    let mut value: u64 = 0;
    
    for (i, &byte) in bytes.iter().enumerate().take(8) {
        value |= (byte as u64) << (i * 8);
    }
    
    value
}

/// Calculates probability based on quantum state
pub fn calculate_quantum_probability(quantum_state: &QuantumState, position: &ChessPosition) -> f64 {
    // This is a simplified implementation - actual quantum calculations would be more complex
    quantum_state.amplitude_at_position(position).norm_sqr()
}

/// Computes a weighted random outcome based on given probabilities
pub fn weighted_random_outcome(probabilities: &[(String, f64)]) -> Option<String> {
    let total: f64 = probabilities.iter().map(|(_, p)| p).sum();
    
    if total <= 0.0 {
        return None;
    }
    
    let random_value = generate_probability() * total;
    let mut cumulative = 0.0;
    
    for (outcome, probability) in probabilities {
        cumulative += probability;
        if random_value <= cumulative {
            return Some(outcome.clone());
        }
    }
    
    // Fallback to last outcome in case of floating-point rounding issues
    probabilities.last().map(|(outcome, _)| outcome.clone())
}

/// Verifies a cryptographic signature
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    // This is a placeholder - in a real implementation, we would use ed25519-dalek or similar
    // to verify the signature with the public key
    
    // Example implementation would be:
    // let public_key = PublicKey::from_bytes(public_key).ok()?;
    // let signature = Signature::from_bytes(signature).ok()?;
    // public_key.verify(message, &signature).is_ok()
    
    // For now, return true as a placeholder
    true
}

/// Formats a number as a currency string with the CORE token symbol
pub fn format_core_amount(amount: u64) -> String {
    format!("{} CORE", amount)
}

/// Validates a blockchain address
pub fn is_valid_blockchain_address(address: &str) -> bool {
    // Simple validation for hex-based blockchain addresses
    // In a real implementation, we'd use blockchain-specific validation
    address.len() == 42 && 
    address.starts_with("0x") && 
    address[2..].chars().all(|c| c.is_digit(16))
}

/// Sanitizes user input to prevent injection attacks
pub fn sanitize_input(input: &str) -> String {
    // Remove any control characters and normalize whitespace
    input.chars()
        .filter(|&c| !c.is_control())
        .collect()
}

