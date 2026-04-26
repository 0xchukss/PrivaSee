#!/bin/bash
set -e

source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.local/share/solana/install/active_release/bin:$HOME/.arcium/bin:$PATH"

echo "=== Configuring Solana for Devnet ==="
solana config set --url devnet

# Check/create keypair
if [ ! -f "$HOME/.config/solana/id.json" ]; then
    echo "=== Generating new keypair ==="
    solana-keygen new --no-bip39-passphrase
else
    echo "=== Existing keypair found ==="
fi

echo "Wallet: $(solana address || true)"
# echo "Balance: $(solana balance || true)"

# Airdrop if balance is low
# BALANCE=$(solana balance | grep -oP '[\d.]+' || echo "2")
# echo "Current balance: $BALANCE SOL"
# if (( $(echo "$BALANCE < 2" | bc -l) )); then
#     echo "=== Requesting devnet airdrop ==="
#     solana airdrop 2 || echo "Airdrop failed - may need to use faucet.solana.com"
#     sleep 3
#     echo "New balance: $(solana balance || true)"
# fi

echo ""
echo "=== Building PrivaSee with Arcium ==="
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
arcium build || true

echo "=== Copying Arcis File ==="
cp build/compute_psi.arcis build/compute_psi_testnet.arcis

echo "=== Building PrivaSee ==="
cd programs/privasee
cargo build-sbf > build_privasee.log 2>&1

echo ""
echo "=== Build complete! ==="
