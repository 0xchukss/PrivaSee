//! Code around generating and using the Program Derived Addresses (PDAs) for the Arcium programs.

use crate::{idl::arcium::ID_CONST as ARCIUM_ID, utils::sha256};
#[cfg(feature = "transactions")]
use anchor_client::solana_sdk::signature::Keypair;
use anchor_lang::solana_program::pubkey::Pubkey;
use const_crypto::ed25519;
use solana_address_lookup_table_interface::program::ID as ADDRESS_LOOKUP_TABLE_PROGRAM_ID;

const fn parse_const_pda(pda: ([u8; 32], u8)) -> (Pubkey, u8) {
    (Pubkey::new_from_array(pda.0), pda.1)
}

pub const CLOCK_PDA: (Pubkey, u8) = parse_const_pda(ed25519::derive_program_address(
    &[b"ClockAccount"],
    &ARCIUM_ID.to_bytes(),
));

pub const FEE_POOL_PDA: (Pubkey, u8) = parse_const_pda(ed25519::derive_program_address(
    &[b"FeePool"],
    &ARCIUM_ID.to_bytes(),
));

pub const ARCIUM_TOKEN_MINT: Pubkey = Pubkey::new_from_array([
    160, 125, 200, 55, 211, 178, 66, 27, 149, 22, 219, 191, 28, 218, 171, 113, 92, 216, 236, 165,
    124, 20, 89, 205, 119, 106, 175, 166, 185, 155, 69, 242,
]);

pub fn arx_acc(node_offset: u32) -> Pubkey {
    arx_acc_w_bump(node_offset).0
}

pub fn arx_acc_w_bump(node_offset: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"ArxNode", node_offset.to_le_bytes().as_ref()],
        &ARCIUM_ID,
    )
}

pub fn recovery_peer_acc(peer_offset: u32) -> Pubkey {
    recovery_peer_acc_w_bump(peer_offset).0
}

pub fn recovery_peer_acc_w_bump(peer_offset: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"RecoveryPeerAccount", peer_offset.to_le_bytes().as_ref()],
        &ARCIUM_ID,
    )
}

pub fn operator_acc(owner_key: &Pubkey) -> Pubkey {
    operator_acc_w_bump(owner_key).0
}

pub fn operator_acc_w_bump(owner_key: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"Operator", owner_key.as_ref()], &ARCIUM_ID)
}

pub fn cluster_acc(cluster_offset: u32) -> Pubkey {
    cluster_acc_w_bump(cluster_offset).0
}

pub fn cluster_acc_w_bump(cluster_offset: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"Cluster", cluster_offset.to_le_bytes().as_ref()],
        &ARCIUM_ID,
    )
}

pub fn mxe_acc(mxe_program: &Pubkey) -> Pubkey {
    mxe_acc_w_bump(mxe_program).0
}

pub fn mxe_acc_w_bump(mxe_program: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"MXEAccount", mxe_program.to_bytes().as_ref()],
        &ARCIUM_ID,
    )
}

/// Derives the address lookup table (LUT) address for an MXE program.
pub fn mxe_lut_acc(mxe_program: &Pubkey, recent_slot: u64) -> Pubkey {
    mxe_lut_acc_w_bump(mxe_program, recent_slot).0
}

pub fn mxe_lut_acc_w_bump(mxe_program: &Pubkey, recent_slot: u64) -> (Pubkey, u8) {
    let mxe_account = mxe_acc(mxe_program);
    Pubkey::find_program_address(
        &[mxe_account.as_ref(), &recent_slot.to_le_bytes()],
        &ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
    )
}

pub fn recovery_cluster_acc(mxe_program: &Pubkey) -> Pubkey {
    recovery_cluster_acc_w_bump(mxe_program).0
}

pub fn recovery_cluster_acc_w_bump(mxe_program: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"RecoveryClusterAccount", mxe_program.to_bytes().as_ref()],
        &ARCIUM_ID,
    )
}

pub fn mxe_recovery_acc(backup_mxe_program: &Pubkey, original_mxe_program: &Pubkey) -> Pubkey {
    mxe_recovery_acc_w_bump(backup_mxe_program, original_mxe_program).0
}

pub fn mxe_recovery_acc_w_bump(
    backup_mxe_program: &Pubkey,
    original_mxe_program: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"MxeRecoveryAccount",
            backup_mxe_program.to_bytes().as_ref(),
            original_mxe_program.to_bytes().as_ref(),
        ],
        &ARCIUM_ID,
    )
}

pub fn mempool_acc(cluster_offset: u32) -> Pubkey {
    mempool_acc_w_bump(cluster_offset).0
}

pub fn mempool_acc_w_bump(cluster_offset: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"Mempool", cluster_offset.to_le_bytes().as_ref()],
        &ARCIUM_ID,
    )
}

pub fn execpool_acc(cluster_offset: u32) -> Pubkey {
    execpool_acc_w_bump(cluster_offset).0
}

pub fn execpool_acc_w_bump(cluster_offset: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"Execpool", cluster_offset.to_le_bytes().as_ref()],
        &ARCIUM_ID,
    )
}

pub fn computation_acc(cluster_offset: u32, computation_offset: u64) -> Pubkey {
    computation_acc_w_bump(cluster_offset, computation_offset).0
}

pub fn computation_acc_w_bump(cluster_offset: u32, computation_offset: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"ComputationAccount",
            cluster_offset.to_le_bytes().as_ref(),
            computation_offset.to_le_bytes().as_ref(),
        ],
        &ARCIUM_ID,
    )
}

