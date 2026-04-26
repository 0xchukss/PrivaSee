# arcium-anchor

[![Crates.io](https://img.shields.io/crates/v/arcium-anchor.svg)](https://crates.io/crates/arcium-anchor)

A helper crate for integrating Arcium into Solana programs. Provides utilities, traits, and account types that simplify the development of Anchor-based Solana programs that interact with the Arcium network for encrypted computations.

## Usage

```rust
use arcium_anchor::{
    queue_computation, init_comp_def,
    SignedComputationOutputs, SharedEncryptedStruct, MXEEncryptedStruct,
    traits::{QueueCompAccs, CallbackCompAccs, InitCompDefAccs},
    prelude::*,
};

// Initialize a computation definition
init_comp_def(&ctx.accounts, circuit_source_override, finalize_authority)?;

// Queue a computation for execution
queue_computation(
    &ctx.accounts,
    computation_offset,
    args,
    callback_instructions,
    num_callback_txs,
    cu_price_micro,
)?;

// Verify and deserialize computation results
let output: MyOutputType = computation_output
    .verify_output(&ctx.accounts.cluster, &ctx.accounts.computation)?;
```

## Main Exports

### Core Functions

- `queue_computation()` - Queue an encrypted computation for execution
- `init_comp_def()` - Initialize a computation definition on-chain
- `comp_def_offset()` - Calculate computation definition account offset

### Types

- `SignedComputationOutputs<O>` - Enum for computation results (Success/Failure) with BLS signature verification
- `SharedEncryptedStruct<const LEN: usize>` - Container for shared encrypted data
- `MXEEncryptedStruct<const LEN: usize>` - Container for MXE encrypted data

### Traits

- `QueueCompAccs` - Trait for accounts that can queue computations
- `CallbackCompAccs` - Trait for accounts that handle computation callbacks
- `InitCompDefAccs` - Trait for accounts that can initialize computation definitions

### PDA Utilities

Various helper macros for deriving Program Derived Addresses (PDAs) used by Arcium accounts.
