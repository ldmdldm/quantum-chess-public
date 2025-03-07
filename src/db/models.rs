use crate::db::schema::*;
use crate::game::state::GameStatus;
use diesel::{Insertable, Queryable, RunQueryDsl, QueryDsl, ExpressionMethods, PgConnection};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
use diesel::result::Error as DieselError;

// Game Models
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub status: String,
    pub white_player: Option<String>,
    pub black_player: Option<String>,
    pub white_to_move: bool,
    pub total_stake: i64,
    pub created_at: SystemTime,
    pub last_move_at: Option<SystemTime>,
    pub result_description: Option<String>,
    pub contract_address: Option<String>,
    pub board_fen: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = games)]
pub struct NewGame {
    pub id: Uuid,
    pub status: String,
    pub white_player: Option<String>,
    pub black_player: Option<String>,
    pub total_stake: i64,
    pub board_fen: String,
}

// Player Models
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub wallet_address: String,
    pub created_at: SystemTime,
    pub last_active_at: SystemTime,
    pub total_games: i32,
    pub total_wins: i32,
    pub total_stakes: i64,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = players)]
pub struct NewPlayer {
    pub wallet_address: String,
}

// Game Move Models
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct GameMove {
    pub id: i32,
    pub game_id: Uuid,
    pub player: String,
    pub move_notation: String,
    pub from_position: String,
    pub to_position: String,
    pub probability: f32,
    pub is_capture: bool,
    pub is_quantum: bool,
    pub transaction_id: Option<String>,
    pub position_hash: String,
    pub created_at: SystemTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = game_moves)]
pub struct NewGameMove {
    pub game_id: Uuid,
    pub player: String,
    pub move_notation: String,
    pub from_position: String,
    pub to_position: String,
    pub probability: f32,
    pub is_capture: bool,
    pub is_quantum: bool,
    pub transaction_id: Option<String>,
    pub position_hash: String,
}

// Quantum State Models
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct QuantumState {
    pub id: i32,
    pub game_id: Uuid,
    pub piece_type: String,
    pub position: String,
    pub superpositions: serde_json::Value,
    pub entanglements: serde_json::Value,
    pub measurement_probability: f32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = quantum_states)]
pub struct NewQuantumState {
    pub game_id: Uuid,
    pub piece_type: String,
    pub position: String,
    pub superpositions: serde_json::Value,
    pub entanglements: serde_json::Value,
    pub measurement_probability: f32,
}

// Game Stake Models
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct GameStake {
    pub id: i32,
    pub game_id: Uuid,
    pub player: String,
    pub amount: i64,
    pub transaction_id: String,
    pub status: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = game_stakes)]
pub struct NewGameStake {
    pub game_id: Uuid,
    pub player: String,
    pub amount: i64,
    pub transaction_id: String,
    pub status: String,
}

// Game Repository - Functions to interact with the database

impl Game {
    pub fn create(conn: &mut PgConnection, new_game: NewGame) -> Result<Game, DieselError> {
        use crate::db::schema::games::dsl::*;
        
        diesel::insert_into(games)
            .values(&new_game)
            .get_result::<Game>(conn)
    }
    
    pub fn find_by_id(conn: &mut PgConnection, game_id: Uuid) -> Result<Game, DieselError> {
        use crate::db::schema::games::dsl::*;
        
        games.find(game_id).first::<Game>(conn)
    }
    
    pub fn find_active_games(conn: &mut PgConnection, limit: i64) -> Result<Vec<Game>, DieselError> {
        use crate::db::schema::games::dsl::*;
        
        games
            .filter(status.eq(GameStatus::Active.to_string()))
            .order(created_at.desc())
            .limit(limit)
            .load::<Game>(conn)
    }
    
    pub fn update_status(
        conn: &mut PgConnection,
        game_id: Uuid,
        new_status: GameStatus,
        description: Option<String>
    ) -> Result<Game, DieselError> {
        use crate::db::schema::games::dsl::*;
        
        diesel::update(games.find(game_id))
            .set((
                status.eq(new_status.to_string()),
                result_description.eq(description),
                last_move_at.eq(std::time::SystemTime::now()),
            ))
            .get_result::<Game>(conn)
    }
    
    pub fn add_player(
        conn: &mut PgConnection,
        game_id: Uuid,
        player_address: String,
        is_white: bool
    ) -> Result<Game, DieselError> {
        use crate::db::schema::games::dsl::*;
        
        let update = if is_white {
            diesel::update(games.find(game_id))
                .set(white_player.eq(Some(player_address)))
        } else {
            diesel::update(games.find(game_id))
                .set(black_player.eq(Some(player_address)))
        };
        
        update.get_result::<Game>(conn)
    }
    
    pub fn update_board(
        conn: &mut PgConnection,
        game_id: Uuid,
        new_fen: String,
        is_white_turn: bool
    ) -> Result<Game, DieselError> {
        use crate::db::schema::games::dsl::*;
        
        diesel::update(games.find(game_id))
            .set((
                board_fen.eq(new_fen),
                white_to_move.eq(is_white_turn),
                last_move_at.eq(std::time::SystemTime::now()),
            ))
            .get_result::<Game>(conn)
    }
}

