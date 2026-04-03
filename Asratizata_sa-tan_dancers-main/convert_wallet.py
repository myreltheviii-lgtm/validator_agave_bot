#!/usr/bin/env python3
import base58
import json
import getpass

try:
    key = getpass.getpass("Enter your Phantom private key (hidden): ")
    bytes_array = list(base58.b58decode(key))
    
    with open('/root/mev-wallet.json', 'w') as f:
        json.dump(bytes_array, f)
    
    print("✅ Wallet saved to /root/mev-wallet.json")
    print("\nNow verify with this command:")
    print("solana-keygen pubkey /root/mev-wallet.json")
    
except Exception as e:
    print(f"❌ Error: {e}")
