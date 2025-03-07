use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use hex::{encode, decode};
use crate::errors::AppError;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Key generation error: {0}")]
    KeyGenerationError(String),
    
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    
    #[error("Signing error: {0}")]
    SigningError(String),
    
    #[error("Verification error: {0}")]
    VerificationError(String),
    
    #[error("Hex decoding error: {0}")]
    HexDecodingError(String),
}

impl From<WalletError> for AppError {
    fn from(err: WalletError) -> Self {
        AppError::BlockchainError(format!("Wallet error: {}", err))
    }
}

/// Represents a cryptographic key pair for blockchain operations
pub struct KeyPair {
    /// The underlying ed25519 keypair
    keypair: Keypair,
}

impl KeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Result<Self, WalletError> {
        let mut csprng = OsRng {};
        
        let keypair = Keypair::generate(&mut csprng);
        
        Ok(Self { keypair })
    }
    
    /// Create a keypair from an existing secret key
    pub fn from_secret_key(secret_key_hex: &str) -> Result<Self, WalletError> {
        let secret_bytes = decode(secret_key_hex)
            .map_err(|e| WalletError::InvalidKeyFormat(e.to_string()))?;
            
        if secret_bytes.len() != 32 {
            return Err(WalletError::InvalidKeyFormat(
                format!("Secret key must be 32 bytes, got {}", secret_bytes.len())
            ));
        }
        
        let secret_key = SecretKey::from_bytes(&secret_bytes)
            .map_err(|e| WalletError::InvalidKeyFormat(e.to_string()))?;
            
        let public_key = PublicKey::from(&secret_key);
        let keypair = Keypair { secret: secret_key, public: public_key };
        
        Ok(Self { keypair })
    }
    
    /// Get the public key as a hex string
    pub fn public_key_hex(&self) -> String {
        encode(self.keypair.public.as_bytes())
    }
    
    /// Get the secret key as a hex string
    pub fn secret_key_hex(&self) -> String {
        encode(self.keypair.secret.as_bytes())
    }
    
    /// Sign a message using the private key
    pub fn sign(&self, message: &[u8]) -> Result<String, WalletError> {
        let signature = self.keypair.sign(message);
        Ok(encode(signature.to_bytes()))
    }
    
    /// Verify a message signature
    pub fn verify(&self, message: &[u8], signature_hex: &str) -> Result<bool, WalletError> {
        let signature_bytes = decode(signature_hex)
            .map_err(|e| WalletError::HexDecodingError(e.to_string()))?;
            
        let signature = Signature::from_bytes(&signature_bytes)
            .map_err(|e| WalletError::VerificationError(e.to_string()))?;
            
        match self.keypair.public.verify(message, &signature) {
            Ok(_) => Ok(true),
            Err(e) => Err(WalletError::VerificationError(e.to_string())),
        }
    }
    
    /// Derive an address from the public key
    pub fn derive_address(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.keypair.public.as_bytes());
        let result = hasher.finalize();
        
        // Take the first 20 bytes of the hash as the address (similar to Ethereum)
        let address_bytes = &result[0..20];
        format!("0x{}", encode(address_bytes))
    }
}

/// Represents a blockchain wallet for the Core network
pub struct Wallet {
    /// The wallet's keypair
    keypair: KeyPair,
    /// The derived blockchain address
    address: String,
    /// The associated balance in Core tokens
    balance: Option<u64>,
}

impl Wallet {
    /// Create a new wallet with a random keypair
    pub fn new() -> Result<Self, WalletError> {
        let keypair = KeyPair::generate()?;
        let address = keypair.derive_address();
        
        Ok(Self {
            keypair,
            address,
            balance: None,
        })
    }
    
    /// Create a wallet from an existing secret key
    pub fn from_secret_key(secret_key_hex: &str) -> Result<Self, WalletError> {
        let keypair = KeyPair::from_secret_key(secret_key_hex)?;
        let address = keypair.derive_address();
        
        Ok(Self {
            keypair,
            address,
            balance: None,
        })
    }
    
    /// Get the wallet's address
    pub fn address(&self) -> &str {
        &self.address
    }
    
    /// Get the wallet's keypair
    pub fn keypair(&self) -> &KeyPair {
        &self.keypair
    }
    
    /// Get the wallet's balance
    pub fn balance(&self) -> Option<u64> {
        self.balance
    }
    
    /// Set the wallet's balance (after querying the blockchain)
    pub fn set_balance(&mut self, balance: u64) {
        self.balance = Some(balance);
    }
    
    /// Sign a transaction or message
    pub fn sign(&self, message: &[u8]) -> Result<String, WalletError> {
        self.keypair.sign(message)
    }
    
    /// Sign a transaction using Core blockchain format
    pub fn sign_transaction(&self, transaction_data: &[u8]) -> Result<String, WalletError> {
        // For Core blockchain, we hash the transaction data first and then sign it
        let mut hasher = Sha256::new();
        hasher.update(transaction_data);
        let hash = hasher.finalize();
        
        self.keypair.sign(&hash)
    }
    
    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature_hex: &str) -> Result<bool, WalletError> {
        self.keypair.verify(message, signature_hex)
    }
    
    /// Export the wallet as a JSON string
    pub fn export_json(&self) -> Result<String, WalletError> {
        let wallet_export = serde_json::json!({
            "address": self.address,
            "public_key": self.keypair.public_key_hex(),
            "private_key": self.keypair.secret_key_hex(),
            "balance": self.balance
        });
        
        serde_json::to_string(&wallet_export)
            .map_err(|e| WalletError::KeyGenerationError(e.to_string()))
    }
}

impl fmt::Debug for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wallet")
            .field("address", &self.address)
            .field("public_key", &self.keypair.public_key_hex())
            .field("balance", &self.balance)
            // Note: We don't display the private key for security reasons
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = KeyPair::generate().unwrap();
        assert!(!keypair.public_key_hex().is_empty());
        assert!(!keypair.secret_key_hex().is_empty());
    }

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new().unwrap();
        assert!(wallet.address().starts_with("0x"));
        assert_eq!(wallet.address().len(), 42); // "0x" + 40 hex chars
    }

    #[test]
    fn test_signing_and_verification() {
        let wallet = Wallet::new().unwrap();
        let message = b"Test message for Core blockchain";
        
        let signature = wallet.sign(message).unwrap();
        assert!(wallet.verify(message, &signature).unwrap());
    }

    #[test]
    fn test_wallet_from_private_key() {
        let original_wallet = Wallet::new().unwrap();
        let secret_key = original_wallet.keypair().secret_key_hex();
        
        let imported_wallet = Wallet::from_secret_key(&secret_key).unwrap();
        assert_eq!(original_wallet.address(), imported_wallet.address());
    }
}

