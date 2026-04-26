#!/bin/bash
set -e
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.local/share/solana/install/active_release/bin:$HOME/.arcium/bin:$PATH"

echo "Copying arcis file..."
cp build/compute_psi.arcis build/compute_psi_testnet.arcis

echo "Running cargo build-sbf..."
cd programs/privasee
cargo build-sbf 2>&1
