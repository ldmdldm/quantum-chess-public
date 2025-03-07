-- Up migration
CREATE TABLE IF NOT EXISTS games (
  id UUID PRIMARY KEY,
  status VARCHAR(20) NOT NULL,
  white_player VARCHAR(50),
  black_player VARCHAR(50),
  white_to_move BOOLEAN NOT NULL DEFAULT TRUE,
  total_stake BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  last_move_at TIMESTAMP WITH TIME ZONE,
  result_description TEXT,
  contract_address VARCHAR(50),
  board_fen TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS players (
  wallet_address VARCHAR(50) PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  last_active_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  total_games INT NOT NULL DEFAULT 0,
  total_wins INT NOT NULL DEFAULT 0,
  total_stakes BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS game_moves (
  id SERIAL PRIMARY KEY,
  game_id UUID NOT NULL REFERENCES games(id),
  player VARCHAR(50) NOT NULL,
  move_notation VARCHAR(10) NOT NULL,
  from_position VARCHAR(3) NOT NULL,
  to_position VARCHAR(3) NOT NULL,
  probability FLOAT NOT NULL,
  is_capture BOOLEAN NOT NULL DEFAULT FALSE,
  is_quantum BOOLEAN NOT NULL DEFAULT FALSE,
  transaction_id VARCHAR(66),
  position_hash VARCHAR(66) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_game FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS quantum_states (
  id SERIAL PRIMARY KEY,
  game_id UUID NOT NULL REFERENCES games(id),
  piece_type VARCHAR(10) NOT NULL,
  position VARCHAR(3) NOT NULL,
  superpositions JSONB NOT NULL DEFAULT '{}',
  entanglements JSONB NOT NULL DEFAULT '[]',
  measurement_probability FLOAT NOT NULL DEFAULT 1.0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_game FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS game_stakes (
  id SERIAL PRIMARY KEY,
  game_id UUID NOT NULL REFERENCES games(id),
  player VARCHAR(50) NOT NULL,
  amount BIGINT NOT NULL,
  transaction_id VARCHAR(66) NOT NULL,
  status VARCHAR(20) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_game FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_games_status ON games(status);
CREATE INDEX idx_game_moves_game_id ON game_moves(game_id);
CREATE INDEX idx_quantum_states_game_id ON quantum_states(game_id);
CREATE INDEX idx_game_stakes_game_id ON game_stakes(game_id);
CREATE INDEX idx_game_stakes_player ON game_stakes(player);

-- Down migration
-- CREATE TABLE IF NOT EXISTS _migrations (
--   DOWN SQL GOES HERE
-- );
