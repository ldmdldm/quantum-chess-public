#!/usr/bin/env node

/**
 * QuantumChess Contract Deployment Script
 * 
 * This script deploys the QuantumChess smart contract to the Core Testnet.
 * It compiles the Solidity contract and deploys it using ethers.js.
 * 
 * Required environment variables:
 * - CORE_PRIVATE_KEY: Your private key for signing transactions
 * - CORE_BLOCKCHAIN_URL: (Optional) Core Testnet RPC URL, defaults to https://rpc.test2.btcs.network
 */

const fs = require('fs');
const path = require('path');
const { ethers } = require('ethers');
const solc = require('solc');
require('dotenv').config();

// Configuration
const DEFAULT_RPC_URL = 'https://rpc.test2.btcs.network';
const CORE_CHAIN_ID = 1114;

// Check for private key in environment variables
const privateKey = process.env.CORE_PRIVATE_KEY;
if (!privateKey) {
  console.error('ERROR: Private key not found. Set CORE_PRIVATE_KEY in your environment or .env file.');
  process.exit(1);
}

// Get RPC URL from environment or use default Core Testnet
const rpcUrl = process.env.CORE_BLOCKCHAIN_URL || DEFAULT_RPC_URL;

// Contract file path
const contractPath = path.resolve(__dirname, '..', 'QuantumChessToken.sol');
const contractOutputDir = path.resolve(__dirname, '..', 'bin');

// Ensure the output directory exists
if (!fs.existsSync(contractOutputDir)) {
  fs.mkdirSync(contractOutputDir, { recursive: true });
}

/**
 * Compile the Solidity contract
 */
