// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

/**
 * @title QuantumChess
 * @dev Implementation of Quantum Chess on blockchain
 * @author Quantum Chess Team
 */
contract QuantumChess is Ownable, ReentrancyGuard {
    using SafeMath for uint256;

    // Game status
    enum GameStatus { 
        CREATED,     // Game created, waiting for second player
        ACTIVE,      // Game active, waiting for moves
        COMPLETED,   // Game completed with a winner
        DRAW,        // Game ended in a draw
        TIMEOUT      // Game ended due to timeout
    }

    // Quantum Move Types
    enum MoveType { 
        STANDARD,     // Regular chess move
        SPLIT,        // Quantum split move
        MERGE,        // Quantum merge move
        OBSERVE,      // Observation (collapses superposition)
        ENTANGLE      // Entangle two pieces
    }

    // Represents a single game
    struct Game {
        address creator;         // Player who created the game
        address opponent;        // Player who joined the game
        uint256 stake;           // Amount staked on the game
        GameStatus status;       // Current status of the game
        string gameState;        // Encoded game state (JSON string)
        uint256 lastMoveTime;    // Timestamp of the last move
        uint256 timeoutDuration; // Duration after which game times out
        address winner;          // Address of the winner (if any)
    }

    // Represents a move in the game
    struct Move {
        address player;          // Player making the move
        MoveType moveType;       // Type of move
        string moveData;         // Encoded move data (JSON string)
        uint256 probability;     // Probability (0-100) for quantum moves
        uint256 timestamp;       // When the move was made
    }

    // Contract state variables
    uint256 public nextGameId;                     // ID to assign to the next game
    mapping(uint256 => Game) public games;         // Game ID to Game
    mapping(uint256 => Move[]) public gameMoves;   // Game ID to array of moves
    mapping(address => uint256[]) public playerGames; // Player address to their game IDs

    // Events
    event GameCreated(uint256 gameId, address creator, uint256 stake);
    event GameJoined(uint256 gameId, address opponent);
    event MoveMade(uint256 gameId, address player, MoveType moveType, uint256 probability);
    event GameCompleted(uint256 gameId, address winner, uint256 prize);
    event GameDraw(uint256 gameId, uint256 returnedStake);
    event GameTimedOut(uint256 gameId, address winner);

    /**
     * @dev Constructor
     */
    constructor() {
        nextGameId = 1;
    }

    /**
     * @dev Create a new game
     * @param timeoutDuration Duration in seconds after which game can be timed out
     * @param initialState Initial game state encoded as JSON string
     */
    function createGame(uint256 timeoutDuration, string calldata initialState) external payable returns (uint256) {
        require(msg.value > 0, "Stake amount must be greater than 0");
        require(timeoutDuration >= 1 hours, "Timeout must be at least 1 hour");

        uint256 gameId = nextGameId++;
        
        games[gameId] = Game({
            creator: msg.sender,
            opponent: address(0),
            stake: msg.value,
            status: GameStatus.CREATED,
            gameState: initialState,
            lastMoveTime: block.timestamp,
            timeoutDuration: timeoutDuration,
            winner: address(0)
        });

        playerGames[msg.sender].push(gameId);
        
        emit GameCreated(gameId, msg.sender, msg.value);
        
        return gameId;
    }

    /**
     * @dev Join an existing game
     * @param gameId ID of the game to join
     */
    function joinGame(uint256 gameId) external payable nonReentrant {
        Game storage game = games[gameId];
        
        require(game.creator != address(0), "Game does not exist");
        require(game.status == GameStatus.CREATED, "Game is not open for joining");
        require(game.creator != msg.sender, "Creator cannot join their own game");
        require(msg.value == game.stake, "Must match the creator's stake");

        game.opponent = msg.sender;
        game.status = GameStatus.ACTIVE;
        game.lastMoveTime = block.timestamp;

        playerGames[msg.sender].push(gameId);
        
        emit GameJoined(gameId, msg.sender);
    }

    /**
     * @dev Record a move
     * @param gameId ID of the game
     * @param moveType Type of move
     * @param moveData Encoded move data as JSON string
     * @param probability Probability for quantum moves (0-100)
     */
    function recordMove(
        uint256 gameId, 
        MoveType moveType, 
        string calldata moveData, 
        uint256 probability
    ) external {
        Game storage game = games[gameId];
        
        require(game.status == GameStatus.ACTIVE, "Game is not active");
        require(msg.sender == game.creator || msg.sender == game.opponent, "Not a player in this game");
        
        // Update last move time
        game.lastMoveTime = block.timestamp;
        
        // Store the move
        gameMoves[gameId].push(Move({
            player: msg.sender,
            moveType: moveType,
            moveData: moveData,
            probability: probability,
            timestamp: block.timestamp
        }));
        
        emit MoveMade(gameId, msg.sender, moveType, probability);
    }

    /**
     * @dev Update game state
     * @param gameId ID of the game
     * @param newState New encoded game state as JSON string
     */
    function updateGameState(uint256 gameId, string calldata newState) external {
        Game storage game = games[gameId];
        
        require(game.status == GameStatus.ACTIVE, "Game is not active");
        require(msg.sender == game.creator || msg.sender == game.opponent, "Not a player in this game");
        
        game.gameState = newState;
    }

    /**
     * @dev Complete a game with a winner
     * @param gameId ID of the game
     * @param winner Address of the winner
     */
    function completeGame(uint256 gameId, address winner) external nonReentrant {
        Game storage game = games[gameId];
        
        require(game.status == GameStatus.ACTIVE, "Game is not active");
        require(msg.sender == game.creator || msg.sender == game.opponent, "Not a player in this game");
        require(winner == game.creator || winner == game.opponent, "Winner must be a player");
        
        // Set game as completed
        game.status = GameStatus.COMPLETED;
        game.winner = winner;
        
        // Calculate prize (total stake)
        uint256 prize = game.stake.mul(2);
        
        // Transfer prize to winner
        (bool success, ) = payable(winner).call{value: prize}("");
        require(success, "Prize transfer failed");
        
        emit GameCompleted(gameId, winner, prize);
    }

    /**
     * @dev End game in a draw
     * @param gameId ID of the game
     */
    function drawGame(uint256 gameId) external nonReentrant {
        Game storage game = games[gameId];
        
        require(game.status == GameStatus.ACTIVE, "Game is not active");
        require(msg.sender == game.creator || msg.sender == game.opponent, "Not a player in this game");
        
        // Both players must agree to a draw - this is simplified for the hackathon
        // In a real implementation, there would be a confirmation from both players
        
        // Set game as draw
        game.status = GameStatus.DRAW;
        
        // Return stakes to both players
        (bool success1, ) = payable(game.creator).call{value: game.stake}("");
        (bool success2, ) = payable(game.opponent).call{value: game.stake}("");
        
        require(success1 && success2, "Stake return failed");
        
        emit GameDraw(gameId, game.stake);
    }

    /**
     * @dev Claim win by timeout
     * @param gameId ID of the game
     */
    function claimTimeoutWin(uint256 gameId) external nonReentrant {
        Game storage game = games[gameId];
        
        require(game.status == GameStatus.ACTIVE, "Game is not active");
        require(msg.sender == game.creator || msg.sender == game.opponent, "Not a player in this game");
        require(block.timestamp > game.lastMoveTime + game.timeoutDuration, "Timeout period has not elapsed");
        
        // The player who made the last move wins
        address winner = msg.sender;
        
        // Set game as timed out
        game.status = GameStatus.TIMEOUT;
        game.winner = winner;
        
        // Calculate prize (total stake)
        uint256 prize = game.stake.mul(2);
        
        // Transfer prize to winner
        (bool success, ) = payable(winner).call{value: prize}("");
        require(success, "Prize transfer failed");
        
        emit GameTimedOut(gameId, winner);
    }

    /**
     * @dev Get all moves for a game
     * @param gameId ID of the game
     */
    function getGameMoves(uint256 gameId) external view returns (Move[] memory) {
        return gameMoves[gameId];
    }
    
    /**
     * @dev Get all games for a player
     * @param player Address of the player
     */
    function getPlayerGames(address player) external view returns (uint256[] memory) {
        return playerGames[player];
    }
    
    /**
     * @dev Get number of games created
     */
    function getGameCount() external view returns (uint256) {
        return nextGameId - 1;
    }
    
    /**
     * @dev Get current game state
     * @param gameId ID of the game
     */
    function getGameState(uint256 gameId) external view returns (string memory) {
        return games[gameId].gameState;
    }
}
