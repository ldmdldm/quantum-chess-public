use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

use crate::game::state::GameState;
use crate::quantum;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantumMoveRequest {
    pub game_id: uuid::Uuid,
    pub piece_id: String,
    pub from_position: String,
    pub to_positions: Vec<String>,
    pub probabilities: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntanglementRequest {
    pub game_id: uuid::Uuid,
    pub piece_ids: Vec<String>,
}

/// Register quantum-related routes
pub fn configure() -> Scope {
    web::scope("/quantum")
        .route("/superposition", web::post().to(create_superposition))
        .route("/entanglement", web::post().to(create_entanglement))
        .route("/collapse/{game_id}/{piece_id}", web::post().to(collapse_quantum_state))
        .route("/probability/{game_id}/{from}/{to}", web::get().to(get_move_probability))
        .route("/states/{game_id}", web::get().to(get_quantum_states))
}

/// Creates a superposition for a chess piece
async fn create_superposition(
    move_req: web::Json<QuantumMoveRequest>,
) -> HttpResponse {
    // Validate the request
    if move_req.to_positions.len() != move_req.probabilities.len() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "The number of positions must match the number of probabilities"
        }));
    }

    // Validate that probabilities sum to 1.0 (with some floating point tolerance)
    let sum: f32 = move_req.probabilities.iter().sum();
    if (sum - 1.0).abs() > 0.001 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Probabilities must sum to 1.0"
        }));
    }

    // Here we would interact with the quantum game logic
    // This is a placeholder - real implementation would use the quantum module
    match quantum::create_superposition(
        move_req.game_id,
        &move_req.piece_id,
        &move_req.from_position,
        &move_req.to_positions,
        &move_req.probabilities,
    ) {
        Ok(state) => HttpResponse::Ok().json(state),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to create superposition: {}", e)
        })),
    }
}

/// Creates entanglement between two or more chess pieces
async fn create_entanglement(
    req: web::Json<EntanglementRequest>,
) -> HttpResponse {
    // Validate request - need at least two pieces for entanglement
    if req.piece_ids.len() < 2 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Entanglement requires at least two pieces"
        }));
    }

    // This is a placeholder - real implementation would use the quantum module
    match quantum::create_entanglement(req.game_id, &req.piece_ids) {
        Ok(states) => HttpResponse::Ok().json(states),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to create entanglement: {}", e)
        })),
    }
}

/// Collapses the quantum state of a piece
async fn collapse_quantum_state(
    path: web::Path<(uuid::Uuid, String)>,
) -> HttpResponse {
    let (game_id, piece_id) = path.into_inner();

    // This is a placeholder - real implementation would use the quantum module
    match quantum::collapse_state(game_id, &piece_id) {
        Ok(state) => HttpResponse::Ok().json(state),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to collapse quantum state: {}", e)
        })),
    }
}

/// Gets the probability of a specific move
async fn get_move_probability(
    path: web::Path<(uuid::Uuid, String, String)>,
) -> HttpResponse {
    let (game_id, from, to) = path.into_inner();

    // This is a placeholder - real implementation would use the quantum module
    match quantum::calculate_move_probability(game_id, &from, &to) {
        Ok(probability) => HttpResponse::Ok().json(serde_json::json!({
            "probability": probability
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to get move probability: {}", e)
        })),
    }
}

/// Gets all quantum states for a game
async fn get_quantum_states(
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let game_id = path.into_inner();

    // This is a placeholder - real implementation would use the quantum module
    match quantum::get_game_quantum_states(game_id) {
        Ok(states) => HttpResponse::Ok().json(states),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to get quantum states: {}", e)
        })),
    }
}

