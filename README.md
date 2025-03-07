# 🔮 Quantum Chess: The Future of Strategic Gaming on Core Blockchain

![Quantum Chess Banner](https://via.placeholder.com/1200x300?text=Quantum+Chess+Revolution)

[![Built on Core](https://img.shields.io/badge/Built%20on-Core-blue)](https://coredao.org)
[![Smart Contract](https://img.shields.io/badge/Smart%20Contract-Solidity-363636)](https://soliditylang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 🌌 Project Overview

**Quantum Chess** revolutionizes the ancient game of chess by introducing quantum mechanics principles directly into gameplay, all secured and enhanced by Core blockchain technology. Experience a mind-bending fusion of strategic thinking and quantum uncertainty where pieces can exist in multiple states simultaneously, creating an entirely new dimension of gameplay that challenges even the most experienced chess masters.

> *"Not even the greatest chess masters can prepare for the quantum realm. Every move creates unlimited possibilities."*

## 🚀 Key Features

- **⚛️ Quantum Superposition**: Pieces can exist in multiple positions simultaneously until observed
- **🔄 Quantum Entanglement**: Link pieces together so actions on one affect the other regardless of board position
- **💰 On-Chain Staking**: Place bets on games with our native QCT (Quantum Chess Token)
- **🏆 Blockchain Tournaments**: Compete in decentralized tournaments with transparent scoring
- **🎭 Quantum Identity**: Each piece has its own NFT identity with unique quantum attributes
- **🔐 Zero-Knowledge Proofs**: Secure and private gameplay verification
- **📱 Cross-Platform Play**: Available on Web, iOS and Android with seamless blockchain integration

## 🧪 Technical Innovation

Quantum Chess introduces a paradigm shift in blockchain gaming by:

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
- Rust for high-performance game state computation
- Axum web framework for API endpoints
- PostgreSQL for persistent storage
- Redis for caching

### Blockchain
- Solidity smart contracts for game logic and token management
- Core blockchain for low gas fees and high throughput
- OpenZeppelin for contract security standards
- IPFS for decentralized storage of game assets

## 🌐 Smart Contract Deployment

Our contracts are deployed on the Core Blockchain Testnet:

```
QuantumChessToken: 0xQCT123456789abcdef0123456789abcdef0123456
QuantumChessGame: 0xQCG123456789abcdef0123456789abcdef0123456
QuantumChessNFT: 0xQCN123456789abcdef0123456789abcdef0123456
```

*Note: Replace the above addresses with the actual deployed contract addresses before submitting.*

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

- **Q2 2025**: Release of Quantum Chess Multiplayer Tournament Edition
- **Q3 2025**: Mobile apps for iOS and Android
- **Q4 2025**: Integration with Core mainnet
- **Q1 2026**: AI opponents trained on quantum gameplay
- **Q2 2026**: VR/AR enhanced quantum visualization

## 👥 Meet the Team

| Name | Role | Contact |
|------|------|---------|
| Alice Quantum | Game Theory & Quantum Algorithms | [@aliceq](https://github.com/aliceq) |
| Bob Blockchain | Smart Contract Engineer | [@bobb](https://github.com/bobb) |
| Charlie Chain | Frontend Developer | [@charliec](https://github.com/charliec) |
| Diana Distributed | Backend & Infrastructure | [@dianad](https://github.com/dianad) |

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Website](https://quantum-chess.example.com)
- [Documentation](https://docs.quantum-chess.example.com)
- [Core Blockchain](https://coredao.org)
- [Discord Community](https://discord.gg/quantumchess)

---

<p align="center">
  <img src="https://via.placeholder.com/100x100?text=QC" alt="Quantum Chess Logo" width="100">
  <br>
  <em>Built with ♟️ and ⚛️ for the Core Global Gaming Hackathon 2025</em>
</p>

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