impl Player {
    pub fn create(conn: &mut PgConnection, new_player: NewPlayer) -> Result<Player, DieselError> {
        use crate::db::schema::players::dsl::*;
        
        diesel::insert_into(players)
            .values(&new_player)
            .get_result::<Player>(conn)
    }
    
    pub fn find_by_address(conn: &mut PgConnection, address: String) -> Result<Player, DieselError> {
        use crate::db::schema::players::dsl::*;
        
        players.filter(wallet_address.eq(address)).first::<Player>(conn)
    }
    
    pub fn update_stats(
        conn: &mut PgConnection,
        address: String,
        won: bool,
        stake_amount: i64
    ) -> Result<Player, DieselError> {
        use crate::db::schema::players::dsl::*;
        
        let player = Player::find_by_address(conn, address.clone())?;
        
        diesel::update(players.filter(wallet_address.eq(address)))
            .set((
                total_games.eq(player.total_games + 1),
                total_wins.eq(player.total_wins + if won { 1 } else { 0 }),
                total_stakes.eq(player.total_stakes + stake_amount),
                last_active_at.eq(std::time::SystemTime::now()),
            ))
            .get_result::<Player>(conn)
    }
}

impl GameMove {
    pub fn create(conn: &mut PgConnection, new_move: NewGameMove) -> Result<GameMove, DieselError> {
        use crate::db::schema::game_moves::dsl::*;
        
        diesel::insert_into(game_moves)
            .values(&new_move)
            .get_result::<GameMove>(conn)
    }
    
    pub fn find_by_game(conn: &mut PgConnection, g_id: Uuid) -> Result<Vec<GameMove>, DieselError> {
        use crate::db::schema::game_moves::dsl::*;
        
        game_moves
            .filter(game_id.eq(g_id))
            .order(created_at.asc())
            .load::<GameMove>(conn)
    }
    
    pub fn update_transaction(
        conn: &mut PgConnection,
        move_id: i32,
        tx_id: String
    ) -> Result<GameMove, DieselError> {
        use crate::db::schema::game_moves::dsl::*;
        
        diesel::update(game_moves.find(move_id))
            .set(transaction_id.eq(Some(tx_id)))
            .get_result::<GameMove>(conn)
    }
}

impl QuantumState {
    pub fn create(conn: &mut PgConnection, new_state: NewQuantumState) -> Result<QuantumState, DieselError> {
        use crate::db::schema::quantum_states::dsl::*;
        
        diesel::insert_into(quantum_states)
            .values(&new_state)
            .get_result::<QuantumState>(conn)
    }
    
    pub fn find_by_game(conn: &mut PgConnection, g_id: Uuid) -> Result<Vec<QuantumState>, DieselError> {
        use crate::db::schema::quantum_states::dsl::*;
        
        quantum_states
            .filter(game_id.eq(g_id))
            .load::<QuantumState>(conn)
    }
    
    pub fn find_by_position(
        conn: &mut PgConnection,
        g_id: Uuid,
        pos: String
    ) -> Result<QuantumState, DieselError> {
        use crate::db::schema::quantum_states::dsl::*;
        
        quantum_states
            .filter(game_id.eq(g_id))
            .filter(position.eq(pos))
            .first::<QuantumState>(conn)
    }
    
    pub fn update(
        conn: &mut PgConnection,
        state_id: i32,
        superpos: serde_json::Value,
        entang: serde_json::Value,
        prob: f32
    ) -> Result<QuantumState, DieselError> {
        use crate::db::schema::quantum_states::dsl::*;
        
        diesel::update(quantum_states.find(state_id))
            .set((
                superpositions.eq(superpos),
                entanglements.eq(entang),
                measurement_probability.eq(prob),
                updated_at.eq(std::time::SystemTime::now()),
            ))
            .get_result::<QuantumState>(conn)
    }
}

impl GameStake {
    pub fn create(conn: &mut PgConnection, new_stake: NewGameStake) -> Result<GameStake, DieselError> {
        use crate::db::schema::game_stakes::dsl::*;
        
        diesel::insert_into(game_stakes)
            .values(&new_stake)
            .get_result::<GameStake>(conn)
    }
    
    pub fn find_by_game(conn: &mut PgConnection, g_id: Uuid) -> Result<Vec<GameStake>, DieselError> {
        use crate::db::schema::game_stakes::dsl::*;
        
        game_stakes
            .filter(game_id.eq(g_id))
            .load::<GameStake>(conn)
    }
    
    pub fn find_by_player(conn: &mut PgConnection, player_addr: String) -> Result<Vec<GameStake>, DieselError> {
        use crate::db::schema::game_stakes::dsl::*;
        
        game_stakes
            .filter(player.eq(player_addr))
            .order(created_at.desc())
            .load::<GameStake>(conn)
    }
    
    pub fn update_status(
        conn: &mut PgConnection,
        stake_id: i32,
        new_status: String
    ) -> Result<GameStake, DieselError> {
        use crate::db::schema::game_stakes::dsl::*;
        
        diesel::update(game_stakes.find(stake_id))
            .set((
                status.eq(new_status),
                updated_at.eq(std::time::SystemTime::now()),
            ))
            .get_result::<GameStake>(conn)
    }
}
