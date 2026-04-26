//! Transaction builders for the Arcium programs.
use crate::{
    idl::arcium::{
        accounts::MXEAccount,
        types::{
            ArgumentList,
            BN254G2BLSPublicKey,
            CallbackInstruction,
            CircuitSource,
            MempoolSize,
            NodeMetadata,
            Output,
            Parameter,
        },
        ID as ARCIUM_PROG_ID,
    },
    instruction::{
        activate_cluster_ix,
        claim_failure_append_ix,
        claim_failure_finalize_ix,
        claim_failure_init_ix,
        claim_node_fees_ix,
        finalize_mxe_keys_ix,
        increase_mempool_size_ix,
        init_arx_node_acc_ix,
        init_cluster_ix,
        init_computation_definition_ix,
        init_mxe_part1_ix,
        init_mxe_part2_ix,
        init_network_program_ix,
        init_node_operator_acc_ix,
        init_recovery_peer_acc_ix,
        join_cluster_ix,
        propose_fee_ix,
        propose_join_cluster_ix,
        queue_computation_ix,
        requeue_mxe_keygen_ix,
        set_arx_node_metadata_ix,
        set_cluster_ix,
        submit_aggregated_bls_pubkey_ix,
        submit_key_recovery_share_ix,
        vote_fee_ix,
    },
    pda::mxe_acc,
    utils::MAX_RECOVERY_PEERS,
};
use anchor_client::{
    solana_sdk::{
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Client,
    Cluster as SolanaCluster,
    Program,
    RequestBuilder,
    ThreadSafeSigner,
};
use anchor_lang::prelude::*;
use solana_rpc_client::nonblocking::rpc_client::RpcClient as AsyncRpcClient;
use solana_rpc_client_api::config::RpcSendTransactionConfig;
use std::{ops::Deref, sync::Arc, vec};

// Maximum callback data size that fits in a single transaction (init + append + finalize).
// Computed as: 1232 (max serialized tx size) - 645 (overhead from signatures, accounts, ix data)
// See test_build_claim_failure_txs_single_tx_at_max_size for validation.
pub(crate) const MAX_SINGLE_TX_FAILURE_CLAIM_DATA_SIZE: usize = 587;

pub const DEFAULT_PRIM_STAKE_AMOUNT: u64 = 1000;
pub const DEFAULT_FEE_BASIS_POINTS: u16 = 0;
pub const DEFAULT_LOCKUP_EPOCHS: u64 = 10;
pub const DEFAULT_URL: &str = "https://arcium.com";
pub const DEFAULT_LOCATION: u8 = 0;
pub const DEFAULT_CU_CLAIM: u64 = 1000;
pub const DEFAULT_MAX_CLUSTERS: u32 = 1;
pub const DEFAULT_MAX_SIZE: u32 = 1;
pub const DEFAULT_CLUSTER_ENCRYPTION_PUBKEY: [u8; 32] = [0; 32];
pub const DEFAULT_CU_PRICE: u64 = 1;
pub const DEFAULT_COMPILED_CIRCUIT: [u8; 24] = [0; 24];
pub const DEFAULT_AUTHORITY_PUBKEYS: Option<Vec<Pubkey>> = None;
pub const DEFAULT_PARAMS: Vec<Parameter> = vec![];
pub const DEFAULT_CALLBACK_DATA_OBJS: Vec<Pubkey> = vec![];
pub const DEFAULT_OUTPUT_SCALAR_LEN: u8 = 0;
pub const DEFAULT_DUMMY_CALLBACK_DISC: [u8; 8] = [0; 8];
pub const DEFAULT_DUMMY_ENCRYPTION_PUBKEY: [u8; 32] = [0; 32];
pub const DEFAULT_DUMMY_PROGRAM_ID: Pubkey =
    pubkey!("Bzabqe5qowkb54kQ96frT6WY7KNLQiMhj4pGGmrZMHRa");

pub async fn init_network_program(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    config: RpcSendTransactionConfig,
) -> Signature {
    let current_time = current_unix_timestamp(arcium_program.internal_rpc()).await;
    let ix = init_network_program_ix(current_time, &signer.pubkey());
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn submit_key_recovery_share(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    original_mxe_program: &Pubkey,
    backup_mxe_program: &Pubkey,
    peer_offset: u32,
    peer_index: u32,
    share: [[u8; 32]; 5],
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = submit_key_recovery_share_ix(
        &signer.pubkey(),
        original_mxe_program,
        backup_mxe_program,
        peer_offset,
        peer_index,
        share,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

// Location as [ISO 3166-1 alpha-2](https://www.iso.org/iso-3166-country-codes.html) country code

pub async fn init_node_operator_acc(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    url: String,
    location: u8,

    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = init_node_operator_acc_ix(&signer.pubkey(), url, location);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn init_arx_node_acc(
    arcium_program: &Program<Arc<Keypair>>,
    operator_signer: Arc<Keypair>,
    node_signer: Arc<Keypair>,
    node_offset: u32,
    callback_authority: Pubkey,
    cu_claim: u64,
    metadata: NodeMetadata,
    bls_pubkey: BN254G2BLSPublicKey,
    x25519_pubkey: [u8; 32],
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = init_arx_node_acc_ix(
        &operator_signer.pubkey(),
        &node_signer.pubkey(),
        node_offset,
        &callback_authority,
        cu_claim,
        bls_pubkey,
        metadata,
        x25519_pubkey,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![operator_signer, node_signer], tx, config).await
}

pub async fn set_arx_node_metadata(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    node_offset: u32,
    metadata: NodeMetadata,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = set_arx_node_metadata_ix(&signer.pubkey(), node_offset, metadata);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn init_cluster(
    arcium_program: &Program<Arc<Keypair>>,
    payer: Arc<Keypair>,
    cluster_authority: Pubkey,
    cluster_offset: u32,
    cluster_size: u16,
    cu_price: u64,
    mempool_size: MempoolSize,
    td_info: Option<NodeMetadata>,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = init_cluster_ix(
        &payer.pubkey(),
        cluster_authority,
        cluster_offset,
        cluster_size,
        cu_price,
        mempool_size,
        td_info,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![payer], tx, config).await
}

pub async fn init_recovery_peer_acc(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    peer_offset: u32,
    x25519_pubkey: [u8; 32],
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = init_recovery_peer_acc_ix(&signer.pubkey(), peer_offset, x25519_pubkey);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn init_mxe(
    arcium_program: &Program<Arc<Keypair>>,
    payer: Arc<Keypair>,
    mxe_program: &Pubkey,
    mxe_authority: Option<Pubkey>,
    keygen_comp_offset: u64,
    key_recovery_init_comp_offset: u64,
    cluster_offset: u32,
    recovery_peers: [u32; MAX_RECOVERY_PEERS],
    recent_slot: u64,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix_pt1 = init_mxe_part1_ix(&payer.pubkey(), mxe_program);
    let ix_pt2 = init_mxe_part2_ix(
        &payer.pubkey(),
        mxe_program,
        cluster_offset,
        recovery_peers,
        keygen_comp_offset,
        key_recovery_init_comp_offset,
        mxe_authority,
        recent_slot,
    );
    let tx = arcium_program
        .request()
        .instruction(ix_pt1)
        .instruction(ix_pt2);
    send_tx(vec![payer], tx, config).await
}

pub async fn finalize_mxe_keys(
    arcium_program: &Program<Arc<Keypair>>,
    payer: Arc<Keypair>,
    mxe_program: &Pubkey,
    cluster_offset: u32,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = finalize_mxe_keys_ix(&payer.pubkey(), mxe_program, cluster_offset);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![payer], tx, config).await
}

pub async fn requeue_mxe_keygen(
    arcium_program: &Program<Arc<Keypair>>,
    payer: Arc<Keypair>,
    mxe_program: &Pubkey,
    mxe_acc: MXEAccount,
    cluster_offset: u32,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = requeue_mxe_keygen_ix(
        &payer.pubkey(),
        mxe_program,
        cluster_offset,
        mxe_acc.keygen_offset,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![payer], tx, config).await
}

pub async fn set_cluster(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    mxe_program: &Pubkey,
    cluster_offset: u32,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = set_cluster_ix(&signer.pubkey(), mxe_program, cluster_offset);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn init_computation_definition(
    arcium_program: &Program<Arc<Keypair>>,
    payer: Arc<Keypair>,
    computation_def_offset: u32,
    mxe_program: &Pubkey,
    circuit_len: u32,
    params: Vec<Parameter>,
    outputs: Vec<Output>,
    circuit_source_override: Option<CircuitSource>,
    cu_amount: u64,
    finalization_authority: Option<Pubkey>,
    lut_slot: u64,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = init_computation_definition_ix(
        &payer.pubkey(),
        computation_def_offset,
        mxe_program,
        circuit_len,
        params,
        outputs,
        cu_amount,
        finalization_authority,
        circuit_source_override,
        lut_slot,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![payer], tx, config).await
}

pub async fn increase_mempool_size(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    cluster_offset: u32,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = increase_mempool_size_ix(&signer.pubkey(), cluster_offset);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

pub async fn activate_cluster(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    cluster_offset: u32,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = activate_cluster_ix(&signer.pubkey(), cluster_offset);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![signer], tx, config).await
}

pub async fn propose_join_cluster(
    arcium_program: &Program<Arc<Keypair>>,
    cluster_auth_signer: Arc<Keypair>,
    cluster_offset: u32,
    node_offset: u32,

    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = propose_join_cluster_ix(&cluster_auth_signer.pubkey(), cluster_offset, node_offset);
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![cluster_auth_signer], tx, config).await
}

pub async fn join_cluster(
    arcium_program: &Program<Arc<Keypair>>,
    node_auth_signer: Arc<Keypair>,
    cluster_offset: u32,
    node_offset: u32,
    join: bool,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = join_cluster_ix(
        &node_auth_signer.pubkey(),
        cluster_offset,
        node_offset,
        join,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![node_auth_signer], tx, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn queue_computation(
    arcium_program: &Program<Arc<Keypair>>,
    payer: Arc<Keypair>,
    mxe_program: &Pubkey,
    computation_offset: u64,
    comp_def_offset: u32,
    args: ArgumentList,
    output_delivery_fee: u64,
    cu_price_micro: u64,
    callback_instructions: Vec<CallbackInstruction>,
    callback_transactions_required: u8,
    config: RpcSendTransactionConfig,
) -> Result<Signature> {
    let mxe_raw_data = arcium_program
        .internal_rpc()
        .get_account_data(&mxe_acc(mxe_program))
        .await
        .expect("Failed to fetch MXE");
    let mxe_data = MXEAccount::try_deserialize(&mut mxe_raw_data.as_slice())
        .expect("Failed to deserialize MXE");

    let ix = queue_computation_ix(
        &payer.pubkey(),
        mxe_program,
        computation_offset,
        comp_def_offset,
        args,
        output_delivery_fee,
        cu_price_micro,
        mxe_data,
        callback_instructions,
        callback_transactions_required,
    )?;
    let tx = arcium_program.request().instruction(ix);
    Ok(send_tx(vec![payer], tx, config).await)
}

pub async fn propose_fee(
    arcium_program: &Program<Arc<Keypair>>,
    node_auth_signer: Arc<Keypair>,
    cluster_offset: u32,
    node_offset: u32,
    proposed_fee: u64,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = propose_fee_ix(
        node_auth_signer.pubkey(),
        cluster_offset,
        node_offset,
        proposed_fee,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![node_auth_signer], tx, config).await
}

pub async fn vote_fee(
    arcium_program: &Program<Arc<Keypair>>,
    node_auth_signer: Arc<Keypair>,
    cluster_offset: u32,
    node_offset: u32,
    proposed_fee: u64,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = vote_fee_ix(
        node_auth_signer.pubkey(),
        cluster_offset,
        node_offset,
        proposed_fee,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![node_auth_signer], tx, config).await
}

pub async fn claim_node_fees(
    arcium_program: &Program<Arc<Keypair>>,
    node_authority: Arc<Keypair>,
    cluster_offset: u32,
    node_offset: u32,
    recipient: Pubkey,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = claim_node_fees_ix(
        &node_authority.pubkey(),
        cluster_offset,
        node_offset,
        &recipient,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![node_authority], tx, config).await
}

pub async fn submit_aggregated_bls_pubkey(
    arcium_program: &Program<Arc<Keypair>>,
    node_authority: Arc<Keypair>,
    cluster_offset: u32,
    node_offset: u32,
    aggregated_bls_pubkey: BN254G2BLSPublicKey,
    config: RpcSendTransactionConfig,
) -> Signature {
    let ix = submit_aggregated_bls_pubkey_ix(
        &node_authority.pubkey(),
        cluster_offset,
        node_offset,
        aggregated_bls_pubkey,
    );
    let tx = arcium_program.request().instruction(ix);
    send_tx(vec![node_authority], tx, config).await
}

#[allow(clippy::too_many_arguments)]
pub async fn claim_failure(
    arcium_program: &Program<Arc<Keypair>>,
    signer: Arc<Keypair>,
    mxe_program: &Pubkey,
    cluster_offset: u32,
    comp_def_offset: u32,
    computation_offset: u64,
    node_offset: u32,
    callback_data: Vec<u8>,
    failure_claim_offset: u32,
    config: RpcSendTransactionConfig,
) -> Vec<Signature> {
    let mut txs = build_claim_failure_txs(
        arcium_program,
        &signer.pubkey(),
        mxe_program,
        cluster_offset,
        comp_def_offset,
        computation_offset,
        node_offset,
        callback_data,
        failure_claim_offset,
    )
    .into_iter();
    let txs_len = txs.len();
    let mut signatures = Vec::with_capacity(txs_len);
    if let Some(first) = txs.next() {
        signatures.push(send_tx(vec![signer.clone()], first, config).await);
    }
    // Take all but the last one (the finalize)
    for tx in txs.by_ref().take(txs_len.saturating_sub(2)) {
        signatures.push(send_tx(vec![signer.clone()], tx, config).await);
    }
    if let Some(finalize_tx) = txs.next() {
        signatures.push(send_tx(vec![signer], finalize_tx, config).await);
    }
    signatures
}

#[allow(clippy::too_many_arguments)]
fn build_claim_failure_txs<'a>(
    arcium_program: &'a Program<Arc<Keypair>>,
    signer_pubkey: &Pubkey,
    mxe_program_id: &Pubkey,
    cluster_offset: u32,
    comp_def_offset: u32,
    computation_offset: u64,
    node_offset: u32,
    callback_data: Vec<u8>,
    failure_claim_offset: u32,
) -> Vec<RequestBuilder<'a, Arc<Keypair>, Arc<dyn ThreadSafeSigner>>> {
    if callback_data.len() <= MAX_SINGLE_TX_FAILURE_CLAIM_DATA_SIZE {
        let init_ix = claim_failure_init_ix(
            signer_pubkey,
            mxe_program_id,
            cluster_offset,
            comp_def_offset,
            computation_offset,
            node_offset,
            callback_data.len() as u32,
        );
        let append_ix = claim_failure_append_ix(
            signer_pubkey,
            mxe_program_id,
            computation_offset,
            callback_data,
            failure_claim_offset,
        );
        let finalize_ix = claim_failure_finalize_ix(
            signer_pubkey,
            mxe_program_id,
            cluster_offset,
            computation_offset,
            node_offset,
        );
        let tx = arcium_program
            .request()
            .instruction(init_ix)
            .instruction(append_ix)
            .instruction(finalize_ix);
        return vec![tx];
    }
    todo!()
}

// Builds a program client for the Arcium program
pub fn arcium_program_client(
    cluster: SolanaCluster,
    signer: Arc<Keypair>,
    rpc: AsyncRpcClient,
) -> Program<Arc<Keypair>> {
    let client = Client::new(cluster, signer);
    client
        .program(ARCIUM_PROG_ID, rpc)
        .unwrap_or_else(|err| panic!("Failed to create program: {}", err))
}

async fn send_tx<C: Deref<Target = impl Signer> + Clone>(
    signers: Vec<Arc<Keypair>>,
    ix: RequestBuilder<'_, C, Arc<dyn ThreadSafeSigner>>,
    config: RpcSendTransactionConfig,
) -> Signature {
    let signed = signers.into_iter().fold(ix, |ix, signer| ix.signer(signer));
    signed
        .send_with_spinner_and_config(config)
        .await
        // We do direct unwrap. Wrapping it in panic! would result in not the full error message.
        .unwrap()
}

/// Get the current unix timestamp from the cluster
async fn current_unix_timestamp(client: &AsyncRpcClient) -> u64 {
    let latest_slot = client
        .get_slot()
        .await
        .unwrap_or_else(|err| panic!("Failed to fetch slot: {}", err));
    client
        .get_block_time(latest_slot)
        .await
        .unwrap_or_else(|err| panic!("Failed to fetch block time: {}", err))
        .try_into()
        .unwrap_or_else(|err| panic!("Failed to convert block time to u64: {}", err))
}

#[cfg(feature = "staking")]
pub mod staking {
    use super::*;
    use crate::{
        instruction::staking::{
            activate_primary_stake_acc_ix,
            airdrop_arcium_token_ixs,
            close_delegated_stake_ix,
            deactivate_primary_stake_acc_ix,
            delegate_stake_ix,
            init_arcium_token_mint_ixs,
            init_delegated_stake_acc_ix,
            init_delegated_stake_master_acc_ix,
            init_primary_stake_acc_ix,
            merge_delegated_stake_account_ix,
            split_delegated_stake_account_ix,
            undelegate_stake_ix,
        },
        pda::arcium_mint_keypair,
    };
    use anchor_client::solana_sdk::program_pack::Pack;
    use anchor_spl::token::spl_token::{state::Mint, ID as TOKEN_PROGRAM_ID};

    pub async fn init_arcium_token_mint(
        cluster: SolanaCluster,
        signer: Arc<Keypair>,
        rpc: AsyncRpcClient,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let minimum_balance_for_rent_exemption = rpc
            .get_minimum_balance_for_rent_exemption(Mint::LEN)
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to fetch min balance for rent exemption: {}", err)
            });
        let token_client = token_program_client(cluster, signer.clone(), rpc);
        let ixs = init_arcium_token_mint_ixs(minimum_balance_for_rent_exemption, &signer.pubkey());

        let tx = token_client
            .request()
            .instruction(ixs[0].clone())
            .instruction(ixs[1].clone());

        send_tx(vec![signer, Arc::new(arcium_mint_keypair())], tx, config).await
    }

    pub async fn airdrop_arcium_token(
        cluster: SolanaCluster,
        signer: Arc<Keypair>,
        recipient: &Pubkey,
        amount: u64,
        rpc: AsyncRpcClient,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let token_client = token_program_client(cluster, signer.clone(), rpc);
        let ixs = airdrop_arcium_token_ixs(&signer.pubkey(), recipient, amount);
        let tx = token_client
            .request()
            .instruction(ixs[0].clone())
            .instruction(ixs[1].clone());

        send_tx(vec![signer], tx, config).await
    }

    fn token_program_client(
        cluster: SolanaCluster,
        signer: Arc<Keypair>,
        rpc: AsyncRpcClient,
    ) -> Program<Arc<Keypair>> {
        let client = Client::new(cluster, signer);
        client
            .program(TOKEN_PROGRAM_ID, rpc)
            .unwrap_or_else(|err| panic!("Failed to create program: {}", err))
    }

    pub async fn init_primary_stake_acc(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        amount: u64,
        fee_basis_points: u16,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = init_primary_stake_acc_ix(&signer.pubkey(), amount, fee_basis_points);
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }

    pub async fn activate_primary_stake_acc(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        lockup_epochs: u64,

        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = activate_primary_stake_acc_ix(&signer.pubkey(), lockup_epochs);
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }

    pub async fn deactivate_primary_stake_acc(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        arx_node_offset: Option<u32>,

        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = deactivate_primary_stake_acc_ix(&signer.pubkey(), arx_node_offset);
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }

    pub async fn init_delegated_stake_master_acc(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        owner: &Pubkey,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = init_delegated_stake_master_acc_ix(&signer.pubkey(), owner);
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }

    pub async fn init_delegated_stake_acc(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        stake_offset: u128,
        amount: u64,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = init_delegated_stake_acc_ix(&signer.pubkey(), stake_offset, amount);
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }

    pub async fn delegate_stake(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        stake_offset: u128,
        primary_stake_owner: &Pubkey,
        lockup_epochs: u64,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = delegate_stake_ix(
            &signer.pubkey(),
            stake_offset,
            primary_stake_owner,
            lockup_epochs,
        );
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }

    pub async fn undelegate_stake(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        stake_offset: u128,
        primary_stake_owner: &Pubkey,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = undelegate_stake_ix(&signer.pubkey(), stake_offset, primary_stake_owner);
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn split_delegated_stake_account(
        arcium_program: &Program<Arc<Keypair>>,
        primary_stake_owner_target: &Pubkey,
        delegation_authority: Arc<Keypair>,
        withdrawal_authority: Arc<Keypair>,
        stake_offset: u128,
        stake_offset_new: u128,
        new_acc_balance: u64,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = split_delegated_stake_account_ix(
            primary_stake_owner_target,
            &delegation_authority.pubkey(),
            &withdrawal_authority.pubkey(),
            stake_offset,
            stake_offset_new,
            new_acc_balance,
        );
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![delegation_authority, withdrawal_authority], tx, config).await
    }

    pub async fn merge_delegated_stake_account(
        arcium_program: &Program<Arc<Keypair>>,
        primary_stake_owner_target: &Pubkey,
        delegation_authority: Arc<Keypair>,
        withdrawal_authority: Arc<Keypair>,
        stake_offset_keep: u128,
        stake_offset_close: u128,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = merge_delegated_stake_account_ix(
            primary_stake_owner_target,
            &delegation_authority.pubkey(),
            &withdrawal_authority.pubkey(),
            stake_offset_keep,
            stake_offset_close,
        );
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![delegation_authority, withdrawal_authority], tx, config).await
    }

    pub async fn close_delegated_stake(
        arcium_program: &Program<Arc<Keypair>>,
        signer: Arc<Keypair>,
        delegation_owner: &Pubkey,
        stake_offset: u128,
        config: RpcSendTransactionConfig,
    ) -> Signature {
        let ix = close_delegated_stake_ix(&signer.pubkey(), delegation_owner, stake_offset);
        let tx = arcium_program.request().instruction(ix);
        send_tx(vec![signer], tx, config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_client::solana_sdk::signature::Keypair;
    use solana_program::hash::Hash;

    #[test]
    fn test_build_claim_failure_txs_single_tx_at_max_size() {
        let payer = Arc::new(Keypair::new());
        let rpc = AsyncRpcClient::new("http://localhost:8899".to_string());
        let arcium_program = arcium_program_client(SolanaCluster::Localnet, payer.clone(), rpc);

        let signer_pubkey = payer.pubkey();
        let mxe_program_id = Pubkey::new_unique();
        let callback_data = vec![0u8; MAX_SINGLE_TX_FAILURE_CLAIM_DATA_SIZE];

        let txs = build_claim_failure_txs(
            &arcium_program,
            &signer_pubkey,
            &mxe_program_id,
            0,
            0,
            0,
            0,
            callback_data,
            0,
        );

        assert_eq!(
            txs.len(),
            1,
            "Expected exactly 1 transaction for data at MAX_SINGLE_TX_FAILURE_CLAIM_DATA_SIZE"
        );
        let mut tx = txs[0].transaction().unwrap();
        tx.sign(&[payer], Hash::new_unique());
        let serialized = bincode::serialize(&tx).unwrap();
        assert_eq!(
            serialized.len(),
            1232,
            "Expected max size of serialized transaction to be 1232 bytes but got {}",
            serialized.len()
        );
    }
}
