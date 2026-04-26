#!/bin/bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
cargo update -p base64ct --precise 1.6.0
cargo update -p const-oid --precise 0.9.6
