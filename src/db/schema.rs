// @generated automatically by Diesel CLI.

diesel::table! {
    game_moves (id) {
        id -> Int4,
        game_id -> Uuid,
        player -> Varchar,
        move_notation -> Varchar,
        from_position -> Varchar,
        to_position -> Varchar,
        probability -> Float4,
        is_capture -> Bool,
        is_quantum -> Bool,
        transaction_id -> Nullable<Varchar>,
        position_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    game_stakes (id) {
        id -> Int4,
        game_id -> Uuid,
        player -> Varchar,
        amount -> Int8,
        transaction_id -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    games (id) {
        id -> Uuid,
        status -> Varchar,
        white_player -> Nullable<Varchar>,
        black_player -> Nullable<Varchar>,
        white_to_move -> Bool,
        total_stake -> Int8,
        created_at -> Timestamp,
        last_move_at -> Nullable<Timestamp>,
        result_description -> Nullable<Text>,
        contract_address -> Nullable<Varchar>,
        board_fen -> Text,
    }
}

diesel::table! {
    players (wallet_address) {
        wallet_address -> Varchar,
        created_at -> Timestamp,
        last_active_at -> Timestamp,
        total_games -> Int4,
        total_wins -> Int4,
        total_stakes -> Int8,
    }
}

diesel::table! {
    quantum_states (id) {
        id -> Int4,
        game_id -> Uuid,
        piece_type -> Varchar,
        position -> Varchar,
        superpositions -> Jsonb,
        entanglements -> Jsonb,
        measurement_probability -> Float4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(game_moves -> games (game_id));
diesel::joinable!(game_stakes -> games (game_id));
diesel::joinable!(quantum_states -> games (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    game_moves,
    game_stakes,
    games,
    players,
    quantum_states,
);
