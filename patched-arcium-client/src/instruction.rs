//! Instruction builders for the Arcium programs.
use crate::{
    idl::arcium::{
        accounts::MXEAccount,
        client::{
            accounts::{
                ActivateArx as ActivateArxAccs,
                ActivateCluster as ActivateClusterAccs,
                BumpEpochCluster as BumpEpochClusterAccs,
                CallbackComputation as CallbackComputationAccs,
                ClaimFailureAppend as ClaimFailureAppendAccs,
                ClaimFailureFinalize as ClaimFailureFinalizeAccs,
                ClaimFailureInit as ClaimFailureInitAccs,
                ClaimNodeFees as ClaimNodeFeesAccs,
                CloseKeyRecovery as CloseKeyRecoveryAccs,
                DeactivateArx as DeactivateArxAccs,
                DeactivateCluster as DeactivateClusterAccs,
                EmbiggenRawCircuitAcc as EmbiggenRawCircuitAccAccs,
                ExtendRecoveryKeyshares as ExtendRecoveryKeysharesAccs,
                FinalizeComputationDefinition as FinalizeComputationDefinitionAccs,
                FinalizeKeyRecoveryCallback as FinalizeKeyRecoveryCallbackAccs,
                FinalizeKeyRecoveryExecution as FinalizeKeyRecoveryExecutionAccs,
                FinalizeKeyRecoverySharesUpload as FinalizeKeyRecoverySharesUploadAccs,
                FinalizeMxeKeys as FinalizeMxeKeysAccs,
                IncreaseMempoolSize as IncreaseMempoolSizeAccs,
                Init as InitNetworkProgramAccs,
                InitArxNode as InitArxNodeAccs,
                InitCluster as InitClusterAccs,
                InitComputationDefinition as InitComputationDefinitionAccs,
                InitKeyRecoveryExecutionPart1 as InitKeyRecoveryExecutionPart1Accs,
                InitKeyRecoveryExecutionPart2 as InitKeyRecoveryExecutionPart2Accs,
                InitMxePart1 as InitMxePart1Accs,
                InitMxePart2 as InitMxePart2Accs,
                InitOperator as InitOperatorAccs,
                InitRawCircuitAcc as InitRawCircuitAccs,
                InitRecoveryPeerAccount as InitRecoveryPeerAccountAccs,
                JoinCluster as JoinClusterAccs,
                ProposeFee as ProposeFeeAccs,
                ProposeJoinCluster as ProposeJoinClusterAccs,
                QueueComputation as QueueComputationAccs,
                QueueKeyRecoveryInit as QueueKeyRecoveryInitAccs,
                ReclaimFailureRentIdempotent as ReclaimFailureRentIdempotentAccs,
                RecoverMxe as RecoverMxeAccs,
                RequeueKeyRecoveryFinalize as RequeueKeyRecoveryFinalizeAccs,
                RequeueMxeKeygen as RequeueMxeKeygenAccs,
                SetArxNodeConfig as SetArxNodeConfigAccs,
                SetArxNodeMetadata as SetArxNodeMetadataAccs,
                SetCluster as SetClusterAccs,
                SetClusterAuthority as SetClusterAuthorityAccs,
                SetMxeKeys as SetMxeKeysAccs,
                SetMxeRecoveryKeysInit as SetMxeRecoveryKeysInitAccs,
                SubmitAggregatedBlsPubkey as SubmitAggregatedBlsPubkeyAccs,
                SubmitKeyRecoveryShare as SubmitKeyRecoveryShareAccs,
                UpdateCurrentEpochIdempotent as UpdateCurrentEpochIdempotentAccs,
                UploadCircuit as UploadCircuitAccs,
                VoteFee as VoteFeeAccs,
            },
            args::{
                ActivateArx as ActivateArxArgs,
                ActivateCluster as ActivateClusterArgs,
                BumpEpochCluster as BumpEpochClusterArgs,
                CallbackComputation as CallbackComputationArgs,
                ClaimFailureAppend as ClaimFailureAppendArgs,
                ClaimFailureFinalize as ClaimFailureFinalizeArgs,
                ClaimFailureInit as ClaimFailureInitArgs,
                ClaimNodeFees as ClaimNodeFeesArgs,
                CloseKeyRecovery as CloseKeyRecoveryArgs,
                DeactivateArx as DeactivateArxArgs,
                DeactivateCluster as DeactivateClusterArgs,
                EmbiggenRawCircuitAcc as EmbiggenRawCircuitAccArgs,
                ExtendRecoveryKeyshares as ExtendRecoveryKeysharesArgs,
                FinalizeComputationDefinition as FinalizeComputationDefinitionArgs,
                FinalizeKeyRecoveryCallback as FinalizeKeyRecoveryCallbackArgs,
                FinalizeKeyRecoveryExecution as FinalizeKeyRecoveryExecutionArgs,
                FinalizeKeyRecoverySharesUpload as FinalizeKeyRecoverySharesUploadArgs,
                FinalizeMxeKeys as FinalizeMxeKeysArgs,
                IncreaseMempoolSize as IncreaseMempoolSizeArgs,
                Init as InitNetworkProgramArgs,
                InitArxNode as InitArxNodeArgs,
                InitCluster as InitClusterArgs,
                InitComputationDefinition as InitComputationDefinitionArgs,
                InitKeyRecoveryExecutionPart1 as InitKeyRecoveryExecutionPart1Args,
                InitKeyRecoveryExecutionPart2 as InitKeyRecoveryExecutionPart2Args,
                InitMxePart1 as InitMxePart1Args,
                InitMxePart2 as InitMxePart2Args,
                InitOperator as InitOperatorArgs,
                InitRawCircuitAcc as InitRawCircuitArgs,
                InitRecoveryPeerAccount as InitRecoveryPeerAccountArgs,
                JoinCluster as JoinClusterArgs,
                ProposeFee as ProposeFeeArgs,
                ProposeJoinCluster as ProposeJoinClusterArgs,
                QueueComputation as QueueComputationArgs,
                QueueKeyRecoveryInit as QueueKeyRecoveryInitArgs,
                ReclaimFailureRentIdempotent as ReclaimFailureRentIdempotentArgs,
                RecoverMxe as RecoverMxeArgs,
                RequeueKeyRecoveryFinalize as RequeueKeyRecoveryFinalizeArgs,
                RequeueMxeKeygen as RequeueMxeKeygenArgs,
                SetArxNodeConfig as SetArxNodeConfigArgs,
                SetArxNodeMetadata as SetArxNodeMetadataArgs,
                SetCluster as SetClusterArgs,
                SetClusterAuthority as SetClusterAuthorityArgs,
                SetMxeKeys as SetMxeKeysArgs,
                SetMxeRecoveryKeysInit as SetMxeRecoveryKeysInitArgs,
                SubmitAggregatedBlsPubkey as SubmitAggregatedBlsPubkeyArgs,
                SubmitKeyRecoveryShare as SubmitKeyRecoveryShareArgs,
                UpdateCurrentEpochIdempotent as UpdateCurrentEpochIdempotentArgs,
                UploadCircuit as UploadCircuitArgs,
                VoteFee as VoteFeeArgs,
            },
        },
    },
};
pub use crate::idl::arcium::types::{CallbackAccount, CallbackInstruction};
use crate::{
    idl::arcium::{
        types::{
            ArgumentList,
            ArxNodeConfig,
            BN254G2BLSPublicKey,
            CircuitSource,
            ComputationDefinitionMeta,
            ComputationSignature,
            Epoch,
            ExecutionStatus,
            MempoolSize,
            NodeMetadata,
            OperatorMeta,
            Output,
            Parameter,
            Timestamp,
        },
        ID as ARCIUM_PROG_ID,
    },
    pda::{
        arx_acc,
        clock_acc,
        cluster_acc,
        computation_acc,
        computation_definition_acc,
        execpool_acc,
        failure_claim_acc,
        fee_pool_acc,
        mempool_acc,
        mxe_acc,
        mxe_lut_acc,
        mxe_recovery_acc,
        operator_acc,
        raw_circuit_acc,
        recovery_cluster_acc,
        recovery_peer_acc,
        signer_acc,
    },
    utils::MAX_RECOVERY_PEERS,
};
#[allow(deprecated)]
use solana_program::bpf_loader_upgradeable;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar,
};
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::instructions::ID as INSTRUCTIONS_SYSVAR_ID,
    InstructionData,
};
const MXE_KEYS_ENC_COUNT: usize = 13;
use solana_program::address_lookup_table::program::ID as ADDRESS_LOOKUP_TABLE_PROGRAM_ID;
use std::vec;

