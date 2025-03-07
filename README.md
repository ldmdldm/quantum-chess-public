# Quantum Chess

A blockchain-based chess game with quantum mechanics implemented on the Core blockchain.

## Overview

Quantum Chess introduces quantum mechanics principles to traditional chess, allowing pieces to exist in superposition and creating a new dimension of strategic gameplay. This project consists of:

1. **Smart Contract**: A Solidity contract deployed on the Core blockchain that manages game states, moves, and stakes
2. **Rust Client**: A game client built in Rust that interacts with the blockchain and provides the user interface

## Smart Contract Functionality

The Quantum Chess smart contract (`QuantumChess.sol`) enables the following features:

### Game Management
- **Create Game**: Start a new game with an initial stake
- **Join Game**: Allow an opponent to join by matching the stake
- **Timeout Game**: Claim victory if opponent is inactive for too long

### Quantum Chess Mechanics
- **Superposition Moves**: Record moves where pieces exist in multiple states simultaneously
- **Quantum Measurement**: Collapse superpositions when pieces interact
- **Probability Management**: Track probability distributions for piece positions

### Stake Management
- **Stake Funds**: Place ETH as stakes for game outcomes
- **Reward Distribution**: Distribute rewards to winners based on game outcomes

## Deploying the Smart Contract

### Prerequisites
- Node.js and npm installed
- Private key for a wallet with funds on Core Testnet

### Deployment Steps

1. **Set up environment**:
   ```bash
   cd deploy_tool
   npm install
   ```

2. **Configure environment variables**:
   Create a `.env` file with:
   ```
   CORE_PRIVATE_KEY=your_private_key_here
   CORE_BLOCKCHAIN_URL=https://rpc.test2.btcs.network
   ```

3. **Deploy the contract**:
   ```bash
   node deploy_quantum_chess.js
   # or
   cargo run --bin deploy_contract
   ```

4. **Verify deployment**:
   The script will output the deployed contract address, which will be saved to your `.env` file.

## Interacting with the Contract from Rust Client

The Rust client communicates with the deployed smart contract through the following components:

### Configuration

1. **Environment Setup**:
   Make sure your `.env` file contains:
   ```
   CORE_PRIVATE_KEY=your_private_key_here
   CORE_BLOCKCHAIN_URL=https://rpc.test2.btcs.network
   CORE_CONTRACT_ADDRESS=your_deployed_contract_address
   ```

2. **Blockchain Client Initialization**:
   ```rust
   use quantum_chess::blockchain::{BlockchainClient, CoreBlockchainClient};
   
   let client = CoreBlockchainClient::new_from_env()?;
   ```

### Game Creation and Joining

```rust
// Create a new game with 0.1 ETH stake
let game_id = client.create_game(player_address, 0.1)?;

// Join an existing game
client.join_game(game_id, opponent_address)?;
```

### Making Moves

```rust
// Record a standard move
client.record_move(game_id, "e2", "e4", 100)?; // 100% probability

// Record a quantum move (piece exists in two positions)
client.record_quantum_move(game_id, "e2", vec![("e3", 50), ("e4", 50)])?; // 50% in each position
```

### Retrieving Game State

```rust
// Get current game state
let game_state = client.get_game_state(game_id)?;

// Check if a position has a piece in superposition
let position_state = client.get_position_state(game_id, "e4")?;
```

### Handling Game Completion

```rust
// Finalize a game and distribute rewards
client.finalize_game(game_id, winner_address)?;

// Claim victory by timeout
client.timeout_game(game_id)?;
```

## Development Setup

1. **Install dependencies**:
   ```bash
   cargo build
   ```

2. **Connect to Core Testnet**:
   Make sure your wallet has testnet ETH, which you can get from the Core faucet.

3. **Run the Quantum Chess client**:
   ```bash
   cargo run
   ```

## Architecture

```
quantum-chess/
├── src/                      # Rust game client source code
│   ├── blockchain/           # Blockchain interaction layer
│   ├── game/                 # Game logic and quantum mechanics
│   └── api/                  # API for client-server communication
├── contracts/                # Smart contract code
│   └── QuantumChess.sol      # Main game contract
└── deploy_tool/              # Deployment utilities
    ├── src/                  # Rust deployment code
    └── deploy_quantum_chess.js # JavaScript deployment alternative
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

# Quantum Chess Backend

A backend implementation for a Quantum Chess game built on Core blockchain technology. This project combines quantum mechanics principles with traditional chess, creating a unique gaming experience where pieces can exist in superposition and become entangled.

## Features

- **Quantum Chess Rules**: Pieces can exist in superposition and become entangled
- **Blockchain Integration**: Game stakes and moves recorded on Core blockchain
- **RESTful API**: Comprehensive API for game creation, moves, and state tracking
- **Real-time Updates**: WebSocket support for live game updates
- **Quantum Simulation**: Accurate simulation of quantum mechanical principles

## Getting Started

### Prerequisites

- Rust (1.58+)
- PostgreSQL database
- Core blockchain account and API key

### Environment Configuration

Copy the example .env file and customize it with your settings:

```
# Server configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
SERVER_WORKERS=4
LOG_LEVEL=info

# Blockchain configuration
CORE_BLOCKCHAIN_URL=https://core-testnet.example.com
CORE_PRIVATE_KEY=your_private_key_here
CORE_CONTRACT_ADDRESS=your_contract_address
CORE_CHAIN_ID=1

# Database configuration
DATABASE_URL=postgres://username:password@localhost/quantum_chess

# Game configuration
MIN_STAKE_AMOUNT=1
MAX_STAKE_AMOUNT=100
DEFAULT_TIME_LIMIT=1800
MAX_SUPERPOSITION_PIECES=3
```

### Building and Running

#### Local Development

```bash
# Build the project
cargo build

# Run the server
cargo run
```

#### Production Deployment with Docker

```bash
# Build the Docker image
docker build -t quantum-chess .

# Run the container
docker run -p 8080:8080 --env-file .env quantum-chess
```

## API Documentation

### Game Endpoints

- `POST /api/games` - Create a new game
- `GET /api/games` - List active games
- `GET /api/games/{id}` - Get game details
- `POST /api/games/{id}/join` - Join an existing game
- `PUT /api/games/{id}/move` - Make a move

### Blockchain Endpoints

- `POST /api/blockchain/stake` - Stake funds for a game
- `POST /api/blockchain/unstake` - Withdraw staked funds
- `GET /api/blockchain/status` - Get blockchain status
- `GET /api/blockchain/transaction/{hash}` - Get transaction details

## Quantum Chess Rules

Quantum Chess extends traditional chess with quantum mechanics principles:

1. **Superposition**: Pieces can exist in multiple positions simultaneously
2. **Entanglement**: Pieces can become entangled, affecting each other's moves
3. **Measurement**: When interacting with a piece in superposition, the state collapses
4. **Probability**: Moves have associated probabilities based on quantum states

## Architecture

The application follows a modular architecture:

- **API Layer**: Handles HTTP requests and responses
- **Game Logic**: Manages game state and rules
- **Blockchain Layer**: Interfaces with Core blockchain
- **Quantum Simulation**: Handles quantum mechanics calculations
- **Database Layer**: Persists game data

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Core blockchain team for their support
- Quantum Chess researchers for theoretical foundations
