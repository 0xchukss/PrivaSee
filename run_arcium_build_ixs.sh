#!/bin/bash
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.cargo/bin:$PATH"

cd "/mnt/c/Users/Hp/Desktop/arcium contact dis/encrypted-ixs"
echo "=== Running Arcium Build in encrypted-ixs ==="
arcium build
