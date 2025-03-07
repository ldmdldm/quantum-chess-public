use actix_web::{web, HttpResponse, Responder, get, post, put};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::state::{GameState, GameMove, GameError, GameStatus};
use crate::blockchain::verify_signature;

#[derive(Serialize, Deserialize)]
pub struct CreateGameRequest {
    player_address: String,
    stake_amount: u64,
    signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct JoinGameRequest {
    player_address: String,
    game_id: Uuid,
    stake_amount: u64,
    signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct MakeMoveRequest {
    game_id: Uuid,
    player_address: String,
    from_position: String,
    to_position: String,
    signature: String,
}

#[derive(Serialize)]
pub struct GameResponse {
    game_id: Uuid,
    status: GameStatus,
    white_player: String,
    black_player: Option<String>,
    current_turn: String,
    board_state: String,
    quantum_state: Vec<QuantumStateInfo>,
    stake_info: StakeInfo,
}

#[derive(Serialize)]
pub struct QuantumStateInfo {
    piece: String,
    positions: Vec<String>,
    probabilities: Vec<f64>,
}

#[derive(Serialize)]
pub struct StakeInfo {
    white_stake: u64,
    black_stake: u64,
    pool_amount: u64,
}

/// Configure game-related routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_game)
       .service(join_game)
       .service(make_move)
       .service(get_game)
       .service(list_active_games);
}

#[post("/games")]
async fn create_game(
    game_req: web::Json<CreateGameRequest>,
    game_state: web::Data<GameState>,
) -> impl Responder {
    // Verify signature
    if !verify_signature(&game_req.player_address, &game_req.signature) {
        return HttpResponse::Unauthorized().json(GameError::new("Invalid signature"));
    }

    // Verify stake amount
    if game_req.stake_amount == 0 {
        return HttpResponse::BadRequest().json(GameError::new("Stake amount must be greater than zero"));
    }

    // Create new game
    match game_state.create_game(&game_req.player_address, game_req.stake_amount).await {
        Ok(game) => HttpResponse::Created().json(game),
        Err(e) => HttpResponse::InternalServerError().json(GameError::new(&e.to_string())),
    }
}

#[post("/games/{game_id}/join")]
async fn join_game(
    path: web::Path<Uuid>,
    join_req: web::Json<JoinGameRequest>,
    game_state: web::Data<GameState>,
) -> impl Responder {
    let game_id = path.into_inner();
    
    // Verify signature
    if !verify_signature(&join_req.player_address, &join_req.signature) {
        return HttpResponse::Unauthorized().json(GameError::new("Invalid signature"));
    }

    // Verify stake amount matches the game's required stake
    match game_state.join_game(game_id, &join_req.player_address, join_req.stake_amount).await {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(e) => HttpResponse::BadRequest().json(GameError::new(&e.to_string())),
    }
}

#[put("/games/{game_id}/move")]
async fn make_move(
    path: web::Path<Uuid>,
    move_req: web::Json<MakeMoveRequest>,
    game_state: web::Data<GameState>,
) -> impl Responder {
    let game_id = path.into_inner();
    
    // Verify signature
    if !verify_signature(&move_req.player_address, &move_req.signature) {
        return HttpResponse::Unauthorized().json(GameError::new("Invalid signature"));
    }

    // Perform the move
    let game_move = GameMove {
        player: move_req.player_address.clone(),
        from: move_req.from_position.clone(),
        to: move_req.to_position.clone(),
    };

    match game_state.make_move(game_id, game_move).await {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(e) => HttpResponse::BadRequest().json(GameError::new(&e.to_string())),
    }
}

#[get("/games/{game_id}")]
async fn get_game(
    path: web::Path<Uuid>,
    game_state: web::Data<GameState>,
) -> impl Responder {
    let game_id = path.into_inner();
    
    match game_state.get_game(game_id).await {
        Some(game) => HttpResponse::Ok().json(game),
        None => HttpResponse::NotFound().json(GameError::new("Game not found")),
    }
}

#[get("/games")]
async fn list_active_games(
    game_state: web::Data<GameState>,
) -> impl Responder {
    let games = game_state.list_active_games().await;
    HttpResponse::Ok().json(games)
}

