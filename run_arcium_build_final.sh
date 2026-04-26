#!/bin/bash
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.local/share/solana/install/active_release/bin:$HOME/.cargo/bin:$PATH"

cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
echo "=== Running Arcium Build from Root ==="
arcium build