pub const ARCIUM_TOKEN_DECIMALS: u8 = 9;

pub fn init_network_program_ix(current_timestamp: u64, signer: &Pubkey) -> Instruction {
    let pool_acc = fee_pool_acc();
    let accounts = InitNetworkProgramAccs {
        signer: signer.to_owned(),
        fee_pool: pool_acc,
        system_program: SYSTEM_PROGRAM_ID,
        clock: clock_acc(),
    }
    .to_account_metas(None);
    let data = InitNetworkProgramArgs {
        start_epoch_timestamp: Timestamp {
            timestamp: current_timestamp,
        },
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

// Location as [ISO 3166-1 alpha-2](https://www.iso.org/iso-3166-country-codes.html) country code
pub fn init_node_operator_acc_ix(signer: &Pubkey, url: String, location: u8) -> Instruction {
    let accounts = InitOperatorAccs {
        signer: signer.to_owned(),
        operator_acc: operator_acc(signer),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitOperatorArgs {
        meta: OperatorMeta { url, location },
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn init_arx_node_acc_ix(
    operator_signer: &Pubkey,
    node_signer: &Pubkey,
    node_offset: u32,
    callback_authority: &Pubkey,
    cu_claim: u64,
    bls_pubkey: BN254G2BLSPublicKey,
    metadata: NodeMetadata,
    x25519_pubkey: [u8; 32],
) -> Instruction {
    let accounts = InitArxNodeAccs {
        operator_signer: operator_signer.to_owned(),
        operator_acc: operator_acc(operator_signer),
        arx_node_acc: arx_acc(node_offset),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let config = ArxNodeConfig {
        authority: node_signer.to_owned(),
        callback_authority: callback_authority.to_owned(),
    };
    let data = InitArxNodeArgs {
        node_offset,
        cu_capacity_claim: cu_claim,
        config,
        bls_pubkey,
        metadata,
        x25519_pubkey,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn init_cluster_ix(
    signer: &Pubkey,
    cluster_authority: Pubkey,
    cluster_offset: u32,
    cluster_size: u16,
    cu_price: u64,
    mempool_size: MempoolSize,
    td_info: Option<NodeMetadata>,
) -> Instruction {
    let accounts = InitClusterAccs {
        signer: signer.to_owned(),
        cluster_acc: cluster_acc(cluster_offset),
        authority: cluster_authority,
        pool_account: fee_pool_acc(),
        system_program: SYSTEM_PROGRAM_ID,
        mempool: mempool_acc(cluster_offset),
        execpool: execpool_acc(cluster_offset),
        clock: clock_acc(),
    }
    .to_account_metas(None);
    let data = InitClusterArgs {
        cluster_size,
        cluster_id: cluster_offset,
        cu_price,
        mempool_size,
        td_info,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn init_recovery_peer_acc_ix(
    signer: &Pubkey,
    peer_offset: u32,
    x25519_pubkey: [u8; 32],
) -> Instruction {
    let accounts = InitRecoveryPeerAccountAccs {
        signer: signer.to_owned(),
        recovery_peer_account: recovery_peer_acc(peer_offset),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitRecoveryPeerAccountArgs {
        peer_offset,
        authority: *signer,
        x25519_pubkey,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn set_cluster_authority_ix(
    signer: &Pubkey,
    cluster_offset: u32,
    new_authority: Option<Pubkey>,
) -> Instruction {
    let accounts = SetClusterAuthorityAccs {
        current_authority: signer.to_owned(),
        cluster_acc: cluster_acc(cluster_offset),
    }
    .to_account_metas(None);
    let data = SetClusterAuthorityArgs {
        cluster_id: cluster_offset,
        new_authority,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn init_mxe_part1_ix(signer: &Pubkey, mxe_program: &Pubkey) -> Instruction {
    let accounts = InitMxePart1Accs {
        signer: signer.to_owned(),
        recovery_cluster_acc: recovery_cluster_acc(mxe_program),
        mxe_program: *mxe_program,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitMxePart1Args {}.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn init_mxe_part2_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    cluster_offset: u32,
    recovery_peers: [u32; MAX_RECOVERY_PEERS],
    keygen_offset: u64,
    key_recovery_init_offset: u64,
    mxe_authority: Option<Pubkey>,
    recent_slot: u64,
) -> Instruction {
    // Derive program_data PDA for upgrade authority verification
    let (program_data, _) =
        Pubkey::find_program_address(&[mxe_program.as_ref()], &bpf_loader_upgradeable::id());

    let accounts = InitMxePart2Accs {
        signer: signer.to_owned(),
        address_lookup_table: mxe_lut_acc(mxe_program, recent_slot),
        cluster: cluster_acc(cluster_offset),
        mxe: mxe_acc(mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(mxe_program),
        executing_pool: execpool_acc(cluster_offset),
        mempool: mempool_acc(cluster_offset),
        mxe_keygen_computation_definition: computation_definition_acc(mxe_program, 1),
        mxe_keygen_computation: computation_acc(cluster_offset, keygen_offset),
        key_recovery_init_computation: computation_acc(cluster_offset, key_recovery_init_offset),
        mxe_authority,
        mxe_program: *mxe_program,
        program_data,
        clock: clock_acc(),
        pool_account: fee_pool_acc(),
        system_program: SYSTEM_PROGRAM_ID,
        lut_program: ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitMxePart2Args {
        cluster_offset,
        mxe_program: *mxe_program,
        recovery_peers,
        keygen_offset,
        key_recovery_init_offset,
        recent_offset: recent_slot,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn init_computation_definition_ix(
    signer: &Pubkey,
    comp_def_offset: u32,
    mxe_program: &Pubkey,
    circuit_len: u32,
    parameters: Vec<Parameter>,
    outputs: Vec<Output>,
    cu_amount: u64,
    finalization_authority: Option<Pubkey>,
    circuit_source_override: Option<CircuitSource>,
    lut_slot: u64,
) -> Instruction {
    let comp_def_acc = computation_definition_acc(mxe_program, comp_def_offset);

    let accounts = InitComputationDefinitionAccs {
        signer: signer.to_owned(),
        mxe: mxe_acc(mxe_program),
        address_lookup_table: mxe_lut_acc(mxe_program, lut_slot),
        comp_def_acc,
        system_program: SYSTEM_PROGRAM_ID,
        lut_program: ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitComputationDefinitionArgs {
        mxe_program: *mxe_program,
        comp_offset: comp_def_offset,
        computation_definition: ComputationDefinitionMeta {
            signature: ComputationSignature {
                parameters,
                outputs,
            },
            circuit_len,
        },
        circuit_source_override,
        finalization_authority,
        cu_amount,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn init_raw_circuit_acc_ix(
    signer: &Pubkey,
    comp_def_offset: u32,
    mxe_program_id: &Pubkey,
    circuit_chunk_index: u8,
) -> Instruction {
    let comp_def_acc = computation_definition_acc(mxe_program_id, comp_def_offset);
    let accounts = InitRawCircuitAccs {
        signer: signer.to_owned(),
        comp_def_acc,
        comp_def_raw: raw_circuit_acc(&comp_def_acc, circuit_chunk_index),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitRawCircuitArgs {
        comp_offset: comp_def_offset,
        mxe_program: mxe_program_id.to_owned(),
        raw_circuit_index: circuit_chunk_index,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn upload_circuit_ix(
    signer: &Pubkey,
    mxe_program_id: &Pubkey,
    comp_def_offset: u32,
    circuit_chunk_index: u8,
    upload_data: [u8; 814],
    offset: u32,
) -> Instruction {
    let comp_def_acc = computation_definition_acc(mxe_program_id, comp_def_offset);
    let accounts = UploadCircuitAccs {
        signer: signer.to_owned(),
        comp_def_raw: raw_circuit_acc(&comp_def_acc, circuit_chunk_index),
        comp_def_acc,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = UploadCircuitArgs {
        comp_offset: comp_def_offset,
        mxe_program: mxe_program_id.to_owned(),
        raw_circuit_index: circuit_chunk_index,
        upload_data,
        offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn embiggen_raw_circuit_acc_ix(
    signer: &Pubkey,
    comp_def_offset: u32,
    mxe_program_id: &Pubkey,
    circuit_chunk_index: u8,
) -> Instruction {
    let comp_def_acc = computation_definition_acc(mxe_program_id, comp_def_offset);

    let accounts = EmbiggenRawCircuitAccAccs {
        signer: signer.to_owned(),
        comp_def_raw: raw_circuit_acc(&comp_def_acc, circuit_chunk_index),
        comp_def_acc,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);

    let data = EmbiggenRawCircuitAccArgs {
        comp_offset: comp_def_offset,
        mxe_program: mxe_program_id.to_owned(),
        raw_circuit_index: circuit_chunk_index,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn finalize_computation_definition_ix(
    signer: &Pubkey,
    comp_def_offset: u32,
    mxe_program: &Pubkey,
) -> Instruction {
    let comp_def_acc = computation_definition_acc(mxe_program, comp_def_offset);
    let accounts = FinalizeComputationDefinitionAccs {
        signer: signer.to_owned(),
        comp_def_acc,
        comp_def_raw: raw_circuit_acc(&comp_def_acc, 0),
    }
    .to_account_metas(None);

    let data = FinalizeComputationDefinitionArgs {
        comp_offset: comp_def_offset,
        mxe_program: mxe_program.to_owned(),
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn increase_mempool_size_ix(signer: &Pubkey, cluster_offset: u32) -> Instruction {
    let accounts = IncreaseMempoolSizeAccs {
        signer: signer.to_owned(),
        mempool: mempool_acc(cluster_offset),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = IncreaseMempoolSizeArgs { cluster_offset }.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Re-queues the MXE keygen computation if it has expired from the mempool.
/// This allows retrying the keygen if it wasn't processed in time.
pub fn requeue_mxe_keygen_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    cluster_offset: u32,
    computation_offset: u64,
) -> Instruction {
    let accounts = RequeueMxeKeygenAccs {
        signer: signer.to_owned(),
        mxe: mxe_acc(mxe_program),
        mempool: mempool_acc(cluster_offset),
        executing_pool: execpool_acc(cluster_offset),
        cluster: cluster_acc(cluster_offset),
        mxe_keygen_computation: computation_acc(cluster_offset, computation_offset),
        mxe_program: *mxe_program,
    }
    .to_account_metas(None);
    let data = RequeueMxeKeygenArgs { cluster_offset }.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn queue_key_recovery_init_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    cluster_offset: u32,
    key_recovery_init_offset: u64,
) -> Instruction {
    let accounts = QueueKeyRecoveryInitAccs {
        signer: signer.to_owned(),
        mxe: mxe_acc(mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(mxe_program),
        executing_pool: execpool_acc(cluster_offset),
        mempool: mempool_acc(cluster_offset),
        key_recovery_init_computation_definition: computation_definition_acc(mxe_program, 2),
        key_recovery_init_computation: computation_acc(cluster_offset, key_recovery_init_offset),
        mxe_program: *mxe_program,
        pool_account: fee_pool_acc(),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = QueueKeyRecoveryInitArgs {
        cluster_offset,
        mxe_program: *mxe_program,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn propose_join_cluster_ix(
    cluster_auth_signer: &Pubkey,
    cluster_offset: u32,
    node_offset: u32,
) -> Instruction {
    let accounts = ProposeJoinClusterAccs {
        cluster_authority: cluster_auth_signer.to_owned(),
        cluster_acc: cluster_acc(cluster_offset),
        arx_node_acc: arx_acc(node_offset),
        clock: clock_acc(),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = ProposeJoinClusterArgs {
        cluster_id: cluster_offset,
        node_bump: node_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn join_cluster_ix(
    node_auth_signer: &Pubkey,
    cluster_offset: u32,
    node_offset: u32,
    join: bool,
) -> Instruction {
    let accounts = JoinClusterAccs {
        node_authority: node_auth_signer.to_owned(),
        cluster_acc: cluster_acc(cluster_offset),
        arx_node_acc: arx_acc(node_offset),
        clock: clock_acc(),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = JoinClusterArgs {
        cluster_id: cluster_offset,
        node_bump: node_offset,
        join,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn queue_computation_ix(
    payer: &Pubkey,
    mxe_program: &Pubkey,
    computation_offset: u64,
    comp_def_offset: u32,
    args: ArgumentList,
    output_delivery_fee: u64,
    cu_price_micro: u64,
    mxe_data: MXEAccount,
    callback_instructions: Vec<CallbackInstruction>,
    callback_transaction_count_required: u8,
) -> Result<Instruction> {
    let cluster_offset = mxe_data.cluster.ok_or(ProgramError::InvalidAccountData)?;

    let accounts = QueueComputationAccs {
        signer: payer.to_owned(),
        sign_seed: signer_acc(mxe_program),
        cluster: cluster_acc(cluster_offset),
        mxe: mxe_acc(mxe_program),
        mempool: mempool_acc(cluster_offset),
        executing_pool: execpool_acc(cluster_offset),
        comp_def_acc: computation_definition_acc(mxe_program, comp_def_offset),
        pool_account: fee_pool_acc(),
        system_program: SYSTEM_PROGRAM_ID,
        clock: clock_acc(),
        comp: computation_acc(cluster_offset, computation_offset),
    }
    .to_account_metas(None);
    let data = QueueComputationArgs {
        mxe_program: *mxe_program,
        comp_offset: computation_offset,
        computation_definition_offset: comp_def_offset,
        output_delivery_fee,
        cu_price_micro,
        args,
        custom_callback_instructions: callback_instructions,
        callback_transactions_required: callback_transaction_count_required,
    }
    .data();

    Ok(Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    })
}

pub fn propose_fee_ix(
    node_auth_signer: Pubkey,
    cluster_offset: u32,
    node_offset: u32,
    proposed_fee: u64,
) -> Instruction {
    let accounts = ProposeFeeAccs {
        node_authority: node_auth_signer,
        cluster_acc: cluster_acc(cluster_offset),
        arx_node_acc: arx_acc(node_offset),
        clock: clock_acc(),
    }
    .to_account_metas(None);
    let data = ProposeFeeArgs {
        cluster_offset,
        node_offset,
        proposed_fee,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn vote_fee_ix(
    node_auth_signer: Pubkey,
    cluster_offset: u32,
    node_offset: u32,
    fee_vote: u64,
) -> Instruction {
    let accounts = VoteFeeAccs {
        node_authority: node_auth_signer,
        cluster_acc: cluster_acc(cluster_offset),
        arx_node_acc: arx_acc(node_offset),
        clock: clock_acc(),
    }
    .to_account_metas(None);
    let data = VoteFeeArgs {
        cluster_offset,
        node_offset,
        fee_vote,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn claim_node_fees_ix(
    node_authority: &Pubkey,
    cluster_offset: u32,
    node_offset: u32,
    recipient: &Pubkey,
) -> Instruction {
    let accounts = ClaimNodeFeesAccs {
        node_authority: *node_authority,
        recipient: *recipient,
        cluster: cluster_acc(cluster_offset),
        node: arx_acc(node_offset),
        pool: fee_pool_acc(),
    }
    .to_account_metas(None);

    let data = ClaimNodeFeesArgs {
        cluster_offset,
        node_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn bump_epoch_cluster_ix(cluster_offset: u32) -> Instruction {
    let accounts = BumpEpochClusterAccs {
        cluster_acc: cluster_acc(cluster_offset),
        clock: clock_acc(),
    }
    .to_account_metas(None);
    let data = BumpEpochClusterArgs { cluster_offset }.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn activate_arx_ix(signer: &Pubkey, node_offset: u32) -> Instruction {
    let arx_node_acc = arx_acc(node_offset);
    let accounts = ActivateArxAccs {
        signer: signer.key(),
        arx_node_acc,
    }
    .to_account_metas(None);
    let data = ActivateArxArgs { node_offset }.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn deactivate_arx_ix(
    signer: &Pubkey,
    node_offset: u32,
    cluster_offset: Option<u32>,
) -> Instruction {
    let cluster_acc = cluster_offset.map(cluster_acc);

    let arx_node_acc = arx_acc(node_offset);
    let accounts = DeactivateArxAccs {
        signer: signer.key(),
        arx_node_acc,
        clock: clock_acc(),
        cluster_acc,
    }
    .to_account_metas(None);
    let data = DeactivateArxArgs { node_offset }.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn activate_cluster_ix(signer: &Pubkey, cluster_offset: u32) -> Instruction {
    let accounts = ActivateClusterAccs {
        authority: signer.key(),
        cluster_acc: cluster_acc(cluster_offset),
        clock: clock_acc(),
    }
    .to_account_metas(None);
    let data = ActivateClusterArgs {
        cluster_id: cluster_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn deactivate_cluster_ix(
    signer: &Pubkey,
    cluster_offset: u32,
    deactivation_epoch: Epoch,
) -> Instruction {
    let accounts = DeactivateClusterAccs {
        authority: signer.key(),
        cluster_acc: cluster_acc(cluster_offset),
        clock: clock_acc(),
    }
    .to_account_metas(None);
    let data = DeactivateClusterArgs {
        cluster_id: cluster_offset,
        deactivation_epoch,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn set_mxe_keys_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    node_offset: u32,
    cluster_offset: u32,
    mxe_x25519_pubkey: [u8; 32],
    mxe_ed25519_verifying_key: [u8; 32],
    mxe_elgamal_pubkey: [u8; 32],
    mxe_pubkey_validity_proof: [u8; 64],
) -> Instruction {
    let accounts = SetMxeKeysAccs {
        signer: signer.key(),
        node: arx_acc(node_offset),
        mxe: mxe_acc(mxe_program),
        cluster_acc: cluster_acc(cluster_offset),
    }
    .to_account_metas(None);
    let data = SetMxeKeysArgs {
        node_offset,
        _mxe_program: *mxe_program,
        mxe_x25519_pubkey,
        mxe_ed25519_verifying_key,
        mxe_elgamal_pubkey,
        mxe_pubkey_validity_proof,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn finalize_mxe_keys_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    cluster_offset: u32,
) -> Instruction {
    let accounts = FinalizeMxeKeysAccs {
        signer: signer.key(),
        mxe: mxe_acc(mxe_program),
        cluster: cluster_acc(cluster_offset),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = FinalizeMxeKeysArgs {
        _mxe_program: *mxe_program,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn set_mxe_recovery_keys_init_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    node_offset: u32,
    cluster_offset: u32,
    nonce: u128,
    encrypted_mxe_keys: [[u8; 32]; MXE_KEYS_ENC_COUNT],
    key_material_hash: [u8; 32],
    bls_sig: [u8; 64],
) -> Instruction {
    let accounts = SetMxeRecoveryKeysInitAccs {
        signer: signer.key(),
        node: arx_acc(node_offset),
        mxe: mxe_acc(mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(mxe_program),
        cluster_acc: cluster_acc(cluster_offset),
    }
    .to_account_metas(None);
    let data = SetMxeRecoveryKeysInitArgs {
        node_offset,
        _mxe_program: *mxe_program,
        nonce: nonce.to_le_bytes(),
        encrypted_mxe_keys,
        key_material_hash,
        bls_sig,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn extend_recovery_keyshares_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    node_offset: u32,
    cluster_offset: u32,
    keyshares_offset: u32,
    keyshares: Vec<[[u8; 32]; 5]>,
) -> Instruction {
    let accounts = ExtendRecoveryKeysharesAccs {
        signer: signer.key(),
        node: arx_acc(node_offset),
        mxe: mxe_acc(mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(mxe_program),
        cluster_acc: cluster_acc(cluster_offset),
    }
    .to_account_metas(None);
    let data = ExtendRecoveryKeysharesArgs {
        node_offset,
        _mxe_program: *mxe_program,
        keyshares_offset,
        keyshares,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn finalize_key_recovery_shares_upload_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    node_offset: u32,
    cluster_offset: u32,
    computation_offset: u64,
) -> Instruction {
    let accounts = FinalizeKeyRecoverySharesUploadAccs {
        signer: signer.key(),
        node: arx_acc(node_offset),
        mxe: mxe_acc(mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(mxe_program),
        cluster_acc: cluster_acc(cluster_offset),
        comp: computation_acc(cluster_offset, computation_offset),
    }
    .to_account_metas(None);
    let data = FinalizeKeyRecoverySharesUploadArgs {
        node_offset,
        _mxe_program: *mxe_program,
        computation_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

// =============================================================================
// KEY RECOVERY EXECUTION INSTRUCTION BUILDERS
// =============================================================================

/// Sets an MXE to the Recovery state, enabling the recovery process to begin.
pub fn recover_mxe_ix(authority: &Pubkey, mxe_program: &Pubkey) -> Instruction {
    let accounts = RecoverMxeAccs {
        authority: authority.key(),
        mxe: mxe_acc(mxe_program),
        mxe_program: *mxe_program,
    }
    .to_account_metas(None);
    let data = RecoverMxeArgs {
        mxe_program: *mxe_program,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Part 1 of key recovery execution initialization.
/// Creates the MxeRecoveryAccount with partial size due to Solana's 10KB limit.
/// Authority is used to verify ownership, payer pays for the account.
pub fn init_key_recovery_execution_part1_ix(
    authority: &Pubkey,
    payer: &Pubkey,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
) -> Instruction {
    let accounts = InitKeyRecoveryExecutionPart1Accs {
        authority: authority.key(),
        payer: payer.key(),
        original_mxe: mxe_acc(original_mxe_program),
        mxe_recovery_account: mxe_recovery_acc(backup_mxe_program, original_mxe_program),
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitKeyRecoveryExecutionPart1Args {
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Part 2 of key recovery execution initialization.
/// Finishes allocating MxeRecoveryAccount and creates the computation definition.
/// Payer is used for both authority and payer accounts.
pub fn init_key_recovery_execution_part2_ix(
    payer: &Pubkey,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
) -> Instruction {
    // MxeKeyRecoveryFinalize uses reserved offset 3
    const MXE_KEY_RECOVERY_FINALIZE_OFFSET: u32 = 3;
    let accounts = InitKeyRecoveryExecutionPart2Accs {
        authority: payer.key(),
        payer: payer.key(),
        original_mxe: mxe_acc(original_mxe_program),
        backup_mxe: mxe_acc(backup_mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(original_mxe_program),
        mxe_recovery_account: mxe_recovery_acc(backup_mxe_program, original_mxe_program),
        key_recovery_finalize_comp_def: computation_definition_acc(
            backup_mxe_program,
            MXE_KEY_RECOVERY_FINALIZE_OFFSET,
        ),
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = InitKeyRecoveryExecutionPart2Args {
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Submits a recovery share from a recovery peer.
/// Each share contains RESCUE_KEY_COUNT (5) field elements of 32 bytes each.
#[allow(clippy::type_complexity)]
pub fn submit_key_recovery_share_ix(
    signer: &Pubkey,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
    peer_offset: u32,
    peer_index: u32,
    share: [[u8; 32]; 5],
) -> Instruction {
    let accounts = SubmitKeyRecoveryShareAccs {
        signer: signer.key(),
        recovery_peer_account: recovery_peer_acc(peer_offset),
        original_mxe: mxe_acc(original_mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(original_mxe_program),
        mxe_recovery_account: mxe_recovery_acc(backup_mxe_program, original_mxe_program),
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
    }
    .to_account_metas(None);
    let data = SubmitKeyRecoveryShareArgs {
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        peer_offset,
        peer_index,
        share,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Finalizes the key recovery execution after threshold is met and queues the computation.
#[allow(clippy::too_many_arguments)]
pub fn finalize_key_recovery_execution_ix(
    authority: &Pubkey,
    payer: &Pubkey,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
    cluster_offset: u32,
    key_recovery_finalize_offset: u64,
) -> Instruction {
    // MxeKeyRecoveryFinalize uses reserved offset 3
    const MXE_KEY_RECOVERY_FINALIZE_OFFSET: u32 = 3;
    let accounts = FinalizeKeyRecoveryExecutionAccs {
        authority: authority.key(),
        payer: payer.key(),
        original_mxe: mxe_acc(original_mxe_program),
        backup_mxe: mxe_acc(backup_mxe_program),
        recovery_cluster_acc: recovery_cluster_acc(original_mxe_program),
        mxe_recovery_account: mxe_recovery_acc(backup_mxe_program, original_mxe_program),
        key_recovery_finalize_comp_def: computation_definition_acc(
            backup_mxe_program,
            MXE_KEY_RECOVERY_FINALIZE_OFFSET,
        ),
        key_recovery_finalize_computation: computation_acc(
            cluster_offset,
            key_recovery_finalize_offset,
        ),
        executing_pool: execpool_acc(cluster_offset),
        mempool: mempool_acc(cluster_offset),
        pool_account: fee_pool_acc(),
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = FinalizeKeyRecoveryExecutionArgs {
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        cluster_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Closes all recovery-related accounts (replaces cancel_key_recovery_execution).
pub fn close_key_recovery_ix(
    authority: &Pubkey,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
) -> Instruction {
    // MxeKeyRecoveryFinal uses reserved offset 3
    const MXE_KEY_RECOVERY_FINAL_OFFSET: u32 = 3;
    let accounts = CloseKeyRecoveryAccs {
        authority: authority.key(),
        original_mxe: mxe_acc(original_mxe_program),
        backup_mxe: mxe_acc(backup_mxe_program),
        mxe_recovery_account: mxe_recovery_acc(backup_mxe_program, original_mxe_program),
        key_recovery_finalize_comp_def: computation_definition_acc(
            backup_mxe_program,
            MXE_KEY_RECOVERY_FINAL_OFFSET,
        ),
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
    }
    .to_account_metas(None);
    let data = CloseKeyRecoveryArgs {
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Re-queues the key_recovery_finalize computation after a failed execution.
#[allow(clippy::too_many_arguments)]
pub fn requeue_key_recovery_finalize_ix(
    authority: &Pubkey,
    payer: &Pubkey,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
    cluster_offset: u32,
    new_key_recovery_finalize_offset: u64,
    previous_key_recovery_finalize_offset: u64,
) -> Instruction {
    // MxeKeyRecoveryFinal uses reserved offset 3
    const MXE_KEY_RECOVERY_FINALIZE_OFFSET: u32 = 3;
    let accounts = RequeueKeyRecoveryFinalizeAccs {
        authority: authority.key(),
        payer: payer.key(),
        original_mxe: mxe_acc(original_mxe_program),
        backup_mxe: mxe_acc(backup_mxe_program),
        mxe_recovery_account: mxe_recovery_acc(backup_mxe_program, original_mxe_program),
        key_recovery_finalize_comp_def: computation_definition_acc(
            backup_mxe_program,
            MXE_KEY_RECOVERY_FINALIZE_OFFSET,
        ),
        previous_computation: computation_acc(
            cluster_offset,
            previous_key_recovery_finalize_offset,
        ),
        new_computation: computation_acc(cluster_offset, new_key_recovery_finalize_offset),
        executing_pool: execpool_acc(cluster_offset),
        mempool: mempool_acc(cluster_offset),
        pool_account: fee_pool_acc(),
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = RequeueKeyRecoveryFinalizeArgs {
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        cluster_offset,
        new_key_recovery_finalize_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

/// Callback for the key_recovery_finalize computation.
#[allow(clippy::too_many_arguments)]
pub fn finalize_key_recovery_callback_ix(
    signer: &Pubkey,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
    cluster_offset: u32,
    comp_offset: u64,
    node_offset: u32,
    execution_status: ExecutionStatus,
    callback_transaction_index: u8,
    bls_sig: Option<[u8; 64]>,
) -> Instruction {
    // MxeKeyRecoveryFinal uses reserved offset 3
    const MXE_KEY_RECOVERY_FINAL_OFFSET: u32 = 3;
    let accounts = FinalizeKeyRecoveryCallbackAccs {
        signer: signer.key(),
        original_mxe: mxe_acc(original_mxe_program),
        backup_mxe: mxe_acc(backup_mxe_program),
        mxe_recovery_account: mxe_recovery_acc(backup_mxe_program, original_mxe_program),
        computation: computation_acc(cluster_offset, comp_offset),
        comp_def: computation_definition_acc(backup_mxe_program, MXE_KEY_RECOVERY_FINAL_OFFSET),
        node: arx_acc(node_offset),
        cluster_acc: cluster_acc(cluster_offset),
        executing_pool: execpool_acc(cluster_offset),
        mempool: mempool_acc(cluster_offset),
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
    }
    .to_account_metas(None);
    let data = FinalizeKeyRecoveryCallbackArgs {
        original_mxe_program: *original_mxe_program,
        backup_mxe_program: *backup_mxe_program,
        cluster_offset,
        comp_offset,
        node_offset,
        execution_status,
        callback_transaction_index,
        bls_sig,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn set_cluster_ix(signer: &Pubkey, mxe_program: &Pubkey, cluster_offset: u32) -> Instruction {
    let accounts = SetClusterAccs {
        signer: signer.key(),
        mxe: mxe_acc(mxe_program),
        cluster: cluster_acc(cluster_offset),
        clock: clock_acc(),
        mxe_program: *mxe_program,
    }
    .to_account_metas(None);
    let data = SetClusterArgs { cluster_offset }.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn callback_computation_ix(
    signer: &Pubkey,
    mxe_program: &Pubkey,
    computation_offset: u64,
    comp_def_offset: u32,
    node_offset: u32,
    cluster_offset: u32,
    execution_status: ExecutionStatus,
    callback_transaction_index: u8,
) -> Instruction {
    let accounts = CallbackComputationAccs {
        signer: signer.key(),
        node: arx_acc(node_offset),
        comp: computation_acc(cluster_offset, computation_offset),
        mxe: mxe_acc(mxe_program),
        cluster_acc: cluster_acc(cluster_offset),
        mempool: mempool_acc(cluster_offset),
        executing_pool: execpool_acc(cluster_offset),
        comp_def_acc: computation_definition_acc(mxe_program, comp_def_offset),
        instructions_sysvar: INSTRUCTIONS_SYSVAR_ID,
    }
    .to_account_metas(None);
    let data = CallbackComputationArgs {
        node_offset,
        comp_def_offset,
        comp_offset: computation_offset,
        mxe_program: *mxe_program,
        execution_status,
        callback_transaction_index,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

// Callback computation instruction on the user program.
pub fn callback_computation_ix_user(
    mxe_prog_id: &Pubkey,
    callback_accs: Vec<CallbackAccount>,
    comp_def_offset: u32,
    callback_discriminator: &[u8],
    output_bytes: Vec<u8>,
) -> Instruction {
    let mut bytes = Vec::with_capacity(callback_discriminator.len() + output_bytes.len());
    bytes.extend_from_slice(callback_discriminator);
    bytes.extend_from_slice(&output_bytes);

    let accounts = vec![
        // `arcium_program`
        AccountMeta {
            pubkey: ARCIUM_PROG_ID,
            is_signer: false,
            is_writable: false,
        },
        // `computation_definition_account`
        AccountMeta {
            pubkey: computation_definition_acc(mxe_prog_id, comp_def_offset),
            is_signer: false,
            is_writable: false,
        },
        // `instructions_sysvar` Needed for acc introspection
        AccountMeta {
            pubkey: sysvar::instructions::id(),
            is_signer: false,
            is_writable: false,
        },
    ]
    .into_iter()
    .chain(callback_accs.iter().map(|c| AccountMeta {
        pubkey: c.pubkey,
        is_writable: c.is_writable,
        is_signer: false,
    }))
    .collect::<Vec<AccountMeta>>()
    .to_account_metas(None);

    Instruction {
        program_id: mxe_prog_id.to_owned(),
        accounts,
        data: bytes,
    }
}

pub fn claim_failure_init_ix(
    signer: &Pubkey,
    mxe_program_id: &Pubkey,
    cluster_offset: u32,
    comp_def_offset: u32,
    computation_offset: u64,
    node_offset: u32,
    output_len_bytes: u32,
) -> Instruction {
    let accounts = ClaimFailureInitAccs {
        signer: signer.key(),
        node_acc: arx_acc(node_offset),
        mxe: mxe_acc(mxe_program_id),
        cluster_acc: cluster_acc(cluster_offset),
        comp_acc: computation_acc(cluster_offset, computation_offset),
        comp_def_acc: computation_definition_acc(mxe_program_id, comp_def_offset),
        failure_acc: failure_claim_acc(mxe_program_id, computation_offset),
        system_program: SYSTEM_PROGRAM_ID,
    }
    .to_account_metas(None);
    let data = ClaimFailureInitArgs {
        comp_offset: computation_offset,
        node_offset,
        mxe_program: mxe_program_id.to_owned(),
        output_len_bytes,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn claim_failure_append_ix(
    signer: &Pubkey,
    mxe_program_id: &Pubkey,
    computation_offset: u64,
    chunk: Vec<u8>,
    failure_claim_offset: u32,
) -> Instruction {
    let accounts = ClaimFailureAppendAccs {
        signer: signer.key(),
        failure_acc: failure_claim_acc(mxe_program_id, computation_offset),
    }
    .to_account_metas(None);
    let data = ClaimFailureAppendArgs {
        comp_offset: computation_offset,
        mxe_program: mxe_program_id.to_owned(),
        chunk,
        failure_claim_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn claim_failure_finalize_ix(
    signer: &Pubkey,
    mxe_program_id: &Pubkey,
    cluster_offset: u32,
    computation_offset: u64,
    node_offset: u32,
) -> Instruction {
    let accounts = ClaimFailureFinalizeAccs {
        signer: signer.key(),
        failure_acc: failure_claim_acc(mxe_program_id, computation_offset),
        executing_pool: execpool_acc(cluster_offset),
        mempool: mempool_acc(cluster_offset),
        comp: computation_acc(cluster_offset, computation_offset),
        cluster_acc: cluster_acc(cluster_offset),
        mxe: mxe_acc(mxe_program_id),
    }
    .to_account_metas(None);
    let data = ClaimFailureFinalizeArgs {
        comp_offset: computation_offset,
        node_offset,
        mxe_program: mxe_program_id.to_owned(),
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn reclaim_failure_rent_idempotent_ix(
    signer: &Pubkey,
    recipient: &Pubkey,
    mxe_program_id: &Pubkey,
    computation_offset: u64,
) -> Instruction {
    let accounts = ReclaimFailureRentIdempotentAccs {
        signer: signer.key(),
        recipient: recipient.key(),
        failure_acc: failure_claim_acc(mxe_program_id, computation_offset),
    }
    .to_account_metas(None);
    let data = ReclaimFailureRentIdempotentArgs {
        comp_offset: computation_offset,
        mxe_program: mxe_program_id.to_owned(),
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn update_current_epoch_idempotent_ix() -> Instruction {
    let accounts = UpdateCurrentEpochIdempotentAccs { clock: clock_acc() }.to_account_metas(None);
    let data = UpdateCurrentEpochIdempotentArgs {}.data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn set_arx_node_config_ix(
    signer: &Pubkey,
    node_offset: u32,
    config: ArxNodeConfig,
) -> Instruction {
    let accounts = SetArxNodeConfigAccs {
        signer: signer.key(),
        arx_node_acc: arx_acc(node_offset),
    }
    .to_account_metas(None);
    let data = SetArxNodeConfigArgs {
        node_offset,
        config,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn set_arx_node_metadata_ix(
    signer: &Pubkey,
    node_offset: u32,
    metadata: NodeMetadata,
) -> Instruction {
    let accounts = SetArxNodeMetadataAccs {
        signer: signer.key(),
        arx_node_acc: arx_acc(node_offset),
    }
    .to_account_metas(None);
    let data = SetArxNodeMetadataArgs {
        node_offset,
        meta: metadata,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}

pub fn submit_aggregated_bls_pubkey_ix(
    node_authority: &Pubkey,
    cluster_offset: u32,
    node_offset: u32,
    aggregated_bls_pubkey: BN254G2BLSPublicKey,
) -> Instruction {
    let accounts = SubmitAggregatedBlsPubkeyAccs {
        cluster_acc: cluster_acc(cluster_offset),
        arx_node_acc: arx_acc(node_offset),
        node_authority: node_authority.key(),
    }
    .to_account_metas(None);

    let data = SubmitAggregatedBlsPubkeyArgs {
        cluster_id: cluster_offset,
        aggregated_bls_pubkey,
        node_bump: node_offset,
    }
    .data();

    Instruction {
        program_id: ARCIUM_PROG_ID,
        accounts,
        data,
    }
}
#[cfg(feature = "staking")]
pub mod staking {
    use super::*;
    use crate::{
        idl::arcium_staking::{
            client::{
                accounts::{
                    ActivatePrimaryStake as ActivatePrimaryStakeAccs,
                    CloseDelegatedStake as CloseDelegatedStakeAccs,
                    DeactivatePrimaryStake as DeactivatePrimaryStakeAccs,
                    DelegateStake as DelegateStakeAccs,
                    FinalizeEpochRewards as FinalizeEpochRewardsAccs,
                    InitDelegatedStakeAcc as InitDelegatedStakeAccs,
                    InitPrimaryStake as InitPrimaryStakeAccs,
                    InitStakeMasterAcc as InitDelegatedStakeMasterAccs,
                    MergeDelegatedStakeAccount as MergeDelegatedStakeAccountAccs,
                    SplitDelegatedStakeAccount as SplitDelegatedStakeAccountAccs,
                    UndelegateStake as UndelegateStakeAccs,
                },
                args::{
                    ActivatePrimaryStake as ActivatePrimaryStakeArgs,
                    CloseDelegatedStake as CloseDelegatedStakeArgs,
                    DeactivatePrimaryStake as DeactivatePrimaryStakeArgs,
                    DelegateStake as DelegateStakeArgs,
                    FinalizeEpochRewards as FinalizeEpochRewardsArgs,
                    InitDelegatedStakeAcc as InitDelegatedStakeArgs,
                    InitPrimaryStake as InitPrimaryStakeArgs,
                    InitStakeMasterAcc as InitDelegatedStakeMasterArgs,
                    MergeDelegatedStakeAccount as MergeDelegatedStakeAccountArgs,
                    SplitDelegatedStakeAccount as SplitDelegatedStakeAccountArgs,
                    UndelegateStake as UndelegateStakeArgs,
                },
            },
            types::{Epoch, RewardClaim},
            ID as ARCIUM_STAKING_PROG_ID,
        },
        pda::{
            staking::{
                delegated_stake_acc,
                primary_stake_acc,
                stake_master_acc,
                stake_queue_acc,
                staking_pool_acc,
            },
            ARCIUM_TOKEN_MINT,
        },
    };
    use anchor_client::solana_sdk::program_pack::Pack;
    use anchor_lang::solana_program::system_instruction::create_account;
    use anchor_spl::{
        associated_token::{
            get_associated_token_address,
            spl_associated_token_account::instruction::create_associated_token_account_idempotent,
        },
        token::{
            spl_token::{
                instruction::{initialize_mint, mint_to},
                state::Mint,
            },
            ID as TOKEN_PROGRAM_ID,
        },
    };

    pub fn init_arcium_token_mint_ixs(
        minimum_balance_for_rent_exemption: u64,
        signer: &Pubkey,
    ) -> [Instruction; 2] {
        let create_acc_ix = create_account(
            signer,
            &ARCIUM_TOKEN_MINT,
            minimum_balance_for_rent_exemption,
            Mint::LEN as u64,
            &TOKEN_PROGRAM_ID,
        );
        let init_mint_ix = initialize_mint(
            &TOKEN_PROGRAM_ID,
            &ARCIUM_TOKEN_MINT,
            signer,
            None,
            ARCIUM_TOKEN_DECIMALS,
        )
        .expect("Failed to create initialize mint instruction");

        [create_acc_ix, init_mint_ix]
    }

    pub fn airdrop_arcium_token_ixs(
        signer: &Pubkey,
        recipient: &Pubkey,
        amount: u64,
    ) -> [Instruction; 2] {
        let rec_ata = get_associated_token_address(recipient, &ARCIUM_TOKEN_MINT);
        let create_ata_ix = create_associated_token_account_idempotent(
            signer,
            recipient,
            &ARCIUM_TOKEN_MINT,
            &TOKEN_PROGRAM_ID,
        );
        let mint_to_ix = mint_to(
            &TOKEN_PROGRAM_ID,
            &ARCIUM_TOKEN_MINT,
            &rec_ata,
            signer,
            &[signer],
            amount,
        )
        .unwrap_or_else(|err| panic!("Failed to create mint ix: {}", err));

        [create_ata_ix, mint_to_ix]
    }

    pub fn init_primary_stake_acc_ix(
        signer: &Pubkey,
        amount: u64,
        fee_basis_points: u16,
    ) -> Instruction {
        let primary_stake_acc = primary_stake_acc(signer);
        let pool_acc = staking_pool_acc();
        let accounts = InitPrimaryStakeAccs {
            from: signer.to_owned(),
            from_ta: get_associated_token_address(signer, &ARCIUM_TOKEN_MINT),
            primary_stake_account: primary_stake_acc.key(),
            stake_queue: stake_queue_acc(&primary_stake_acc),
            mint: ARCIUM_TOKEN_MINT,
            pool_account: pool_acc,
            pool_ata: get_associated_token_address(&pool_acc, &ARCIUM_TOKEN_MINT),
            clock: clock_acc(),
            system_program: SYSTEM_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = InitPrimaryStakeArgs {
            amount,
            fee_basis_points,
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn activate_primary_stake_acc_ix(signer: &Pubkey, lockup_epochs: u64) -> Instruction {
        let accounts = ActivatePrimaryStakeAccs {
            signer: signer.to_owned(),
            primary_stake_account: primary_stake_acc(signer),
            clock: clock_acc(),
        }
        .to_account_metas(None);
        let data = ActivatePrimaryStakeArgs {
            lockup_epochs: Epoch(lockup_epochs),
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn deactivate_primary_stake_acc_ix(
        signer: &Pubkey,
        arx_node_offset: Option<u32>,
    ) -> Instruction {
        let accounts = DeactivatePrimaryStakeAccs {
            signer: signer.to_owned(),
            primary_stake_account: primary_stake_acc(signer),
            clock: clock_acc(),
        }
        .to_account_metas(None);
        let data = DeactivatePrimaryStakeArgs {
            node_offset: arx_node_offset,
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn finalize_epoch_rewards_ix(
        signer: &Pubkey,
        primary_stake_owner: &Pubkey,
        node_offset: u32,
        stake_reward: RewardClaim,
    ) -> Instruction {
        let pool_account = staking_pool_acc();
        let mint = ARCIUM_TOKEN_MINT;
        let primary_stake_acc = primary_stake_acc(primary_stake_owner);

        let accounts = FinalizeEpochRewardsAccs {
            signer: signer.key(),
            primary_stake_owner: primary_stake_owner.key(),
            primary_stake_owner_ata: get_associated_token_address(primary_stake_owner, &mint),
            stake_queue: stake_queue_acc(&primary_stake_acc),
            primary_stake_account: primary_stake_acc,
            pool_account,
            pool_ata: get_associated_token_address(&pool_account, &mint),
            clock: clock_acc(),
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = FinalizeEpochRewardsArgs {
            node_offset,
            stake_reward,
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn init_delegated_stake_master_acc_ix(signer: &Pubkey, owner: &Pubkey) -> Instruction {
        let accounts = InitDelegatedStakeMasterAccs {
            signer: signer.to_owned(),
            master_stake_account: stake_master_acc(owner),
            owner: owner.to_owned(),
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = InitDelegatedStakeMasterArgs {}.data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn init_delegated_stake_acc_ix(
        signer: &Pubkey,
        stake_offset: u128,
        amount: u64,
    ) -> Instruction {
        let accounts = InitDelegatedStakeAccs {
            from: signer.to_owned(),
            from_ata: get_associated_token_address(signer, &ARCIUM_TOKEN_MINT),
            master_stake_account: stake_master_acc(signer),
            user_stake_account: delegated_stake_acc(stake_offset),
            mint: ARCIUM_TOKEN_MINT,
            pool_account: staking_pool_acc(),
            pool_ata: get_associated_token_address(&staking_pool_acc(), &ARCIUM_TOKEN_MINT),
            system_program: SYSTEM_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = InitDelegatedStakeArgs {
            stake_offset,
            amount,
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn delegate_stake_ix(
        signer: &Pubkey,
        stake_offset: u128,
        primary_stake_owner: &Pubkey,
        lockup_epochs: u64,
    ) -> Instruction {
        let accounts = DelegateStakeAccs {
            signer: signer.to_owned(),
            primary_acc_owner: primary_stake_owner.to_owned(),
            user_stake_account: delegated_stake_acc(stake_offset),
            primary: primary_stake_acc(primary_stake_owner),
            stake_queue: stake_queue_acc(&primary_stake_acc(primary_stake_owner)),
            clock: clock_acc(),
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = DelegateStakeArgs {
            stake_offset,
            lockup_epochs: Epoch(lockup_epochs),
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn undelegate_stake_ix(
        signer: &Pubkey,
        stake_offset: u128,
        primary_stake_owner: &Pubkey,
    ) -> Instruction {
        let accounts = UndelegateStakeAccs {
            signer: signer.to_owned(),
            user_stake_account: delegated_stake_acc(stake_offset),
            primary: primary_stake_acc(primary_stake_owner),
            clock: clock_acc(),
        }
        .to_account_metas(None);
        let data = UndelegateStakeArgs { stake_offset }.data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    /// Split a delegated stake account into two.
    /// * `primary_stake_owner_target` - The owner of the primary stake account we're delegating to.
    pub fn split_delegated_stake_account_ix(
        primary_stake_owner_target: &Pubkey,
        delegation_authority: &Pubkey,
        withdrawal_authority: &Pubkey,
        stake_offset: u128,
        stake_offset_new: u128,
        new_acc_balance: u64,
    ) -> Instruction {
        let primary_stake_acc = primary_stake_acc(primary_stake_owner_target);

        let accounts = SplitDelegatedStakeAccountAccs {
            delegation_authority: delegation_authority.to_owned(),
            withdrawal_authority: withdrawal_authority.to_owned(),
            delegation_master: stake_master_acc(delegation_authority),
            withdrawal_master: stake_master_acc(withdrawal_authority),
            old_stake_account: delegated_stake_acc(stake_offset),
            new_stake_account: delegated_stake_acc(stake_offset_new),
            stake_queue: stake_queue_acc(&primary_stake_acc),
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = SplitDelegatedStakeAccountArgs {
            new_acc_balance,
            _stake_offset: stake_offset,
            _stake_offset_new: stake_offset_new,
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    /// Split a delegated stake account into two.
    /// * `primary_stake_owner_target` - The owner of the primary stake account we're delegating to.
    pub fn merge_delegated_stake_account_ix(
        primary_stake_owner_target: &Pubkey,
        delegation_authority: &Pubkey,
        withdrawal_authority: &Pubkey,
        stake_offset_keep: u128,
        stake_offset_close: u128,
    ) -> Instruction {
        let primary_stake_acc = primary_stake_acc(primary_stake_owner_target);
        let accounts = MergeDelegatedStakeAccountAccs {
            delegation_authority: delegation_authority.to_owned(),
            withdrawal_authority: withdrawal_authority.to_owned(),
            delegation_master: stake_master_acc(delegation_authority),
            withdrawal_master: stake_master_acc(withdrawal_authority),
            stake_acc_to_keep: delegated_stake_acc(stake_offset_keep),
            stake_acc_to_close: delegated_stake_acc(stake_offset_close),
            stake_queue: stake_queue_acc(&primary_stake_acc),
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = MergeDelegatedStakeAccountArgs {
            _stake_offset_keep: stake_offset_keep,
            _stake_offset_close: stake_offset_close,
        }
        .data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }

    pub fn close_delegated_stake_ix(
        signer: &Pubkey,
        delegation_owner: &Pubkey,
        stake_offset: u128,
    ) -> Instruction {
        let accounts = CloseDelegatedStakeAccs {
            signer: signer.to_owned(),
            signer_ata: get_associated_token_address(signer, &ARCIUM_TOKEN_MINT),
            withdrawal_master: stake_master_acc(signer),
            delegation_master: stake_master_acc(delegation_owner),
            delegation_owner: delegation_owner.to_owned(),
            user_stake_account: delegated_stake_acc(stake_offset),
            mint: ARCIUM_TOKEN_MINT,
            pool_account: staking_pool_acc(),
            pool_ata: get_associated_token_address(&staking_pool_acc(), &ARCIUM_TOKEN_MINT),
            clock: clock_acc(),
            system_program: SYSTEM_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
        }
        .to_account_metas(None);
        let data = CloseDelegatedStakeArgs { stake_offset }.data();

        Instruction {
            program_id: ARCIUM_STAKING_PROG_ID,
            accounts,
            data,
        }
    }
}