async function compileContract() {
  console.log('Reading contract source...');
  
  // Read the contract source
  const contractSource = fs.readFileSync(contractPath, 'utf8');
  
  // Create a function to handle imports
  function findImports(importPath) {
    // Handle imports from @openzeppelin and other node_modules
    if (importPath.startsWith('@') || importPath.startsWith('node_modules')) {
      const npmPath = path.resolve(__dirname, 'node_modules', importPath);
      if (fs.existsSync(npmPath)) {
        return { contents: fs.readFileSync(npmPath, 'utf8') };
      }
      
      // If not found directly, try in parent node_modules
      const parentNpmPath = path.resolve(__dirname, '..', 'node_modules', importPath);
      if (fs.existsSync(parentNpmPath)) {
        return { contents: fs.readFileSync(parentNpmPath, 'utf8') };
      }
    }
    
    // Handle local imports
    let fullPath;
    if (path.isAbsolute(importPath)) {
      fullPath = importPath;
    } else {
      // Try to resolve relative to the contract directory
      fullPath = path.resolve(path.dirname(contractPath), importPath);
    }
    
    if (fs.existsSync(fullPath)) {
      return { contents: fs.readFileSync(fullPath, 'utf8') };
    }
    
    return { error: `Import file not found: ${importPath}` };
  }
  
  // Prepare input for the Solidity compiler
  const input = {
    language: 'Solidity',
    sources: {
      'QuantumChessToken.sol': {
        content: contractSource
      }
    },
    settings: {
      outputSelection: {
        '*': {
          '*': ['abi', 'evm.bytecode']
        }
      },
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  };

  console.log('Compiling contract...');
  
  // Compile the contract with import resolution
  const output = JSON.parse(solc.compile(JSON.stringify(input), { import: findImports }));
  
  // Check for errors
  if (output.errors) {
    const errors = output.errors.filter(error => error.severity === 'error');
    if (errors.length > 0) {
      console.error('Compilation errors:');
      errors.forEach(error => console.error(error.formattedMessage));
      process.exit(1);
    }
    
    // Log warnings
    const warnings = output.errors.filter(error => error.severity === 'warning');
    if (warnings.length > 0) {
      console.warn('Compilation warnings:');
      warnings.forEach(warning => console.warn(warning.formattedMessage));
    }
  }
  
  // Extract the contract data
  const contractOutput = output.contracts['QuantumChessToken.sol']['QuantumChessToken'];
  
  // Save the ABI and bytecode to files
  fs.writeFileSync(
    path.join(contractOutputDir, 'QuantumChessToken.abi.json'),
    JSON.stringify(contractOutput.abi, null, 2)
  );
  
  fs.writeFileSync(
    path.join(contractOutputDir, 'QuantumChessToken.bin'),
    contractOutput.evm.bytecode.object
  );
  
  console.log('Contract compiled successfully.');
  console.log(`ABI saved to ${path.join(contractOutputDir, 'QuantumChessToken.abi.json')}`);
  console.log(`Bytecode saved to ${path.join(contractOutputDir, 'QuantumChessToken.bin')}`);
  return {
    abi: contractOutput.abi,
    bytecode: contractOutput.evm.bytecode.object
  };
}

/**
 * Deploy the contract to the Core Testnet
 */
async function deployContract(abi, bytecode) {
  console.log(`Connecting to Core Testnet at ${rpcUrl}...`);
  
  // Define contractAddress at the beginning to ensure it's in scope for the return statement
  let contractAddress;
  
  // Connect to the provider
  const provider = new ethers.JsonRpcProvider(rpcUrl);
  
  // Create a wallet from the private key
  const wallet = new ethers.Wallet(privateKey, provider);
  const address = wallet.address;
  
  console.log(`Using wallet address: ${address}`);
  
  // Check wallet balance
  const balance = await provider.getBalance(address);
  const balanceEth = ethers.formatEther(balance);
  
  console.log(`Wallet balance: ${balanceEth} ETH`);
  
  if (balance === 0n) {
    console.error('ERROR: Wallet has zero balance. Please fund your wallet before deployment.');
    process.exit(1);
  }
  
  // Create contract factory
  const factory = new ethers.ContractFactory(abi, bytecode, wallet);
  
  // Estimate gas for deployment
  console.log('Estimating gas for deployment...');
  
  try {
    const estimatedGas = await provider.estimateGas({
      from: address,
      data: '0x' + bytecode
    });
    
    // Add 20% safety buffer
    const gasLimit = estimatedGas * 12n / 10n;
    
    console.log(`Estimated gas: ${estimatedGas.toString()}`);
    console.log(`Gas limit with safety buffer: ${gasLimit.toString()}`);
    
    // Deploy the contract with the estimated gas and wallet address as initialMinter
    console.log('Deploying contract...');
    console.log(`Setting initialMinter to wallet address: ${address}`);
    const contract = await factory.deploy(address, { gasLimit });
    
    const deployTxResponse = await contract.deploymentTransaction();
    console.log(`Transaction hash: ${deployTxResponse.hash}`);
    console.log('Waiting for confirmation...');
    
    // Wait for deployment to complete
    await contract.waitForDeployment();
    
    console.log(`Contract deployed successfully at address: ${await contract.getAddress()}`);
    
    // Update .env file with the contract address
    try {
      const envPath = path.resolve(__dirname, '..', '.env');
      let envContent = '';
      
      contractAddress = await contract.getAddress();
      
      if (fs.existsSync(envPath)) {
        envContent = fs.readFileSync(envPath, 'utf8');
        
        // Replace or add the contract address
        if (envContent.includes('CORE_CONTRACT_ADDRESS=')) {
          envContent = envContent.replace(
            /CORE_CONTRACT_ADDRESS=.*/,
            `CORE_CONTRACT_ADDRESS=${contractAddress}`
          );
        } else {
          envContent += `\nCORE_CONTRACT_ADDRESS=${contractAddress}\n`;
        }
      } else {
        envContent = `CORE_CONTRACT_ADDRESS=${contractAddress}\n`;
      }
      
      fs.writeFileSync(envPath, envContent);
      console.log(`Updated .env file with contract address.`);
    } catch (error) {
      console.warn(`Warning: Could not update .env file: ${error.message}`);
    }
    
    return contractAddress;
  } catch (error) {
    console.error('ERROR: Deployment failed:', error.message);
    
    if (error.message.includes('insufficient funds')) {
      console.error('Your wallet does not have enough ETH to cover the gas costs.');
    }
    
    process.exit(1);
  }
}

/**
 * Main function
 */
async function main() {
  try {
    console.log('='.repeat(60));
    console.log('QuantumChessToken Contract Deployment');
    console.log('='.repeat(60));
    
    // Compile the contract
    const { abi, bytecode } = await compileContract();
    
    // Deploy the contract
    const contractAddress = await deployContract(abi, bytecode);
    
    console.log('='.repeat(60));
    console.log(`Deployment successful!`);
    console.log(`Contract address: ${contractAddress}`);
    console.log('='.repeat(60));
    
    // Create a package.json for easy running with npm
    const packageJsonPath = path.resolve(__dirname, 'package.json');
    if (!fs.existsSync(packageJsonPath)) {
      const packageJson = {
        name: "quantum-chess-deployer",
        version: "1.0.0",
        description: "Deployment tool for QuantumChess contract",
        main: "deploy_quantum_chess.js",
        scripts: {
          "deploy": "node deploy_quantum_chess.js"
        },
        dependencies: {
          "dotenv": "^16.0.3",
          "ethers": "^6.13.5",
          "solc": "^0.8.19"
        }
      };
      
      fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));
      console.log('Created package.json for easy deployment with npm');
      console.log('You can now run deployment with: npm run deploy');
    }
    
  } catch (error) {
    console.error('ERROR:', error.message);
    process.exit(1);
  }
}

// Run the main function
main();

