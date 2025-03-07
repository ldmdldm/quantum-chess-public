use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::{Result, anyhow};

use crate::errors::AppError;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::wallet::Wallet;

/// Represents a method that can be called on a smart contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMethod {
    /// Name of the method
    pub name: String,
    
    /// Method signature (function selector)
    pub signature: String,
    
    /// ABI encoded parameters
    pub parameters: Vec<Value>,
    
    /// Gas limit for the transaction
    pub gas_limit: u64,
    
    /// Optional fixed gas price (if None, the network's recommended gas price will be used)
    pub gas_price: Option<u64>,
}

impl ContractMethod {
    /// Create a new contract method with the given name and signature
    pub fn new(name: &str, signature: &str) -> Self {
        Self {
            name: name.to_string(),
            signature: signature.to_string(),
            parameters: Vec::new(),
            gas_limit: 250000, // Default gas limit
            gas_price: None,
        }
    }
    
    /// Add a parameter to the method call
    pub fn add_parameter<T: Serialize>(&mut self, param: T) -> Result<&mut Self> {
        let value = serde_json::to_value(param)?;
        self.parameters.push(value);
        Ok(self)
    }
    
    /// Set the gas limit for the method call
    pub fn with_gas_limit(&mut self, gas_limit: u64) -> &mut Self {
        self.gas_limit = gas_limit;
        self
    }
    
    /// Set a fixed gas price for the method call
    pub fn with_gas_price(&mut self, gas_price: u64) -> &mut Self {
        self.gas_price = Some(gas_price);
        self
    }
    
    /// Encode the method call to ABI format
    pub fn encode(&self) -> Result<Vec<u8>> {
        // In a real implementation, this would use the proper ABI encoding
        // For now, we'll just serialize to JSON as a placeholder
        let encoded = serde_json::to_vec(self)?;
        Ok(encoded)
    }
}

/// Represents a smart contract on the Core blockchain
#[derive(Debug, Clone)]
pub struct SmartContract {
    /// Contract address on the blockchain
    pub address: String,
    
    /// Contract ABI (Application Binary Interface)
    pub abi: HashMap<String, String>,
    
    /// Chain ID of the network where the contract is deployed
    pub chain_id: u64,
}

impl SmartContract {
    /// Create a new smart contract instance
    pub fn new(address: &str, chain_id: u64) -> Self {
        Self {
            address: address.to_string(),
            abi: HashMap::new(),
            chain_id,
        }
    }
    
    /// Load the contract ABI from a JSON string
    pub fn with_abi_from_json(&mut self, abi_json: &str) -> Result<&mut Self> {
        let abi: HashMap<String, String> = serde_json::from_str(abi_json)?;
        self.abi = abi;
        Ok(self)
    }
    
    /// Get a method object for calling a contract method
    pub fn method(&self, name: &str) -> Result<ContractMethod> {
        let signature = self.abi.get(name)
            .ok_or_else(|| anyhow!("Method {} not found in contract ABI", name))?;
            
        Ok(ContractMethod::new(name, signature))
    }
    
    /// Call a method on the contract (read-only, no state changes)
    pub fn call_method(&self, method: &ContractMethod) -> Result<Value> {
        // In a real implementation, this would make an RPC call to the blockchain
        // For now, we'll return a placeholder response
        log::info!("Calling method {} on contract {}", method.name, self.address);
        
        // Mock response for different methods
        match method.name.as_str() {
            "getGameState" => Ok(serde_json::json!({
                "status": "ACTIVE",
                "currentTurn": "WHITE",
                "moveCount": 10,
                "totalStake": 100,
            })),
            "getQuantumState" => Ok(serde_json::json!({
                "superpositions": 2,
                "entanglements": 1,
                "uncertainty": 0.75,
            })),
            _ => Err(anyhow!("Unsupported method: {}", method.name)),
        }
    }
    
    /// Send a transaction to call a method on the contract (can change state)
    pub fn send_transaction(&self, method: &ContractMethod, wallet: &Wallet) -> Result<Transaction> {
        let encoded_data = method.encode()?;
        
        // Create a transaction
        let transaction = Transaction {
            from: wallet.address().to_string(),
            to: self.address.clone(),
            value: 0, // No ETH being sent
            data: hex::encode(encoded_data),
            gas_limit: method.gas_limit,
            gas_price: method.gas_price.unwrap_or(1_000_000_000), // Default to 1 Gwei
            nonce: 0, // In a real implementation, this would be fetched from the network
            chain_id: self.chain_id,
            hash: String::new(), // Will be set when signed
        };
        
        // In a real implementation, the transaction would be signed and sent to the network
        log::info!("Sending transaction to method {} on contract {}", method.name, self.address);
        
        Ok(transaction)
    }
    
    /// Record a chess move on the blockchain
    pub fn record_move(&self, wallet: &Wallet, game_id: &str, from: &str, to: &str, probability: f64) -> Result<Transaction> {
        let mut method = self.method("recordMove")?;
        
        method.add_parameter(game_id)?
              .add_parameter(from)?
              .add_parameter(to)?
              .add_parameter(probability)?
              .with_gas_limit(300000); // Slightly higher gas limit for game moves
              
        self.send_transaction(&method, wallet)
    }
    
    /// Place a stake for a game
    pub fn place_stake(&self, wallet: &Wallet, game_id: &str, amount: u64) -> Result<Transaction> {
        let mut method = self.method("placeStake")?;
        
        method.add_parameter(game_id)?
              .add_parameter(amount)?
              .with_gas_limit(200000);
              
        self.send_transaction(&method, wallet)
    }
    
    /// Create a new game on the blockchain
    pub fn create_game(&self, wallet: &Wallet, initial_stake: u64, time_control: u64) -> Result<Transaction> {
        let mut method = self.method("createGame")?;
        
        method.add_parameter(initial_stake)?
              .add_parameter(time_control)?
              .with_gas_limit(500000); // Higher gas limit for contract deployment
              
        self.send_transaction(&method, wallet)
    }
    
    /// Join an existing game
    pub fn join_game(&self, wallet: &Wallet, game_id: &str, stake_amount: u64) -> Result<Transaction> {
        let mut method = self.method("joinGame")?;
        
        method.add_parameter(game_id)?
              .add_parameter(stake_amount)?
              .with_gas_limit(250000);
              
        self.send_transaction(&method, wallet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contract_method() {
        let mut method = ContractMethod::new("testMethod", "test(address,uint256)");
        method.add_parameter("0x1234567890").unwrap()
              .add_parameter(100).unwrap()
              .with_gas_limit(300000);
              
        assert_eq!(method.name, "testMethod");
        assert_eq!(method.parameters.len(), 2);
        assert_eq!(method.gas_limit, 300000);
    }
    
    #[test]
    fn test_smart_contract() {
        let mut contract = SmartContract::new("0x1234567890", 1);
        contract.with_abi_from_json(r#"{"testMethod": "test(address,uint256)"}"#).unwrap();
        
        assert_eq!(contract.address, "0x1234567890");
        assert_eq!(contract.chain_id, 1);
        assert!(contract.abi.contains_key("testMethod"));
    }
}

