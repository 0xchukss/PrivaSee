#!/bin/bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
cargo update -p base64ct@1.8.3 --precise 1.6.0 || true
cargo update -p const-oid@0.10.2 --precise 0.9.6 || true
