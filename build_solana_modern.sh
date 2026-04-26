#!/bin/bash
export PATH="$HOME/.cargo/bin:$HOME/.local/share/solana/install/active_release/bin:$PATH"
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
echo "=== Building Solana Program with 1.89.0 Toolchain ==="
rustup run 1.89.0-sbpf-solana-v1.52 cargo build-sbf
