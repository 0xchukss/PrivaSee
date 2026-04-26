# PrivaSee — Private Contact Discovery on Solana

> **Find mutual friends without exposing your contacts.** Powered by [Arcium](https://arcium.com) MPC on Solana.

[![Solana](https://img.shields.io/badge/Solana-Devnet-purple)](https://solana.com)
[![Arcium](https://img.shields.io/badge/Arcium-MPC-teal)](https://arcium.com)
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

## The Problem

Every social app asks for your contact list. Once you hand it over, you lose control — your connections are harvested, sold, or leaked. Even "privacy-focused" apps often process contacts server-side, creating a honeypot of social graph data.

**There has to be a better way.**

## The Solution: PrivaSee

PrivaSee uses **Private Set Intersection (PSI)** via **Multi-Party Computation (MPC)** to let two users discover mutual contacts without revealing their full contact lists to anyone — not to each other, not to the app, not to any server.

### Privacy Guarantee

> **Arcium's MPC nodes process only ciphertexts — your contact list is mathematically provably never reconstructible by any single party.**

Under Arcium's dishonest majority model, privacy holds even if all but one MPC node are compromised. The only information revealed is the intersection — which contacts you have in common.

## How It Works

```
┌──────────────────────────────────────────────────────────────┐
│                       CLIENT BROWSER                         │
│                                                              │
│  Contacts ──→ SHA3 Hash ──→ X25519+Rescue Encrypt ──→ ████  │
│                                                      │       │
│  Decrypted Matches ←── Rescue Decrypt ←──────────────┼──┐    │
└──────────────────────────────────────────────────────┼──┼────┘
                                                       │  │
                                                       ▼  │
┌──────────────────────────────────────────────────────────┼───┐
│                    SOLANA BLOCKCHAIN                      │   │
│                                                          │   │
│  PrivaSee Program (MXE)                                  │   │
│  ├─ register_contacts(encrypted_set) ──→ UserRegistry    │   │
│  ├─ request_psi(user_a, user_b)      ──→ Queue Comp      │   │
│  └─ compute_psi_callback(result)     ──→ PsiResultEvent──┘   │
│                         │                                     │
│                         ▼                                     │
│              Arcium Program (CPI)                             │
│              └─ queue_computation()                            │
└──────────────────────┬───────────────────────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────────────────────┐
│                   ARCIUM MPC CLUSTER                         │
│                                                              │
│  Node A: share₁ ──┐                                         │
│  Node B: share₂ ──┼──→ compute_psi(████, ████) ──→ result   │
│  Node C: share₃ ──┘                                         │
│                                                              │
│  ⚡ No node sees plaintext contacts                          │
│  ⚡ Only encrypted intersection returned                     │
└──────────────────────────────────────────────────────────────┘
```

### Step-by-Step Flow

1. **User connects** their Solana wallet (Phantom/Backpack)
2. **User inputs contacts** — wallet addresses are hashed with SHA3-256 **client-side**
3. **Hashes are encrypted** using X25519 key exchange + Rescue cipher (Arcium's encryption)
4. **Encrypted set is submitted** to the Solana program and stored on-chain
5. **User requests PSI** against another registered user
6. **Arcium's MPC cluster** executes the `compute_psi` circuit on encrypted data
7. **Only the intersection** is returned as encrypted ciphertext
8. **Client decrypts** the result locally — only mutual matches are displayed

### What Data Never Leaves the Client

| Data | Leaves Browser? | Stored On-Chain? |
|------|-----------------|------------------|
| Raw contact addresses | ❌ Never | ❌ Never |
| SHA3 hashes (plaintext) | ❌ Never | ❌ Never |
| Encrypted ciphertexts | ✅ (encrypted) | ✅ (encrypted) |
| Encryption private key | ❌ Never | ❌ Never |
| PSI match results | ❌ Never (plaintext) | ❌ Never (plaintext) |

## How Arcium Is Used

### Private Set Intersection via MPC

The core innovation is using Arcium's MPC infrastructure for **PSI** — a cryptographic protocol that computes the intersection of two sets without revealing the sets themselves.

**Arcis Circuit** (`encrypted-ixs/src/lib.rs`):
- Written in Rust using the Arcis framework (`#[encrypted]` module + `#[instruction]` macro)
- Takes two `Enc<Shared, ContactSet>` parameters — encrypted contact sets
- Compares each contact hash pair using element-wise equality
- Returns `Enc<Shared, PsiResult>` — only match flags and matched hashes
- Both branches of conditionals always execute (MPC requirement)
- Fixed-size arrays `[[u8; 32]; 20]` used (no Vec/String in MPC circuits)

**Solana Program** (`programs/privasee/src/lib.rs`):
- Uses `#[arcium_program]` macro (extends Anchor)
- `register_contacts` stores encrypted commitments on-chain
- `request_psi` queues computation via `queue_computation()` CPI
- `compute_psi_callback` receives verified results from MPC cluster
- Uses `ArgBuilder` for encrypted argument construction
- Proper `#[queue_computation_accounts]` and `#[callback_accounts]` macros

**TypeScript Client** (`app/utils/arcium.ts`):
- X25519 key exchange with MXE public key
- RescueCipher for symmetric encryption
- `awaitComputationFinalization()` for result polling
- Client-side decryption of PSI results

## Innovation

**PSI for social contact discovery is a novel, real-world Arcium use case.** While PSI has been studied in academic cryptography for decades, applying it to Web3 social graphs via on-chain MPC is new. PrivaSee demonstrates that:

1. **Privacy-preserving social features are possible** without centralized servers
2. **MPC can be practical** for consumer applications (not just DeFi)
3. **Arcium's infrastructure** handles the cryptographic complexity transparently

## Project Structure

```
privasee/
├── programs/privasee/
│   ├── src/lib.rs              # Solana program (MXE) — Anchor + Arcium
│   └── Cargo.toml              # Rust dependencies
├── encrypted-ixs/
│   └── src/lib.rs              # Arcis PSI circuit — runs in MPC
├── app/                        # Next.js frontend
│   ├── page.tsx                # Landing page
│   ├── register/page.tsx       # Contact registration
│   ├── discover/page.tsx       # PSI discovery
│   ├── components/
│   │   ├── PrivacyShield.tsx   # Animated MPC status indicator
│   │   ├── ContactInput.tsx    # Multi-address input
│   │   ├── MatchCard.tsx       # Match result cards
│   │   ├── Navbar.tsx          # Navigation
│   │   └── WalletProvider.tsx  # Solana wallet adapter
│   └── utils/
│       ├── arcium.ts           # Arcium SDK helpers
│       └── crypto.ts           # Client-side SHA3 hashing
├── tests/
│   └── privasee.ts             # Integration tests
├── Anchor.toml                 # Anchor configuration (devnet)
├── Arcium.toml                 # Arcium cluster configuration
├── package.json
├── tailwind.config.js
└── README.md
```

## Setup & Deployment

### Prerequisites

- **Rust**: [Install](https://www.rust-lang.org/tools/install)
- **Solana CLI 2.3.0**: [Install](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor 0.32.1**: [Install](https://www.anchor-lang.com/docs/installation)
- **Node.js 18+**: [Install](https://nodejs.org)
- **Docker & Docker Compose**: Required for local Arcium testing

### Install Arcium Toolchain

```bash
# Install arcup (Arcium version manager)
curl --proto '=https' --tlsv1.2 -sSfL https://install.arcium.com/ | bash

# Install latest Arcium CLI
arcup install

# Verify
arcium --version
```

### Build & Test Locally

```bash
# Clone the repo
git clone <repo-url> && cd privasee

# Install JS dependencies
npm install

# Build the Arcis circuit and Solana program
arcium build

# Run tests against local Docker cluster
arcium test
```

### Deploy to Devnet

```bash
# Generate a keypair if needed
solana-keygen new

# Airdrop SOL for deployment (need 2-5 SOL)
solana airdrop 5 --url devnet

# Deploy program
arcium deploy --cluster devnet

# Initialize the computation definition (one-time)
# This registers the PSI circuit with the Arcium network
npx ts-node scripts/init-comp-def.ts

# Run tests against devnet
arcium test --cluster devnet
```

### Run Frontend

```bash
# Install dependencies
npm install

# Start dev server
npm run dev

# Open http://localhost:3000
```

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Blockchain | Solana | On-chain state & computation coordination |
| MPC Framework | Arcium (Arcis) | Confidential computation on encrypted data |
| Smart Contract | Anchor 0.32.1 | Solana program framework |
| Frontend | Next.js + TypeScript | User interface |
| Styling | Tailwind CSS | Dark-mode, glassmorphism design |
| Wallet | Solana Wallet Adapter | Phantom/Backpack connectivity |
| Encryption | X25519 + Rescue Cipher | Client-side data encryption |
| Hashing | SHA3-256 (Keccak) | Contact address hashing |

## License

MIT
