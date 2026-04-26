#!/bin/bash
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.local/share/solana/install/active_release/bin:$HOME/.arcium/bin:$PATH"

cd "/mnt/c/Users/Hp/Desktop/arcium contact dis/programs/privasee"
cargo build-sbf 2>&1
