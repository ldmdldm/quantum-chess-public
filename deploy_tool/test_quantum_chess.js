const { ethers } = require('ethers');
require('dotenv').config();

// Contract ABI - This should match your deployed contract's ABI
// This is a subset focusing on the main functions we need to test
const contractABI = [
  // Game creation and joining
  "function createGame(uint256 stake) external payable returns (uint256 gameId)",
  "function joinGame(uint256 gameId) external payable",
  
  // Move recording and game state
  "function recordMove(uint256 gameId, string memory moveNotation, uint8 fromX, uint8 fromY, uint8 toX, uint8 toY) external",
  "function recordQuantumMove(uint256 gameId, string memory moveNotation, uint8 fromX, uint8 fromY, uint8 toX, uint8 toY, uint8 probability) external",
  
  // Game state getters
  "function getGameState(uint256 gameId) external view returns (uint8 state, address player1, address player2, uint256 stake, uint256 lastMoveTime)",
  "function getGameMoves(uint256 gameId) external view returns (string[] memory)",

  // Events
  "event GameCreated(uint256 indexed gameId, address indexed creator, uint256 stake)",
  "event GameJoined(uint256 indexed gameId, address indexed player)",
  "event MoveMade(uint256 indexed gameId, address indexed player, string moveNotation, uint8 probability)"
];

async function main() {
  try {
    // Connect to the blockchain
    const provider = new ethers.providers.JsonRpcProvider(process.env.CORE_BLOCKCHAIN_URL || "https://rpc.test2.btcs.network");
    
    // Load wallet using private key from .env file
    const privateKey = process.env.CORE_PRIVATE_KEY;
    if (!privateKey) {
      throw new Error("Private key not found in .env file");
    }
    
    const wallet = new ethers.Wallet(privateKey, provider);
    const address = wallet.address;
    console.log(`Using wallet address: ${address}`);
    
    // Get wallet balance
    const balance = await provider.getBalance(address);
    console.log(`Wallet balance: ${ethers.utils.formatEther(balance)} ETH`);
    
    if (balance.eq(0)) {
      throw new Error("Wallet has zero balance. Please fund your wallet before testing.");
    }
    
    // Connect to the deployed contract
    const contractAddress = process.env.CORE_CONTRACT_ADDRESS;
    if (!contractAddress) {
      throw new Error("Contract address not found in .env file");
    }
    
    console.log(`Connecting to contract at address: ${contractAddress}`);
    const contract = new ethers.Contract(contractAddress, contractABI, wallet);
    
    // Create a new game with a small stake
    const stake = ethers.utils.parseEther("0.01"); // 0.01 ETH stake
    console.log(`Creating a new game with ${ethers.utils.formatEther(stake)} ETH stake...`);
    
    const createGameTx = await contract.createGame(stake, { 
      value: stake,
      gasLimit: 500000 // Set an appropriate gas limit
    });
    
    console.log(`Transaction hash: ${createGameTx.hash}`);
    console.log("Waiting for transaction confirmation...");
    
    const createGameReceipt = await createGameTx.wait();
    console.log("Game creation confirmed in block:", createGameReceipt.blockNumber);
    
    // Get the gameId from the event
    const gameCreatedEvent = createGameReceipt.events.find(event => event.event === 'GameCreated');
    if (!gameCreatedEvent) {
      throw new Error("GameCreated event not found in transaction receipt");
    }
    
    const gameId = gameCreatedEvent.args.gameId.toNumber();
    console.log(`New game created with ID: ${gameId}`);
    
    // Get game state
    const gameState = await contract.getGameState(gameId);
    console.log("Game state:", {
      state: gameState.state,
      player1: gameState.player1,
      player2: gameState.player2,
      stake: ethers.utils.formatEther(gameState.stake),
      lastMoveTime: new Date(gameState.lastMoveTime.toNumber() * 1000).toISOString()
    });
    
    // Create a second wallet for testing joining the game
    // In practice, this would be a different user
    const privateKey2 = ethers.Wallet.createRandom().privateKey;
    const wallet2 = new ethers.Wallet(privateKey2, provider);
    console.log(`Created second wallet with address: ${wallet2.address}`);
    
    // Fund the second wallet to join the game
    console.log("Funding second wallet...");
    const fundTx = await wallet.sendTransaction({
      to: wallet2.address,
      value: stake.add(ethers.utils.parseEther("0.01")) // Stake + gas costs
    });
    
    await fundTx.wait();
    console.log("Second wallet funded successfully");
    
    // Connect the second wallet to the contract
    const contractWithWallet2 = contract.connect(wallet2);
    
    // Join the game with the second wallet
    console.log(`Joining game ${gameId} with second wallet...`);
    const joinGameTx = await contractWithWallet2.joinGame(gameId, {
      value: stake,
      gasLimit: 500000
    });
    
    console.log(`Transaction hash: ${joinGameTx.hash}`);
    console.log("Waiting for transaction confirmation...");
    
    const joinGameReceipt = await joinGameTx.wait();
    console.log("Game joined confirmed in block:", joinGameReceipt.blockNumber);
    
    // Get updated game state
    const updatedGameState = await contract.getGameState(gameId);
    console.log("Updated game state:", {
      state: updatedGameState.state,
      player1: updatedGameState.player1,
      player2: updatedGameState.player2,
      stake: ethers.utils.formatEther(updatedGameState.stake),
      lastMoveTime: new Date(updatedGameState.lastMoveTime.toNumber() * 1000).toISOString()
    });
    
    // Make a standard move (non-quantum)
    console.log("Making a standard chess move...");
    const standardMoveTx = await contract.recordMove(
      gameId,
      "e2-e4", // Standard chess notation
      4, 1,    // From position (e2)
      4, 3,    // To position (e4)
    );
    
    await standardMoveTx.wait();
    console.log("Standard move recorded successfully");
    
    // Make a quantum move with 70% probability
    console.log("Making a quantum move with 70% probability...");
    const quantumMoveTx = await contractWithWallet2.recordQuantumMove(
      gameId,
      "Nb8-Nc6/Na6", // Quantum move notation (knight can go to c6 or a6)
      1, 7,          // From position (b8)
      2, 5,          // To position 1 (c6)
      0, 5,          // To position 2 (a6)
      70             // 70% probability for first position, 30% for second
    );
    
    await quantumMoveTx.wait();
    console.log("Quantum move recorded successfully");
    
    // Get game moves
    const moves = await contract.getGameMoves(gameId);
    console.log("Game moves:", moves);
    
    console.log("Test completed successfully!");
    
  } catch (error) {
    console.error("Error:", error.message);
    // If we have transaction data in the error, print it
    if (error.transaction) {
      console.error("Transaction:", error.transaction);
    }
    if (error.receipt) {
      console.error("Receipt:", error.receipt);
    }
  }
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });

