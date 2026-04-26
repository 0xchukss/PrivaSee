#!/bin/bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
echo "=== Building Solana Program ==="
cargo build-sbf
