#!/usr/bin/env python3
import os
import hashlib
import binascii

print("Generating new Core blockchain wallet for testnet...")

# Generate random private key (32 bytes)
private_key_bytes = os.urandom(32)
private_key_hex = binascii.hexlify(private_key_bytes).decode('utf-8')

# Derive public key (in a real implementation this would use proper elliptic curve crypto)
# For demo purposes, we'll just hash the private key
public_key_hash = hashlib.sha256(private_key_bytes).digest()

# Take first 20 bytes for the address (similar to Ethereum)
address_bytes = public_key_hash[:20]
address = "0x" + binascii.hexlify(address_bytes).decode('utf-8')

# Display wallet information
print("\n=== New Core Blockchain Wallet ===")
print(f"Address: {address}")
print(f"Private Key: {private_key_hex}")
print("\nIMPORTANT: Save your private key securely. It cannot be recovered if lost!")
print("This wallet is intended for testnet use only.")
