#!/bin/bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis"
rm Cargo.lock 2>/dev/null || true
cargo generate-lockfile
cargo update -p const-oid --precise 0.9.6
cargo update -p base64ct --precise 1.6.0
cargo build-sbf
