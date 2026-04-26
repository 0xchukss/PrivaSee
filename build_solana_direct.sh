#!/bin/bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis/programs/privasee"
echo "=== Building Solana Program in programs/privasee ==="
cargo build-sbf
