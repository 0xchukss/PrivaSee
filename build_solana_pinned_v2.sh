#!/bin/bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
echo "=== Pinning dependencies for Solana ==="
cargo update -p base64ct --precise 1.6.0
cargo update -p const-oid --precise 0.9.6
cargo update -p der@0.8.0-rc.1 --precise 0.7.10
cargo update -p spki@0.8.0-rc.1 --precise 0.7.3
cargo update -p pkcs8@0.11.0-rc.1 --precise 0.10.2
echo "=== Building Solana Program ==="
cargo build-sbf