pub fn failure_claim_acc(mxe_program_id: &Pubkey, computation_offset: u64) -> Pubkey {
    failure_claim_acc_w_bump(mxe_program_id, computation_offset).0
}

pub fn failure_claim_acc_w_bump(mxe_program_id: &Pubkey, computation_offset: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"FailureClaimAccountHeader",
            mxe_program_id.to_bytes().as_ref(),
            computation_offset.to_le_bytes().as_ref(),
        ],
        &ARCIUM_ID,
    )
}

pub fn computation_definition_acc(
    mxe_program: &Pubkey,
    computation_definition_offset: u32,
) -> Pubkey {
    computation_definition_acc_w_bump(mxe_program, computation_definition_offset).0
}

pub fn computation_definition_acc_w_bump(
    mxe_program: &Pubkey,
    computation_definition_offset: u32,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"ComputationDefinitionAccount",
            mxe_program.to_bytes().as_ref(),
            computation_definition_offset.to_le_bytes().as_ref(),
        ],
        &ARCIUM_ID,
    )
}

pub fn signer_acc(mxe_program: &Pubkey) -> Pubkey {
    signer_acc_w_bump(mxe_program).0
}

pub fn signer_acc_w_bump(mxe_program: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"ArciumSignerAccount"], mxe_program)
}

pub fn raw_circuit_acc(computation_definition_acc: &Pubkey, circuit_chunk_index: u8) -> Pubkey {
    raw_circuit_acc_w_bump(computation_definition_acc, circuit_chunk_index).0
}

pub fn raw_circuit_acc_w_bump(
    computation_definition_acc: &Pubkey,
    circuit_chunk_index: u8,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"ComputationDefinitionRaw",
            computation_definition_acc.as_ref(),
            circuit_chunk_index.to_le_bytes().as_ref(),
        ],
        &ARCIUM_ID,
    )
}

pub const fn clock_acc() -> Pubkey {
    clock_acc_w_bump().0
}

pub const fn clock_acc_w_bump() -> (Pubkey, u8) {
    CLOCK_PDA
}

pub const fn fee_pool_acc() -> Pubkey {
    fee_pool_acc_w_bump().0
}

pub const fn fee_pool_acc_w_bump() -> (Pubkey, u8) {
    FEE_POOL_PDA
}

#[cfg(feature = "transactions")]
pub fn arcium_mint_keypair() -> Keypair {
    #[allow(deprecated)]
    Keypair::from_bytes(&[
        233, 132, 53, 39, 177, 254, 146, 147, 56, 5, 201, 25, 151, 108, 175, 134, 226, 255, 11,
        184, 116, 200, 236, 178, 88, 203, 30, 213, 123, 29, 34, 101, 160, 125, 200, 55, 211, 178,
        66, 27, 149, 22, 219, 191, 28, 218, 171, 113, 92, 216, 236, 165, 124, 20, 89, 205, 119,
        106, 175, 166, 185, 155, 69, 242,
    ])
    .expect("Failed to create arcium mint keypair from bytes")
}

pub fn comp_def_offset(circuit_name: &str) -> u32 {
    let result = sha256(&[circuit_name.as_bytes()]);
    u32::from_le_bytes([result[0], result[1], result[2], result[3]])
}

#[cfg(feature = "staking")]
pub mod staking {
    use super::*;
    use crate::idl::arcium_staking::ID as ARCIUM_STAKING_ID;

    pub const STAKING_POOL_PDA: (Pubkey, u8) = parse_const_pda(ed25519::derive_program_address(
        &[b"StakingPoolAccount"],
        &ARCIUM_STAKING_ID.to_bytes(),
    ));

    pub const fn staking_pool_acc() -> Pubkey {
        staking_pool_acc_w_bump().0
    }

    pub const fn staking_pool_acc_w_bump() -> (Pubkey, u8) {
        STAKING_POOL_PDA
    }

    pub fn delegated_stake_acc(stake_offset: u128) -> Pubkey {
        delegated_stake_acc_w_bump(stake_offset).0
    }

    pub fn delegated_stake_acc_w_bump(stake_offset: u128) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"DelegatedStakingAccount",
                stake_offset.to_le_bytes().as_ref(),
            ],
            &ARCIUM_STAKING_ID,
        )
    }

    pub fn stake_master_acc(owner_key: &Pubkey) -> Pubkey {
        stake_master_acc_w_bump(owner_key).0
    }

    pub fn stake_master_acc_w_bump(owner_key: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"DelegationMasterAccount", owner_key.as_ref()],
            &ARCIUM_STAKING_ID,
        )
    }

    pub fn primary_stake_acc(owner_key: &Pubkey) -> Pubkey {
        primary_stake_acc_w_bump(owner_key).0
    }

    pub fn primary_stake_acc_w_bump(owner_key: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"PrimaryStakingAccount", owner_key.as_ref()],
            &ARCIUM_STAKING_ID,
        )
    }

    pub fn stake_queue_acc(primary_stake_acc: &Pubkey) -> Pubkey {
        stake_queue_acc_w_bump(primary_stake_acc).0
    }

    pub fn stake_queue_acc_w_bump(primary_stake_acc: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"StakeQueueAccount", primary_stake_acc.as_ref()],
            &ARCIUM_STAKING_ID,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "transactions")]
    use anchor_client::solana_sdk::signer::Signer;

    #[cfg(feature = "transactions")]
    #[test]
    fn test_arcium_mint_keypair() {
        let kp = arcium_mint_keypair();
        assert_eq!(kp.pubkey(), ARCIUM_TOKEN_MINT);
    }
}
