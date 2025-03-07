# 🔮 Quantum Chess: Quantum Gaming on Core Blockchain

[![Built on Core](https://img.shields.io/badge/Built%20on-Core-blue)](https://coredao.org)
[![Smart Contract](https://img.shields.io/badge/Smart%20Contract-Solidity-363636)](https://soliditylang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 🌌 TL,DR

**Quantum Chess**  takes the ancient game of chess and introduces quantum mechanics principles directly into gameplay on Core blockchain. You can experience a fusion of strategic thinking and quantum uncertainty where pieces can exist in multiple states simultaneously and create an entirely new dimension of gameplay that challenges even the most experienced chess masters.

> *"Not even the greatest chess masters can prepare for the quantum rules."*


## 🌌 Quantum Chess Rules

Quantum Chess merges classical chess with quantum mechanics to create a revolutionary gaming experience. Understanding these rules is essential for mastering the game.

### Fundamental Principles

#### ⚛️ Superposition
- **Definition**: A piece in superposition exists in multiple board positions simultaneously
- **Creation**: Use a "Q-Move" action instead of a standard move
- **Limitation**: Each player may have a maximum of 3 pieces in superposition at any time
- **Visualization**: Pieces in superposition appear semi-transparent on all potential positions

#### 🔄 Entanglement
- **Definition**: Two pieces become linked so that observing one affects the other instantly
- **Creation**: Use the "Entangle" action on two of your pieces
- **Effect**: When one entangled piece is forced to collapse, its partner also collapses according to correlated probabilities
- **Limitation**: Each piece can be entangled with only one other piece

#### 📊 Collapse
- **Definition**: When a piece in superposition is captured or performs a capture, it "collapses" to a single position
- **Trigger**: Interaction with another piece, either by capturing or being captured
- **Probability**: The position it collapses to is determined by quantum probability calculations
- **Chain Reaction**: May trigger collapse of entangled pieces

### Game Mechanics

#### Setup and Board
- Standard 8×8 chess board with traditional starting positions
- Each player begins with standard chess pieces
- Quantum state tracker displays at the side of the board

#### Turn Structure
1. **Move Phase**: Choose to make either:
   - A Classical Move (standard chess rules)
   - A Quantum Move (place a piece in superposition)
   - An Entanglement Action (link two of your pieces)
   
2. **Measurement Phase**:
   - If your move creates an attack on an opponent's superposed piece, measurement occurs
   - If your move places a piece in the same position as an opponent's superposed piece, measurement occurs
   - All affected pieces collapse according to quantum probabilities

#### Special Quantum Moves

1. **Q-Split** (Superposition Creation):
   - Move a piece to two possible positions simultaneously
   - Each position must be a legal move for that piece
   - Assign probability distributions across positions (default: equal probability)

2. **Q-Merge**:
   - Voluntarily collapse your own superposed piece
   - You select which valid position the piece collapses to

3. **Q-Link** (Entanglement):
   - Connect two of your pieces that aren't in superposition
   - Creates quantum relationship between pieces
   - Can be performed instead of a movement

#### Winning the Game

- **Checkmate**: Place opponent's king in checkmate across all possible states
- **Quantum Checkmate**: Create a superposition where all possible states lead to checkmate
- **Resignation**: A player may resign at any time
- **Time Control**: Standard chess clock rules apply

### Strategy Tips

1. **Uncertainty as Defense**: Place important pieces in superposition to make them harder to capture
2. **Quantum Fork**: Create superpositions that threaten multiple pieces simultaneously
3. **Entanglement Trap**: Entangle pieces to create correlated defenses or attacks
4. **Measurement Forcing**: Strategically force measurements to collapse opponent's quantum advantage
5. **Probability Shifting**: Advanced players can manipulate collapse probabilities through careful setup

### Differences from Classical Chess

1. **Non-deterministic outcomes**: Captures may succeed or fail based on quantum probability
2. **Expanded decision space**: More possible moves and strategies
3. **Incomplete information**: Opponents' superposed pieces have uncertain positions
4. **Multi-dimensional thinking**: Must consider all possible board states simultaneously

The integration with Core blockchain ensures all quantum states and probabilities are provably fair and cannot be manipulated.

## 🧪 Technical Innovation

1. Implementing quantum algorithms directly in game mechanics
2. Using smart contracts to enforce quantum rules
3. Creating a provably fair random system based on quantum entropy
4. Generating game states that can exist across multiple blockchain states simultaneously

## 🛠️ Technical Stack

### Frontend
- React/Next.js for web client
- React Native for mobile apps
- Three.js for 3D board visualization
- Ethers.js for blockchain integration

### Backend
- Rust for game state computation
- Axum web framework for API endpoints
- PostgreSQL for persistent storage
- Redis for caching

### Blockchain
- Solidity smart contracts for game logic and token management
- Core blockchain for low gas fees and high throughput
- OpenZeppelin for contract security standards
- IPFS for decentralized storage of game assets

## 🌐 Testnet Deployment

Our contracts are deployed on the Core Blockchain Testnet:

```
QuantumChessGame: 0x6ECbc602615Ecb1051f9AC3fD984840795dDd9D6
```

## 🚦 Getting Started

### Prerequisites
- Node.js (v16+)
- Rust (latest stable)
- Docker and Docker Compose
- Core Blockchain Wallet (MetaMask configured for Core)

### Installation

1. Clone the repository
```bash
git clone https://github.com/yourusername/quantum-chess.git
cd quantum-chess
```

2. Install dependencies
```bash
npm install
cd server && cargo build --release && cd ..
```

3. Set up environment variables
```bash
cp .env.example .env
# Edit .env with your configuration
```

4. Run the development environment
```bash
docker-compose up -d
npm run dev
```

5. Access the application
```
Web: http://localhost:3000
API: http://localhost:8000
```

## 🎮 Gameplay Instructions

1. **Connect Wallet**: Link your Core blockchain wallet to stake QCT tokens
2. **Choose Game Mode**: Select from Classic, Quantum Casual, or Tournament
3. **Make Quantum Moves**: Use quantum mechanics to your advantage:
   - **Q-Shift**: Place a piece in quantum superposition
   - **Entangle**: Connect two pieces across the board
   - **Measure**: Force a quantum collapse to determine final piece positions
4. **Win Conditions**: Checkmate the opponent's king across all quantum states

## 🔮 Future Roadmap

- **Q2 2025**: Integration with Core mainnet
- **Q3 2025**: Start of Quantum Chess Tournament Edition Mobile apps for iOS and Android
- **Q2 2025**: Mobile apps for iOS and Android
- **Q1 2026**: AI opponents trained on quantum gameplay

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
ility**: Moves have associated probabilities based on quantum states


## Acknowledgments

- Core blockchain team for their support
- Quantum Chess researchers for theoretical foundations

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  <img src="https://via.placeholder.com/100x100?text=QC" alt="Quantum Chess Logo" width="100">
  <br>
  <em>Built with ♟️ and ⚛️ for the Core Global Gaming Hackathon 2025</em>
</p>
