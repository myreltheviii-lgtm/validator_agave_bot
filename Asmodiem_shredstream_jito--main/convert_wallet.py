#!/usr/bin/env python3
import base58
import json
import getpass

try:
    # Get private key (hidden input)
    key = getpass.getpass("Enter your Phantom private key (hidden): ")
    bytes_array = list(base58.b58decode(key))
    
    # Save to file
    with open('/workspaces/Asmodiem_shredstream_jito-/wallet.json', 'w') as f:
        json.dump(bytes_array, f)
    
    print("✅ Wallet saved to /workspaces/Asmodiem_shredstream_jito-/wallet.json")
    print("\nNow verify with this command:")
    print("solana-keygen pubkey /workspaces/Asmodiem_shredstream_jito-/wallet.json")
    
except Exception as e:
    print(f"❌ Error: {e}")
