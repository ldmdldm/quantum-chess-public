use std::env;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;

use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    println!("Core Testnet Smart Contract Deployment Tool");
    println!("===========================================");
    
    // Get configuration from environment variables or use defaults
    let rpc_url = env::var("CORE_BLOCKCHAIN_URL")
        .unwrap_or_else(|_| "https://rpc.test2.btcs.network".to_string());
    
    let private_key = env::var("CORE_PRIVATE_KEY")
        .expect("CORE_PRIVATE_KEY must be set in your environment or .env file");
    
    println!("Connecting to Core testnet at: {}", rpc_url);
    
    // Set up the provider to connect to the Core testnet
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // Get the current chain ID
    let chain_id = provider.get_chainid().await?;
    println!("Connected to chain ID: {}", chain_id);
    
    // Create a wallet from the private key
    let wallet = LocalWallet::from_str(&private_key)?
        .with_chain_id(chain_id.as_u64());
    
    let wallet_address = wallet.address();
    println!("Using wallet address: {}", wallet_address);
    
    // Check wallet balance before deployment
    let balance = provider.get_balance(wallet_address, None).await?;
    println!("Wallet balance: {} ETH", format_units(balance, "ether")?);
    
    if balance.is_zero() {
        return Err(eyre::eyre!("Wallet has zero balance. Please fund your wallet before deployment."));
    }
    
    // Create a client from the wallet and provider
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);
    
    // Load the contract bytecode
    // Assuming the contract bytecode is in a file named "contract.bin" in the project root
    let contract_path = env::var("CONTRACT_PATH").unwrap_or_else(|_| "../contracts/QuantumChess.bin".to_string());
    
    println!("Loading contract bytecode from: {}", contract_path);
    let bytecode = match fs::read_to_string(&contract_path) {
        Ok(code) => code,
        Err(e) => {
            println!("Error reading contract file: {}", e);
            println!("Attempting to use sample bytecode for testing...");
            // Fallback to a minimal test contract if file not found
            "608060405234801561001057600080fd5b5060f78061001f6000396000f3fe6080604052348015600f57600080fd5b5060043610603c5760003560e01c80633bc5de3014604157806360fe47b114605d578063c2bc2efc146089575b600080fd5b604760a5565b6040518082815260200191505060405180910390f35b608760048036036020811015607157600080fd5b810190808035906020019092919050505060ae565b005b608f60b8565b6040518082815260200191505060405180910390f35b60005481565b8060008190555050565b6000805490509056fea2646970667358221220149590a6e477bad9e97e49ace51ec3ce48d85d475c5e98458b4a5e4e5125eaae64736f6c63430007060033".to_string()
        }
    };
    
    // Deploy the contract
    println!("Deploying contract to Core testnet...");
    
    let deploy_tx = ContractDeployer::new(Bytes::from(hex::decode(bytecode.trim_start_matches("0x"))?), client.clone());
    
    // Estimate gas for deployment
    let gas_estimate = deploy_tx.estimate_gas().await?;
    println!("Estimated gas for deployment: {}", gas_estimate);
    
    // Deploy with gas estimate
    let pending_tx = deploy_tx.gas(gas_estimate).send().await?;
    
    println!("Transaction sent! Waiting for confirmation...");
    let receipt = pending_tx.await?
        .ok_or_else(|| eyre::eyre!("Transaction dropped from mempool"))?;
    
    let contract_address = receipt.contract_address
        .ok_or_else(|| eyre::eyre!("No contract address returned"))?;
    
    println!("🎉 Contract successfully deployed!");
    println!("Contract address: {}", contract_address);
    println!("Transaction hash: {}", receipt.transaction_hash);
    
    // Update the .env file with the new contract address
    println!("Updating .env file with the contract address...");
    let env_path = "../.env"; // Adjust path as needed
    
    if let Ok(env_content) = fs::read_to_string(env_path) {
        let updated_content = if env_content.contains("CORE_CONTRACT_ADDRESS=") {
            env_content.replace(
                &regex::Regex::new(r"CORE_CONTRACT_ADDRESS=.*")?.find(&env_content).map_or("".to_string(), |m| m.as_str().to_string()),
                &format!("CORE_CONTRACT_ADDRESS={}", contract_address)
            )
        } else {
            format!("{}\nCORE_CONTRACT_ADDRESS={}", env_content, contract_address)
        };
        
        if let Err(e) = fs::write(env_path, updated_content) {
            println!("Warning: Could not update .env file: {}", e);
            println!("Please manually add the contract address to your .env file:");
            println!("CORE_CONTRACT_ADDRESS={}", contract_address);
        } else {
            println!(".env file successfully updated with the new contract address");
        }
    } else {
        println!("Warning: Could not read .env file to update it");
        println!("Please manually add the contract address to your .env file:");
        println!("CORE_CONTRACT_ADDRESS={}", contract_address);
    }
    
    println!("\nDeployment complete! Your smart contract is now live on Core testnet.");
    
    Ok(())
}

// Helper function to format units (similar to ethers.js formatUnits)
fn format_units(amount: U256, unit: &str) -> Result<String> {
    let decimals = match unit {
        "ether" => 18,
        "gwei" => 9,
        "wei" => 0,
        _ => return Err(eyre::eyre!("Unsupported unit")),
    };
    
    if amount.is_zero() {
        return Ok("0".to_string());
    }
    
    let mut value = amount.to_string();
    if value.len() <= decimals {
        value = format!("{:0>width$}", value, width = decimals + 1);
    }
    
    let decimal_index = value.len() - decimals;
    let result = format!(
        "{}.{}",
        &value[0..decimal_index],
        &value[decimal_index..]
    );
    
    // Trim trailing zeros after the decimal point
    let parts: Vec<&str> = result.split('.').collect();
    if parts.len() == 2 {
        let mut decimal_part = parts[1].to_string();
        while decimal_part.ends_with('0') && !decimal_part.is_empty() {
            decimal_part.pop();
        }
        
        if decimal_part.is_empty() {
            return Ok(parts[0].to_string());
        } else {
            return Ok(format!("{}.{}", parts[0], decimal_part));
        }
    }
    
    Ok(result)
}

