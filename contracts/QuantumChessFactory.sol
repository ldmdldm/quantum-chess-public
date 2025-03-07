// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./QuantumChess.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * @title QuantumChessFactory
 * @dev A factory contract for creating and managing QuantumChess game instances
 * @author Quantum Chess Team
 */
contract QuantumChessFactory is Ownable, ReentrancyGuard {
    // Array to store all created QuantumChess instances
    address[] public games;
    
    // Mapping from game address to its metadata
    mapping(address => GameMetadata) public gameMetadata;
    
    // Track player participation in games
    mapping(address => address[]) public playerGames;
    
    // Configuration options for new games
    uint256 public defaultTimeoutDuration = 24 hours;
    uint256 public platformFee = 50; // 0.5% in basis points (10000 = 100%)
    address public feeCollector;
    
    // Metadata for each game instance
    struct GameMetadata {
        string name;
        string description;
        uint256 creationTime;
        address creator;
        bool featured;
    }
    
    // Events
    event GameCreated(address indexed gameAddress, address indexed creator, string name);
    event GameFeatured(address indexed gameAddress, bool featured);
    event FeeUpdated(uint256 newFee);
    event FeeCollectorUpdated(address newFeeCollector);
    
    /**
     * @dev Constructor
     * @param _feeCollector Address to collect platform fees
     */
    constructor(address _feeCollector) {
        require(_feeCollector != address(0), "Invalid fee collector address");
        feeCollector = _feeCollector;
    }
    
    /**
     * @dev Create a new QuantumChess game instance
     * @param name Game name
     * @param description Game description
     * @param initialState Initial game state encoded as JSON string
     * @param timeoutDuration Duration in seconds after which game can be timed out
     * @return New game contract address
     */
    function createGame(
        string memory name,
        string memory description,
        string memory initialState,
        uint256 timeoutDuration
    ) external returns (address) {
        // Use default timeout if none specified or if specified value is too low
        if (timeoutDuration < 1 hours) {
            timeoutDuration = defaultTimeoutDuration;
        }
        
        // Deploy new QuantumChess contract
        QuantumChess newGame = new QuantumChess();
        
        // Store game metadata
        gameMetadata[address(newGame)] = GameMetadata({
            name: name,
            description: description,
            creationTime: block.timestamp,
            creator: msg.sender,
            featured: false
        });
        
        // Add to game collections
        games.push(address(newGame));
        playerGames[msg.sender].push(address(newGame));
        
        // Emit creation event
        emit GameCreated(address(newGame), msg.sender, name);
        
        return address(newGame);
    }
    
    /**
     * @dev Get count of all created games
     * @return Number of games created
     */
    function getGameCount() external view returns (uint256) {
        return games.length;
    }
    
    /**
     * @dev Get all games for a specific player
     * @param player Player address
     * @return Array of game addresses
     */
    function getPlayerGames(address player) external view returns (address[] memory) {
        return playerGames[player];
    }
    
    /**
     * @dev Get games with pagination
     * @param offset Starting index
     * @param limit Maximum number of games to return
     * @return Array of game addresses
     */
    function getGames(uint256 offset, uint256 limit) external view returns (address[] memory) {
        if (offset >= games.length) {
            return new address[](0);
        }
        
        uint256 resultSize = (games.length - offset) < limit ? (games.length - offset) : limit;
        address[] memory result = new address[](resultSize);
        
        for (uint256 i = 0; i < resultSize; i++) {
            result[i] = games[offset + i];
        }
        
        return result;
    }
    
    /**
     * @dev Set featured status for a game
     * @param gameAddress Address of the game
     * @param featured Featured status
     */
    function setGameFeatured(address gameAddress, bool featured) external onlyOwner {
        require(gameMetadata[gameAddress].creationTime > 0, "Game does not exist");
        
        gameMetadata[gameAddress].featured = featured;
        
        emit GameFeatured(gameAddress, featured);
    }
    
    /**
     * @dev Get all featured games
     * @return Array of featured game addresses
     */
    function getFeaturedGames() external view returns (address[] memory) {
        uint256 featuredCount = 0;
        
        // Count featured games
        for (uint256 i = 0; i < games.length; i++) {
            if (gameMetadata[games[i]].featured) {
                featuredCount++;
            }
        }
        
        address[] memory featuredGames = new address[](featuredCount);
        uint256 currentIndex = 0;
        
        // Populate featured games array
        for (uint256 i = 0; i < games.length; i++) {
            if (gameMetadata[games[i]].featured) {
                featuredGames[currentIndex] = games[i];
                currentIndex++;
            }
        }
        
        return featuredGames;
    }
    
    /**
     * @dev Update platform fee (in basis points, 10000 = 100%)
     * @param newFee New fee in basis points
     */
    function updatePlatformFee(uint256 newFee) external onlyOwner {
        require(newFee <= 500, "Fee cannot exceed 5%");
        platformFee = newFee;
        
        emit FeeUpdated(newFee);
    }
    
    /**
     * @dev Update fee collector address
     * @param newFeeCollector New fee collector address
     */
    function updateFeeCollector(address newFeeCollector) external onlyOwner {
        require(newFeeCollector != address(0), "Invalid fee collector address");
        feeCollector = newFeeCollector;
        
        emit FeeCollectorUpdated(newFeeCollector);
    }
    
    /**
     * @dev Update default timeout duration
     * @param newTimeoutDuration New timeout duration in seconds
     */
    function updateDefaultTimeout(uint256 newTimeoutDuration) external onlyOwner {
        require(newTimeoutDuration >= 1 hours, "Timeout must be at least 1 hour");
        defaultTimeoutDuration = newTimeoutDuration;
    }
    
    /**
     * @dev Calculate platform fee amount
     * @param amount Base amount
     * @return Fee amount
     */
    function calculateFee(uint256 amount) public view returns (uint256) {
        return (amount * platformFee) / 10000;
    }
    
    /**
     * @dev Join a game with stake
     * @param gameAddress Address of the game to join
     */
    function joinGame(address gameAddress) external payable nonReentrant {
        require(gameMetadata[gameAddress].creationTime > 0, "Game does not exist");
        
        QuantumChess game = QuantumChess(gameAddress);
        
        // Calculate platform fee
        uint256 fee = calculateFee(msg.value);
        uint256 stakeAmount = msg.value - fee;
        
        // Transfer fee to fee collector
        if (fee > 0) {
            (bool feeSuccess, ) = payable(feeCollector).call{value: fee}("");
            require(feeSuccess, "Fee transfer failed");
        }
        
        // Add game to player's games
        playerGames[msg.sender].push(gameAddress);
        
        // Join the game with stake amount
        game.joinGame{value: stakeAmount}(msg.sender);
    }
}

