use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use sha2::{Sha256, Digest};
use log::debug;

use crate::errors::AppError;
use crate::blockchain::wallet::KeyPair;

/// Represents the status of a blockchain transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    /// Transaction has been created but not yet submitted
    Created,
    /// Transaction has been submitted to the blockchain
    Pending,
    /// Transaction has been confirmed
    Confirmed(u64), // Block number
    /// Transaction has failed
    Failed(String), // Error message
    /// Transaction has been dropped from the mempool
    Dropped,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionStatus::Created => write!(f, "Created"),
            TransactionStatus::Pending => write!(f, "Pending"),
            TransactionStatus::Confirmed(block) => write!(f, "Confirmed (Block #{})", block),
            TransactionStatus::Failed(reason) => write!(f, "Failed: {}", reason),
            TransactionStatus::Dropped => write!(f, "Dropped"),
        }
    }
}

/// Represents a blockchain transaction for the quantum chess game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash (id)
    pub hash: String,
    /// Address of the sender
    pub from: String,
    /// Address of the recipient (if applicable)
    pub to: Option<String>,
    /// Transaction value (in Core tokens)
    pub value: u64,
    /// Gas limit for transaction execution
    pub gas_limit: u64,
    /// Gas price in Core token gwei
    pub gas_price: u64,
    /// Transaction nonce
    pub nonce: u64,
    /// Transaction data (hex-encoded contract call)
    pub data: String,
    /// Unix timestamp when the transaction was created
    pub timestamp: u64,
    /// Current status of the transaction
    pub status: TransactionStatus,
    /// Chain ID for EIP-155 replay protection
    pub chain_id: u64,
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(
        from: String,
        to: Option<String>,
        value: u64,
        gas_limit: u64,
        gas_price: u64,
        nonce: u64,
        data: String,
        chain_id: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let mut tx = Transaction {
            hash: String::new(),
            from,
            to,
            value,
            gas_limit,
            gas_price,
            nonce,
            data,
            timestamp,
            status: TransactionStatus::Created,
            chain_id,
        };
        
        // Calculate hash
        tx.hash = tx.calculate_hash();
        tx
    }
    
    /// Calculates the transaction hash
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Create a string representation to hash
        let tx_string = format!(
            "from:{},to:{},value:{},gas_limit:{},gas_price:{},nonce:{},data:{},timestamp:{},chain_id:{}",
            self.from,
            self.to.clone().unwrap_or_default(),
            self.value,
            self.gas_limit,
            self.gas_price,
            self.nonce,
            self.data,
            self.timestamp,
            self.chain_id
        );
        
        hasher.update(tx_string.as_bytes());
        let result = hasher.finalize();
        format!("0x{:x}", result)
    }
    
    /// Signs the transaction with the given key pair
    pub fn sign(&self, key_pair: &KeyPair) -> Result<String, AppError> {
        debug!("Signing transaction: {}", self.hash);
        
        // In an actual implementation, we would create the RLP encoding
        // of the transaction and sign it with the private key
        
        // For now, we'll just return a mock signature
        let signature = format!("0xsignature_for_{}", self.hash);
        Ok(signature)
    }
    
    /// Submits the transaction to the blockchain
    pub async fn submit(&mut self, signed_tx: &str) -> Result<(), AppError> {
        debug!("Submitting transaction to blockchain: {}", self.hash);
        
        // In a real implementation, we would send the signed transaction to the Core blockchain node
        
        // Update status
        self.status = TransactionStatus::Pending;
        
        Ok(())
    }
    
    /// Checks the status of the transaction on the blockchain
    pub async fn check_status(&mut self) -> Result<TransactionStatus, AppError> {
        debug!("Checking status of transaction: {}", self.hash);
        
        // In a real implementation, we would query the Core blockchain for the transaction status
        
        // Mock implementation for demonstration
        // In a real app, this would query the Core blockchain
        
        match self.status {
            TransactionStatus::Pending => {
                // Simulate a confirmed transaction after some time
                if SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() > self.timestamp + 30
                {
                    self.status = TransactionStatus::Confirmed(100); // Mock block number
                }
            }
            _ => {} // No change for other statuses
        }
        
        Ok(self.status.clone())
    }
    
    /// Estimate the gas required for this transaction
    pub async fn estimate_gas(&self) -> Result<u64, AppError> {
        debug!("Estimating gas for transaction type: {}", self.data);
        
        // In a real implementation, we would call the Core blockchain's eth_estimateGas method
        
        // Mock implementation
        let estimated_gas = match self.to {
            Some(_) => {
                if !self.data.is_empty() {
                    // Contract call
                    100_000
                } else {
                    // Simple transfer
                    21_000
                }
            }
            None => {
                // Contract creation
                200_000
            }
        };
        
        Ok(estimated_gas)
    }
    
    /// Creates a stake transaction
    pub fn create_stake_transaction(
        from: String,
        contract_address: String,
        stake_amount: u64,
        game_id: String,
        nonce: u64,
        chain_id: u64,
    ) -> Self {
        // In a real implementation, we would encode the contract call data
        // This is a simplified example
        let data = format!("stake({},{})", game_id, stake_amount);
        
        Self::new(
            from,
            Some(contract_address),
            stake_amount,
            150_000, // Gas limit for stake operation
            5,       // Gas price in gwei
            nonce,
            data,
            chain_id,
        )
    }
    
    /// Creates a transaction to record a chess move on the blockchain
    pub fn create_move_transaction(
        from: String,
        contract_address: String,
        game_id: String,
        move_notation: String,
        position_hash: String,
        nonce: u64,
        chain_id: u64,
    ) -> Self {
        // Encode the contract call data
        let data = format!("recordMove({},{},{})", game_id, move_notation, position_hash);
        
        Self::new(
            from,
            Some(contract_address),
            0,       // No value transfer for recording moves
            200_000, // Gas limit for recording move
            5,       // Gas price in gwei
            nonce,
            data,
            chain_id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            "0x1234...".to_string(),
            Some("0x5678...".to_string()),
            100,
            21_000,
            5,
            0,
            "".to_string(),
            1,
        );
        
        assert_eq!(tx.status, TransactionStatus::Created);
        assert!(!tx.hash.is_empty());
    }
    
    #[test]
    fn test_transaction_status_display() {
        assert_eq!(format!("{}", TransactionStatus::Created), "Created");
        assert_eq!(format!("{}", TransactionStatus::Pending), "Pending");
        assert_eq!(format!("{}", TransactionStatus::Confirmed(123)), "Confirmed (Block #123)");
        assert_eq!(format!("{}", TransactionStatus::Failed("Out of gas".to_string())), "Failed: Out of gas");
        assert_eq!(format!("{}", TransactionStatus::Dropped), "Dropped");
    }
}

