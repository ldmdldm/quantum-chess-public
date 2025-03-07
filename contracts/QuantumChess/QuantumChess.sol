// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

/**
 * @title QuantumChess
 * @dev A smart contract implementing quantum chess game mechanics
 */
contract QuantumChess {
    // Game states
    enum GameState {
        NotExists,
        Created,
        InProgress,
        Completed,
        Timeout
    }

    // Move structure with quantum probabilities
    struct Move {
        address player;
        string moveNotation;
        string fromPosition;
        string toPosition;
        uint256 timestamp;
        uint256 quantumProbability; // 0-100 representing probability percentage
    }

    // Game structure
    struct Game {
        address creator;
        address opponent;
        address winner;
        address currentTurn;
        uint256 stake;
        uint256 timeLimit; // time limit in seconds
        uint256 lastMoveTime;
        uint256 creationTime;
        GameState state;
        string initialBoardState;
        Move[] moves;
        mapping(address => bool) playerConfirmedEnd;
    }

    // Storage
    mapping(uint256 => Game) public games;
    uint256 public gameCount;
    
    // Events
    event GameCreated(uint256 indexed gameId, address indexed creator, uint256 stake, uint256 timeLimit);
    event GameJoined(uint256 indexed gameId, address indexed opponent);
    event MoveRecorded(uint256 indexed gameId, address indexed player, string moveNotation, string fromPosition, string toPosition, uint256 quantumProbability);
    event GameCompleted(uint256 indexed gameId, address indexed winner, uint256 reward);
    event GameTimeout(uint256 indexed gameId, address indexed winner);
    event StakeIncreased(uint256 indexed gameId, address indexed player, uint256 newStake);

    // Modifiers
    modifier gameExists(uint256 gameId) {
        require(games[gameId].state != GameState.NotExists, "Game does not exist");
        _;
    }

    modifier onlyPlayers(uint256 gameId) {
        require(
            msg.sender == games[gameId].creator || msg.sender == games[gameId].opponent,
            "Only game players can call this function"
        );
        _;
    }

    modifier inState(uint256 gameId, GameState state) {
        require(games[gameId].state == state, "Game is not in the required state");
        _;
    }

    modifier isPlayerTurn(uint256 gameId) {
        require(games[gameId].currentTurn == msg.sender, "Not your turn");
        _;
    }

    /**
     * @dev Create a new quantum chess game
     * @param timeLimit Time limit for moves in seconds
     * @param initialBoardState Initial board state in FEN or custom format
     * @return gameId The ID of the newly created game
     */
    function createGame(uint256 timeLimit, string memory initialBoardState) external payable returns (uint256) {
        require(msg.value > 0, "Stake must be greater than 0");
        require(timeLimit >= 60, "Time limit must be at least 60 seconds");

        uint256 gameId = gameCount++;
        
        Game storage newGame = games[gameId];
        newGame.creator = msg.sender;
        newGame.stake = msg.value;
        newGame.timeLimit = timeLimit;
        newGame.creationTime = block.timestamp;
        newGame.lastMoveTime = block.timestamp;
        newGame.state = GameState.Created;
        newGame.initialBoardState = initialBoardState;
        newGame.currentTurn = msg.sender; // Creator goes first by default
        
        emit GameCreated(gameId, msg.sender, msg.value, timeLimit);
        
        return gameId;
    }

    /**
     * @dev Join an existing quantum chess game
     * @param gameId The game ID to join
     */
    function joinGame(uint256 gameId) external payable gameExists(gameId) inState(gameId, GameState.Created) {
        Game storage game = games[gameId];
        
        require(msg.sender != game.creator, "Creator cannot join their own game");
        require(msg.value == game.stake, "Must match the creator's stake");
        
        game.opponent = msg.sender;
        game.state = GameState.InProgress;
        game.lastMoveTime = block.timestamp;
        
        emit GameJoined(gameId, msg.sender);
    }

    /**
     * @dev Record a chess move with quantum probability
     * @param gameId The game ID
     * @param moveNotation The chess move in standard notation
     * @param fromPosition Starting position
     * @param toPosition Ending position
     * @param quantumProbability Probability value (0-100)
     */
    function recordMove(
        uint256 gameId,
        string memory moveNotation,
        string memory fromPosition,
        string memory toPosition,
        uint256 quantumProbability
    ) external gameExists(gameId) onlyPlayers(gameId) inState(gameId, GameState.InProgress) isPlayerTurn(gameId) {
        require(quantumProbability <= 100, "Probability must be between 0-100");
        
        Game storage game = games[gameId];
        
        // Reset timeout timer
        game.lastMoveTime = block.timestamp;
        
        // Switch turns
        game.currentTurn = (msg.sender == game.creator) ? game.opponent : game.creator;
        
        // Record the move
        game.moves.push(Move({
            player: msg.sender,
            moveNotation: moveNotation,
            fromPosition: fromPosition,
            toPosition: toPosition,
            timestamp: block.timestamp,
            quantumProbability: quantumProbability
        }));
        
        emit MoveRecorded(gameId, msg.sender, moveNotation, fromPosition, toPosition, quantumProbability);
    }

    /**
     * @dev Increase the stake for a game
     * @param gameId The game ID
     */
    function increaseStake(uint256 gameId) external payable gameExists(gameId) onlyPlayers(gameId) inState(gameId, GameState.InProgress) {
        require(msg.value > 0, "Stake increase must be greater than 0");
        
        Game storage game = games[gameId];
        game.stake += msg.value;
        
        emit StakeIncreased(gameId, msg.sender, game.stake);
    }

    /**
     * @dev Check if a game has timed out and handle timeout
     * @param gameId The game ID
     * @return hasTimedOut Whether the game has timed out
     */
    function checkTimeout(uint256 gameId) public gameExists(gameId) inState(gameId, GameState.InProgress) returns (bool) {
        Game storage game = games[gameId];
        
        if (block.timestamp > game.lastMoveTime + game.timeLimit) {
            // Current player's turn timed out, other player wins
            address winner = (game.currentTurn == game.creator) ? game.opponent : game.creator;
            
            game.winner = winner;
            game.state = GameState.Timeout;
            
            // Transfer stake to winner
            uint256 totalStake = game.stake * 2;
            payable(winner).transfer(totalStake);
            
            emit GameTimeout(gameId, winner);
            return true;
        }
        
        return false;
    }

    /**
     * @dev Complete a game and declare a winner
     * @param gameId The game ID
     * @param winner The address of the winner
     */
    function completeGame(uint256 gameId, address winner) external gameExists(gameId) onlyPlayers(gameId) inState(gameId, GameState.InProgress) {
        Game storage game = games[gameId];
        
        // Ensure the winner is one of the players
        require(winner == game.creator || winner == game.opponent || winner == address(0), "Invalid winner address");
        
        // For a draw, use address(0) as winner
        if (winner == address(0)) {
            // Handle draw - split the stake
            uint256 playerStake = game.stake;
            payable(game.creator).transfer(playerStake);
            payable(game.opponent).transfer(playerStake);
            
            game.winner = address(0);
            game.state = GameState.Completed;
            
            emit GameCompleted(gameId, address(0), 0);
        } else {
            // Mark the game as completed with the specified winner
            game.playerConfirmedEnd[msg.sender] = true;
            
            // If both players have confirmed, or if it's been 24 hours since the first confirmation
            if (game.playerConfirmedEnd[game.creator] && game.playerConfirmedEnd[game.opponent]) {
                game.winner = winner;
                game.state = GameState.Completed;
                
                // Transfer the combined stake to the winner
                uint256 totalStake = game.stake * 2;
                payable(winner).transfer(totalStake);
                
                emit GameCompleted(gameId, winner, totalStake);
            }
        }
    }

    /**
     * @dev Get game details
     * @param gameId The game ID
     * @return creator Game creator address
     * @return opponent Opponent address
     * @return winner Winner address (if any)
     * @return currentTurn Current turn address
     * @return stake Game stake
     * @return timeLimit Time limit in seconds
     * @return lastMoveTime Timestamp of the last move
     * @return creationTime Game creation timestamp
     * @return state Game state
     * @return initialBoardState Initial board state
     */
    function getGameDetails(uint256 gameId) external view gameExists(gameId) returns (
        address creator,
        address opponent,
        address winner,
        address currentTurn,
        uint256 stake,
        uint256 timeLimit,
        uint256 lastMoveTime,
        uint256 creationTime,
        GameState state,
        string memory initialBoardState
    ) {
        Game storage game = games[gameId];
        
        return (
            game.creator,
            game.opponent,
            game.winner,
            game.currentTurn,
            game.stake,
            game.timeLimit,
            game.lastMoveTime,
            game.creationTime,
            game.state,
            game.initialBoardState
        );
    }

    /**
     * @dev Get the number of moves in a game
     * @param gameId The game ID
     * @return moveCount The number of moves
     */
    function getMoveCount(uint256 gameId) external view gameExists(gameId) returns (uint256) {
        return games[gameId].moves.length;
    }

    /**
     * @dev Get move details
     * @param gameId The game ID
     * @param moveIndex The index of the move
     * @return player Player who made the move
     * @return moveNotation Move notation
     * @return fromPosition Starting position
     * @return toPosition Ending position
     * @return timestamp Move timestamp
     * @return quantumProbability Quantum probability
     */
    function getMove(uint256 gameId, uint256 moveIndex) external view gameExists(gameId) returns (
        address player,
        string memory moveNotation,
        string memory fromPosition,
        string memory toPosition,
        uint256 timestamp,
        uint256 quantumProbability
    ) {
        require(moveIndex < games[gameId].moves.length, "Move index out of bounds");
        
        Move storage move = games[gameId].moves[moveIndex];
        
        return (
            move.player,
            move.moveNotation,
            move.fromPosition,
            move.toPosition,
            move.timestamp,
            move.quantumProbability
        );
    }

    /**
     * @dev Withdraw funds in case a game is abandoned
     * @param gameId The game ID
     */
    function withdrawFromAbandonedGame(uint256 gameId) external gameExists(gameId) onlyPlayers(gameId) {
        Game storage game = games[gameId];
        
        // Created games can be withdrawn by creator after 24 hours
        if (game.state == GameState.Created) {
            require(msg.sender == game.creator, "Only creator can withdraw from a created game");
            require(block.timestamp > game.creationTime + 24 hours, "Must wait 24 hours before withdrawing");
            
            game.state = GameState.Completed;
            payable(game.creator).transfer(game.stake);
        }
        // In progress games need timeout check
        else if (game.state == GameState.InProgress) {
            // Check for timeout first
            if (!checkTimeout(gameId)) {
                revert("Game is not timed out");
            }
        } else {
            revert("Cannot withdraw from this game state");
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title QuantumChess
 * @dev A smart contract for managing Quantum Chess games on the blockchain
 */
contract QuantumChess {
    // Game status enum
    enum GameStatus { 
        WAITING,    // Game is created but waiting for opponent
        ACTIVE,     // Game is active and in progress
        COMPLETED,  // Game has been completed with a winner
        DRAW,       // Game ended in a draw
        TIMEOUT,    // Game ended due to timeout
        CANCELED    // Game was canceled before completion
    }

    // Move structure to store quantum chess moves
    struct Move {
        address player;
        uint8 fromX;
        uint8 fromY;
        uint8 toX;
        uint8 toY;
        uint8 pieceType;
        uint8 probability; // Quantum probability (0-100)
        uint256 timestamp;
    }

    // Game structure to store quantum chess games
    struct Game {
        address creator;
        address opponent;
        address currentPlayer;
        uint256 stake;
        uint256 gameStartTime;
        uint256 lastMoveTime;
        uint256 timeoutDuration;
        GameStatus status;
        Move[] moves;
        mapping(address => uint256) balances;
        string quantumState; // JSON string representing the quantum state of the board
    }

    // State variables
    uint256 public gameCount;
    mapping(uint256 => Game) public games;
    mapping(address => uint256[]) public playerGames;
    
    // Events
    event GameCreated(uint256 indexed gameId, address indexed creator, uint256 stake);
    event GameJoined(uint256 indexed gameId, address indexed opponent);
    event MoveMade(uint256 indexed gameId, address indexed player, uint8 fromX, uint8 fromY, uint8 toX, uint8 toY, uint8 probability);
    event GameFinished(uint256 indexed gameId, address indexed winner, uint256 prize);
    event GameCanceled(uint256 indexed gameId);
    event GameTimeout(uint256 indexed gameId, address indexed winner);
    event StakeAdded(uint256 indexed gameId, address indexed player, uint256 amount);
    event FundsWithdrawn(address indexed player, uint256 amount);

    // Modifiers
    modifier gameExists(uint256 gameId) {
        require(gameId < gameCount, "Game does not exist");
        _;
    }
    
    modifier onlyGamePlayer(uint256 gameId) {
        require(
            msg.sender == games[gameId].creator || 
            msg.sender == games[gameId].opponent,
            "Not a player in this game"
        );
        _;
    }
    
    modifier gameActive(uint256 gameId) {
        require(games[gameId].status == GameStatus.ACTIVE, "Game is not active");
        _;
    }
    
    modifier isPlayerTurn(uint256 gameId) {
        require(msg.sender == games[gameId].currentPlayer, "Not your turn");
        _;
    }

    /**
     * @dev Create a new quantum chess game
     * @param timeoutDuration The duration in seconds after which the game times out
     * @return gameId The ID of the created game
     */
    function createGame(uint256 timeoutDuration) external payable returns (uint256) {
        require(msg.value > 0, "Stake must be greater than 0");
        require(timeoutDuration >= 300, "Timeout duration must be at least 5 minutes");
        
        uint256 gameId = gameCount;
        Game storage game = games[gameId];
        
        game.creator = msg.sender;
        game.currentPlayer = msg.sender; // Creator moves first
        game.stake = msg.value;
        game.gameStartTime = block.timestamp;
        game.lastMoveTime = block.timestamp;
        game.timeoutDuration = timeoutDuration;
        game.status = GameStatus.WAITING;
        game.balances[msg.sender] = msg.value;
        
        // Initialize quantum state as empty board in JSON format
        game.quantumState = "{ \"board\": [[],[],[],[],[],[],[],[]]}";
        
        playerGames[msg.sender].push(gameId);
        gameCount++;
        
        emit GameCreated(gameId, msg.sender, msg.value);
        
        return gameId;
    }
    
    /**
     * @dev Join an existing quantum chess game
     * @param gameId The ID of the game to join
     */
    function joinGame(uint256 gameId) external payable gameExists(gameId) {
        Game storage game = games[gameId];
        
        require(game.status == GameStatus.WAITING, "Game is not waiting for players");
        require(msg.sender != game.creator, "Cannot join your own game");
        require(msg.value == game.stake, "Must match the game stake");
        
        game.opponent = msg.sender;
        game.status = GameStatus.ACTIVE;
        game.balances[msg.sender] = msg.value;
        
        playerGames[msg.sender].push(gameId);
        
        emit GameJoined(gameId, msg.sender);
    }
    
    /**
     * @dev Record a move in the quantum chess game
     * @param gameId The ID of the game
     * @param fromX X coordinate of the starting position
     * @param fromY Y coordinate of the starting position
     * @param toX X coordinate of the destination position
     * @param toY Y coordinate of the destination position
     * @param pieceType Type of the chess piece
     * @param probability Quantum probability of the move (0-100)
     * @param quantumState New quantum state of the board as JSON
     */
    function recordMove(
        uint256 gameId,
        uint8 fromX,
        uint8 fromY,
        uint8 toX,
        uint8 toY,
        uint8 pieceType,
        uint8 probability,
        string calldata quantumState
    ) external gameExists(gameId) gameActive(gameId) isPlayerTurn(gameId) {
        require(probability <= 100, "Probability must be between 0 and 100");
        
        Game storage game = games[gameId];
        
        // Add the move to the game's move history
        game.moves.push(
            Move({
                player: msg.sender,
                fromX: fromX,
                fromY: fromY,
                toX: toX,
                toY: toY,
                pieceType: pieceType,
                probability: probability,
                timestamp: block.timestamp
            })
        );
        
        // Update quantum state
        game.quantumState = quantumState;
        
        // Update last move time
        game.lastMoveTime = block.timestamp;
        
        // Switch turns
        game.currentPlayer = (msg.sender == game.creator) ? game.opponent : game.creator;
        
        emit MoveMade(gameId, msg.sender, fromX, fromY, toX, toY, probability);
    }
    
    /**
     * @dev Add more stake to the game
     * @param gameId The ID of the game
     */
    function placeStake(uint256 gameId) external payable gameExists(gameId) gameActive(gameId) onlyGamePlayer(gameId) {
        require(msg.value > 0, "Stake must be greater than 0");
        
        Game storage game = games[gameId];
        game.stake += msg.value;
        game.balances[msg.sender] += msg.value;
        
        emit StakeAdded(gameId, msg.sender, msg.value);
    }
    
    /**
     * @dev Check if a game has timed out and update its status
     * @param gameId The ID of the game to check
     * @return True if the game has timed out
     */
    function checkTimeout(uint256 gameId) public gameExists(gameId) gameActive(gameId) returns (bool) {
        Game storage game = games[gameId];
        
        if (block.timestamp > game.lastMoveTime + game.timeoutDuration) {
            // Current player has timed out, the other player wins
            address winner = (game.currentPlayer == game.creator) ? game.opponent : game.creator;
            
            // Update game status
            game.status = GameStatus.TIMEOUT;
            
            // Transfer funds to winner
            uint256 prize = game.balances[game.creator] + game.balances[game.opponent];
            game.balances[winner] = prize;
            game.balances[game.currentPlayer] = 0;
            
            emit GameTimeout(gameId, winner);
            return true;
        }
        
        return false;
    }
    
    /**
     * @dev Claim victory due to opponent's timeout
     * @param gameId The ID of the game
     */
    function claimTimeoutVictory(uint256 gameId) external gameExists(gameId) gameActive(gameId) onlyGamePlayer(gameId) {
        Game storage game = games[gameId];
        
        // Ensure that the claimer is not the current player
        require(msg.sender != game.currentPlayer, "Cannot claim timeout on your turn");
        
        // Check if timeout has occurred
        require(checkTimeout(gameId), "No timeout has occurred");
        
        // Payout should already be handled in checkTimeout
        withdrawFunds(gameId);
    }
    
    /**
     * @dev Complete a game with a winner
     * @param gameId The ID of the game
     * @param winner Address of the winner (address(0) for draw)
     */
    function completeGame(uint256 gameId, address winner) external gameExists(gameId) gameActive(gameId) onlyGamePlayer(gameId) {
        Game storage game = games[gameId];
        
        // Both players must agree on the outcome
        // This is a simplified implementation - in practice, you would need both players to sign off
        
        if (winner == address(0)) {
            // Game ended in a draw
            game.status = GameStatus.DRAW;
            
            // Split the stakes
            uint256 halfStake = (game.balances[game.creator] + game.balances[game.opponent]) / 2;
            game.balances[game.creator] = halfStake;
            game.balances[game.opponent] = halfStake;
            
            emit GameFinished(gameId, address(0), 0);
        } else {
            // Game ended with a winner
            require(winner == game.creator || winner == game.opponent, "Winner must be a player");
            
            game.status = GameStatus.COMPLETED;
            
            // Transfer all funds to winner
            uint256 prize = game.balances[game.creator] + game.balances[game.opponent];
            game.balances[winner] = prize;
            
            // Reset loser's balance
            address loser = (winner == game.creator) ? game.opponent : game.creator;
            game.balances[loser] = 0;
            
            emit GameFinished(gameId, winner, prize);
        }
    }
    
    /**
     * @dev Withdraw funds from a completed game
     * @param gameId The ID of the game
     */
    function withdrawFunds(uint256 gameId) public gameExists(gameId) onlyGamePlayer(gameId) {
        Game storage game = games[gameId];
        
        // Can only withdraw if game is completed, draw, timeout, or canceled
        require(
            game.status == GameStatus.COMPLETED || 
            game.status == GameStatus.DRAW || 
            game.status == GameStatus.TIMEOUT || 
            game.status == GameStatus.CANCELED,
            "Game is still in progress"
        );
        
        uint256 amount = game.balances[msg.sender];
        require(amount > 0, "No funds to withdraw");
        
        // Reset balance before transfer to prevent reentrancy attacks
        game.balances[msg.sender] = 0;
        
        // Transfer funds to player
        (bool success, ) = payable(msg.sender).call{value: amount}("");
        require(success, "Transfer failed");
        
        emit FundsWithdrawn(msg.sender, amount);
    }
    
    /**
     * @dev Cancel a game that hasn't been joined yet
     * @param gameId The ID of the game
     */
    function cancelGame(uint256 gameId) external gameExists(gameId) {
        Game storage game = games[gameId];
        
        require(msg.sender == game.creator, "Only creator can cancel");
        require(game.status == GameStatus.WAITING, "Game is already active");
        
        game.status = GameStatus.CANCELED;
        
        emit GameCanceled(gameId);
        
        // Allow creator to withdraw their stake
        withdrawFunds(gameId);
    }
    
    /**
     * @dev Get game details
     * @param gameId The ID of the game
     * @return creator Creator of the game
     * @return opponent Opponent in the game
     * @return currentPlayer Current player's turn
     * @return stake Total stake in the game
     * @return gameStartTime Time when the game started
     * @return lastMoveTime Time of the last move
     * @return timeoutDuration Timeout duration in seconds
     * @return status Status of the game
     * @return quantumState Current quantum state of the board
     */
    function getGameDetails(uint256 gameId) external view gameExists(gameId) returns (
        address creator,
        address opponent,
        address currentPlayer,
        uint256 stake,
        uint256 gameStartTime,
        uint256 lastMoveTime,
        uint256 timeoutDuration,
        GameStatus status,
        string memory quantumState
    ) {
        Game storage game = games[gameId];
        
        return (
            game.creator,
            game.opponent,
            game.currentPlayer,
            game.stake,
            game.gameStartTime,
            game.lastMoveTime,
            game.timeoutDuration,
            game.status,
            game.quantumState
        );
    }
    
    /**
     * @dev Get the number of moves in a game
     * @param gameId The ID of the game
     * @return Number of moves
     */
    function getMoveCount(uint256 gameId) external view gameExists(gameId) returns (uint256) {
        return games[gameId].moves.length;
    }
    
    /**
     * @dev Get details of a specific move
     * @param gameId The ID of the game
     * @param moveIndex Index of the move
     * @return player Player who made the move
     * @return fromX X coordinate of the starting position
     * @return fromY Y coordinate of the starting position
     * @return toX X coordinate of the destination position
     * @return toY Y coordinate of the destination position
     * @return pieceType Type of the chess piece
     * @return probability Quantum probability of the move
     * @return timestamp Time when the move was made
     */
    function getMoveDetails(uint256 gameId, uint256 moveIndex) external view gameExists(gameId) returns (
        address player,
        uint8 fromX,
        uint8 fromY,
        uint8 toX,
        uint8 toY,
        uint8 pieceType,
        uint8 probability,
        uint256 timestamp
    ) {
        require(moveIndex < games[gameId].moves.length, "Move does not exist");
        
        Move storage move = games[gameId].moves[moveIndex];
        
        return (
            move.player,
            move.fromX,

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title QuantumChess
 * @dev Smart contract for a quantum chess game with quantum probabilities and game state management
 */
contract QuantumChess {
    // Game constants
    uint256 private constant MAX_PLAYERS = 2;
    uint256 private constant BOARD_SIZE = 8;
    uint256 private constant DEFAULT_TIMEOUT = 1 days;
    
    // Game state enum
    enum GameState { 
        NotExist,      // Game does not exist
        Created,       // Game is created but waiting for opponent
        Active,        // Game is active with both players
        Completed,     // Game completed normally
        Timeout,       // Game ended due to timeout
        Cancelled      // Game cancelled
    }
    
    // Piece type enum
    enum PieceType { 
        None, 
        Pawn, 
        Knight, 
        Bishop, 
        Rook, 
        Queen, 
        King 
    }
    
    // Quantum state for pieces
    struct QuantumState {
        bool isQuantum;            // Whether the piece is in quantum state
        uint8[] possiblePositions; // Array of possible positions (row*8 + col)
        uint8[] probabilities;     // Corresponding probabilities (0-100)
    }
    
    // Piece structure
    struct Piece {
        PieceType pieceType;       // Type of piece
        bool isWhite;              // Color of the piece
        uint8 position;            // Current position on board (row*8 + col)
        QuantumState quantumState; // Quantum state if applicable
    }
    
    // Move structure
    struct Move {
        address player;            // Player who made the move
        uint8 fromPosition;        // Starting position
        uint8 toPosition;          // Target position
        uint256 timestamp;         // When the move was made
        bool isQuantum;            // Whether it's a quantum move
        uint8[] quantumPositions;  // Possible positions for quantum move
        uint8[] probabilities;     // Probabilities for each position
    }
    
    // Game structure
    struct Game {
        uint256 gameId;               // Unique game ID
        address creator;              // Address of game creator
        address opponent;             // Address of opponent
        GameState state;              // Current state of the game
        uint256 stake;                // Amount staked on the game
        uint256 lastMoveTime;         // Timestamp of the last move
        uint256 timeoutDuration;      // Duration after which game times out
        address currentTurn;          // Address of the player whose turn it is
        bool creatorIsWhite;          // Whether creator plays as white
        mapping(uint8 => Piece) board; // Game board state
        Move[] moves;                 // Record of all moves made
        uint256 winner;               // 0 = not determined, 1 = creator, 2 = opponent
    }
    
    // Contract state variables
    uint256 private gameCounter = 0;
    mapping(uint256 => Game) private games;
    mapping(address => uint256[]) private playerGames;
    
    // Events
    event GameCreated(uint256 indexed gameId, address indexed creator, uint256 stake, uint256 timeout);
    event GameJoined(uint256 indexed gameId, address indexed opponent);
    event MoveMade(uint256 indexed gameId, address indexed player, uint8 fromPosition, uint8 toPosition, bool isQuantum);
    event QuantumStateCollapsed(uint256 indexed gameId, uint8 piecePosition, uint8 finalPosition);
    event GameCompleted(uint256 indexed gameId, address indexed winner, uint256 reward);
    event GameTimedOut(uint256 indexed gameId, address indexed winner);
    event GameCancelled(uint256 indexed gameId);
    event StakeIncreased(uint256 indexed gameId, address indexed player, uint256 newStake);
    
    // Modifiers
    modifier onlyPlayer(uint256 _gameId) {
        require(
            msg.sender == games[_gameId].creator || 
            msg.sender == games[_gameId].opponent,
            "Only game players can call this function"
        );
        _;
    }
    
    modifier gameExists(uint256 _gameId) {
        require(games[_gameId].state != GameState.NotExist, "Game does not exist");
        _;
    }
    
    modifier gameActive(uint256 _gameId) {
        require(games[_gameId].state == GameState.Active, "Game is not active");
        _;
    }
    
    modifier notCompleted(uint256 _gameId) {
        require(
            games[_gameId].state != GameState.Completed && 
            games[_gameId].state != GameState.Timeout && 
            games[_gameId].state != GameState.Cancelled,
            "Game is already completed"
        );
        _;
    }
    
    modifier onlyCurrentTurn(uint256 _gameId) {
        require(msg.sender == games[_gameId].currentTurn, "Not your turn");
        _;
    }
    
    /**
     * @dev Create a new game with optional stake
     * @param _timeoutDuration Duration in seconds after which game times out
     * @param _playAsWhite Whether the creator plays as white
     */
    function createGame(uint256 _timeoutDuration, bool _playAsWhite) external payable returns (uint256) {
        uint256 gameId = gameCounter++;
        
        Game storage newGame = games[gameId];
        newGame.gameId = gameId;
        newGame.creator = msg.sender;
        newGame.state = GameState.Created;
        newGame.stake = msg.value;
        newGame.lastMoveTime = block.timestamp;
        newGame.timeoutDuration = _timeoutDuration > 0 ? _timeoutDuration : DEFAULT_TIMEOUT;
        newGame.creatorIsWhite = _playAsWhite;
        
        // Initialize the board with standard chess pieces
        // This would be a more complex initialization in practice
        
        playerGames[msg.sender].push(gameId);
        
        emit GameCreated(gameId, msg.sender, msg.value, _timeoutDuration);
        
        return gameId;
    }
    
    /**
     * @dev Join an existing game as an opponent
     * @param _gameId ID of the game to join
     */
    function joinGame(uint256 _gameId) external payable gameExists(_gameId) {
        Game storage game = games[_gameId];
        
        require(game.state == GameState.Created, "Game is not open for joining");
        require(game.creator != msg.sender, "Cannot join your own game");
        require(msg.value == game.stake, "Stake amount must match the creator's stake");
        
        game.opponent = msg.sender;
        game.state = GameState.Active;
        game.lastMoveTime = block.timestamp;
        
        // Set the starting turn
        game.currentTurn = game.creatorIsWhite ? game.creator : game.opponent;
        
        playerGames[msg.sender].push(_gameId);
        
        emit GameJoined(_gameId, msg.sender);
    }
    
    /**
     * @dev Record a standard (non-quantum) move
     * @param _gameId ID of the game
     * @param _fromPosition Starting position of the piece
     * @param _toPosition Target position for the piece
     */
    function recordMove(
        uint256 _gameId, 
        uint8 _fromPosition, 
        uint8 _toPosition
    ) 
        external 
        gameExists(_gameId) 
        gameActive(_gameId) 
        onlyPlayer(_gameId) 
        onlyCurrentTurn(_gameId) 
    {
        Game storage game = games[_gameId];
        
        // Validate the move (simplified)
        require(_fromPosition < BOARD_SIZE * BOARD_SIZE, "Invalid from position");
        require(_toPosition < BOARD_SIZE * BOARD_SIZE, "Invalid to position");
        
        // Check if the piece at from position belongs to the player
        // In a real implementation, we would have more complex validation
        
        // Update the board state
        Piece storage piece = game.board[_fromPosition];
        game.board[_toPosition] = piece;
        delete game.board[_fromPosition];
        
        // Store the move
        Move memory newMove = Move({
            player: msg.sender,
            fromPosition: _fromPosition,
            toPosition: _toPosition,
            timestamp: block.timestamp,
            isQuantum: false,
            quantumPositions: new uint8[](0),
            probabilities: new uint8[](0)
        });
        
        game.moves.push(newMove);
        
        // Update turn
        game.currentTurn = (msg.sender == game.creator) ? game.opponent : game.creator;
        game.lastMoveTime = block.timestamp;
        
        // Check if the game is over (simplified)
        // In a real implementation, we would check for checkmate, etc.
        
        emit MoveMade(_gameId, msg.sender, _fromPosition, _toPosition, false);
    }
    
    /**
     * @dev Record a quantum move with multiple possible positions
     * @param _gameId ID of the game
     * @param _fromPosition Starting position of the piece
     * @param _quantumPositions Array of possible target positions
     * @param _probabilities Corresponding probabilities for each position (0-100)
     */
    function recordQuantumMove(
        uint256 _gameId, 
        uint8 _fromPosition, 
        uint8[] calldata _quantumPositions, 
        uint8[] calldata _probabilities
    ) 
        external 
        gameExists(_gameId) 
        gameActive(_gameId) 
        onlyPlayer(_gameId) 
        onlyCurrentTurn(_gameId) 
    {
        Game storage game = games[_gameId];
        
        // Validate quantum move
        require(_quantumPositions.length > 1, "Quantum move needs multiple positions");
        require(_quantumPositions.length == _probabilities.length, "Positions and probabilities must match");
        
        // Validate all positions
        for (uint i = 0; i < _quantumPositions.length; i++) {
            require(_quantumPositions[i] < BOARD_SIZE * BOARD_SIZE, "Invalid position");
            if (i > 0) {
                require(_probabilities[i-1] <= 100, "Probability must be between 0-100");
            }
        }
        
        // Validate sum of probabilities equals 100
        uint256 totalProb = 0;
        for (uint i = 0; i < _probabilities.length; i++) {
            totalProb += _probabilities[i];
        }
        require(totalProb == 100, "Sum of probabilities must be 100%");
        
        // Set the piece in quantum state
        Piece storage piece = game.board[_fromPosition];
        piece.quantumState.isQuantum = true;
        piece.quantumState.possiblePositions = _quantumPositions;
        piece.quantumState.probabilities = _probabilities;
        
        // Store the move
        Move memory newMove = Move({
            player: msg.sender,
            fromPosition: _fromPosition,
            toPosition: 0, // This is a quantum move, so no single destination
            timestamp: block.timestamp,
            isQuantum: true,
            quantumPositions: _quantumPositions,
            probabilities: _probabilities
        });
        
        game.moves.push(newMove);
        
        // Update turn
        game.currentTurn = (msg.sender == game.creator) ? game.opponent : game.creator;
        game.lastMoveTime = block.timestamp;
        
        emit MoveMade(_gameId, msg.sender, _fromPosition, 0, true);
    }
    
    /**
     * @dev Collapse a quantum state based on a random seed
     * @param _gameId ID of the game
     * @param _piecePosition Position of the quantum piece to collapse
     * @param _seed Random seed for determining the outcome
     */
    function collapseQuantumState(
        uint256 _gameId,
        uint8 _piecePosition,
        uint256 _seed
    )
        external
        gameExists(_gameId)
        gameActive(_gameId)
        onlyPlayer(_gameId)
    {
        Game storage game = games[_gameId];
        
        // Get the piece
        Piece storage piece = game.board[_piecePosition];
        require(piece.quantumState.isQuantum, "Piece is not in quantum state");
        
        // Determine the final position based on probabilities and seed
        uint8 finalPosition = determineQuantumOutcome(
            piece.quantumState.possiblePositions,
            piece.quantumState.probabilities,
            _seed
        );
        
        // Update the piece position
        piece.position = finalPosition;
        piece.quantumState.isQuantum = false;
        
        emit QuantumStateCollapsed(_gameId, _piecePosition, finalPosition);
    }
    
    /**
     * @dev Determine the outcome of a quantum state collapse
     * @param _positions Possible positions
     * @param _probabilities Corresponding probabilities
     * @param _seed Random seed
     * @return The selected position based on probabilities
     */
    function determineQuantumOutcome(
        uint8[] memory _positions,
        uint8[] memory _probabilities,
        uint256 _seed
    ) 
        private 
        pure 
        returns (uint8) 
    {
        // Use the seed to generate a number between 0-100
        uint8 rand = uint8(_seed % 100);
        
        // Find which probability bucket the random number falls into
        uint8 cumulativeProb = 0;
        for (uint i = 0; i < _probabilities.length; i++) {
            cumulativeProb += _probabilities[i];
            if (rand < cumulativeProb) {
                return _positions[i];
            }
        }
        
        // Fallback to the last position (should never happen if probabilities sum to 100)
        return _positions[_positions.length - 1];
    }
    
    /**
     * @dev Place additional stake on a game
     * @param _gameId ID of the game
     */
    function placeStake(uint256 _gameId) 
        external 
        payable 
        gameExists(_gameId) 
        notCompleted(_gameId) 
        onlyPlayer(_gameId) 
    {
        Game storage game = games[_gameId];
        
        // Check if the opponent has also joined (for matching stakes)
        if (game.state == GameState.Active) {
            require(
                msg.sender == game.creator && game.opponent != address(0),
                "Only creator can add stake in active games"
            );
            
            // In active games, notify the opponent they need to match the stake
            // In a real implementation, we might implement a time window for matching
        }
        
        // Add stake to the game
        game.stake += msg.value;
        
        emit StakeIncreased(_gameId, msg.sender, game.stake);
    }
    
    /**
     * @dev Check for timeout and handle it if necessary
     * @param _gameId ID of the game
     * @return Whether the game

