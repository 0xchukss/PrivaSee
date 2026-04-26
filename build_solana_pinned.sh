#!/bin/bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
echo "=== Pinning dependencies for Solana ==="
cargo update -p base64ct --precise 1.6.0
cargo update -p der --precise 0.7.8
cargo update -p spki --precise 0.7.3
cargo update -p pkcs8 --precise 0.10.2
echo "=== Building Solana Program ==="
cargo build-sbf
