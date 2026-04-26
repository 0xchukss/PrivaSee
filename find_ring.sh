#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"
cd "/mnt/c/Users/Hp/Desktop/arcium contact dis/programs/privasee"
echo "=== Finding ring dependencies ==="
cargo tree -p privasee -i ring
