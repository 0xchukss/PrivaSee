#!/bin/bash
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.cargo/bin:$PATH"

cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
echo "=== Running Arcium Build ==="
arcium build
