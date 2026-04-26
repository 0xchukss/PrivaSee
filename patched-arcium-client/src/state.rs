use crate::{
    idl::arcium::{
        accounts::{
            ArxNode,
            ClockAccount,
            Cluster,
            LargeExecPool,
            LargeMempool,
            MXEAccount,
            MediumExecPool,
            MediumMempool,
            SmallExecPool,
            SmallMempool,
            TinyExecPool,
            TinyMempool,
        },
        types::{ClusterMembership, ComputationReference},
    },
    pda::{arx_acc, clock_acc, cluster_acc},
};
use anchor_client::{
    anchor_lang::{AccountDeserialize, Discriminator},
    solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig},
};
use anchor_lang::prelude::Pubkey;
use bytemuck::Zeroable;
use solana_rpc_client::nonblocking::rpc_client::RpcClient as AsyncRpcClient;
use solana_rpc_client_api::{
    client_error::Error as SolanaClientError,
    config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use std::{collections::HashSet, hash::Hash};
use thiserror::Error;

/// Minimum context slot for filtering cluster accounts.
/// This helps skip stale accounts from old program versions.
/// Set to last known program upgrade slot
pub const MIN_CLUSTER_CONTEXT_SLOT: u64 = 0;

/// Errors that can occur during cluster offset discovery
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ClusterOffsetError {
    /// Failed to fetch node accounts from RPC
    #[error("Failed to fetch node accounts from RPC: {0}")]
    AccountFetchFailed(String),
    /// Failed to deserialize node account data
    #[error("Failed to deserialize node account data: {0}")]
    DeserializationFailed(String),
    /// Found an inactive node in the cluster
    #[error("Found inactive node in cluster at offset {0}")]
    InactiveNode(u32),
    /// Cluster has no nodes
    #[error("Cluster has no nodes")]
    EmptyCluster,
    /// Node has no cluster membership matching the target cluster
    #[error("No cluster membership found for target cluster")]
    NoClusterMembership,
}

/// Represents the state of a cluster offset lookup.
#[derive(Debug, Clone, PartialEq)]
pub enum ClusterOffsetState {
    /// Offset successfully looked up, cluster is healthy with all nodes active
    Available(u32),
    /// Cluster exists but offset has not yet been looked up (or empty cluster)
    NotLookedUp,
    /// Cluster is unhealthy or unreachable
    Unavailable(ClusterOffsetError),
}

impl ClusterOffsetState {
    /// Returns true if the cluster is available with a known offset
    pub fn is_available(&self) -> bool {
        matches!(self, ClusterOffsetState::Available(_))
    }

    /// Returns the offset if available
    pub fn get(&self) -> Option<u32> {
        match self {
            ClusterOffsetState::Available(offset) => Some(*offset),
            _ => None,
        }
    }

    /// Returns the error if unavailable
    pub fn error(&self) -> Option<&ClusterOffsetError> {
        match self {
            ClusterOffsetState::Unavailable(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for ClusterOffsetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClusterOffsetState::Available(offset) => write!(f, "Available (offset: {})", offset),
            ClusterOffsetState::NotLookedUp => write!(f, "Not looked up"),
            ClusterOffsetState::Unavailable(err) => write!(f, "Unavailable: {}", err),
        }
    }
}

pub async fn arx_acc_active(
    rpc_client: &AsyncRpcClient,
    node_offset: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let arx_acc = arx_acc(node_offset);
    let bytes = rpc_client
        .get_account(&arx_acc)
        .await
        .map_err(|e| format!("Failed to get account data: {}", e))?
        .data;
    let arx_data = ArxNode::try_deserialize(&mut bytes.as_slice())
        .map_err(|e| format!("Failed to deserialize account data: {}", e))?;
    Ok(arx_data.is_active)
}

pub async fn active_proposals(
    rpc_client: &AsyncRpcClient,
    cluster_offset: u32,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let cluster_acc = cluster_acc(cluster_offset);
    let bytes = rpc_client
        .get_account(&cluster_acc)
        .await
        .map_err(|e| format!("Failed to get account data: {}", e))?
        .data;
    let cluster_data = Cluster::try_deserialize(&mut bytes.as_slice())
        .map_err(|e| format!("Failed to deserialize account data: {}", e))?;
    // Proposals are all set to the current price by default, so we filter out duplicates
    Ok(dedupe(cluster_data.cu_price_proposals.to_vec()))
}

fn dedupe<T: PartialEq + Eq + Hash + Copy>(arr: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for &item in arr.iter() {
        if seen.insert(item) {
            result.push(item);
        }
    }

    result
}

/// Fetches all Cluster accounts from the Arcium program using discriminator filtering.
///
/// # Arguments
/// * `rpc_client` - The RPC client to use for fetching accounts
/// * `min_context_slot` - Optional minimum context slot to filter out stale accounts from old
///   program versions
pub async fn get_all_cluster_accounts(
    rpc_client: &AsyncRpcClient,
    min_context_slot: Option<u64>,
) -> Result<Vec<(Pubkey, Cluster)>, Box<dyn std::error::Error>> {
    let program_id = crate::idl::arcium::ID;
    let discriminator = Cluster::DISCRIMINATOR;

    let memcmp_filter = RpcFilterType::Memcmp(Memcmp::new(
        0,
        MemcmpEncodedBytes::Bytes(discriminator.to_vec()),
    ));

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![memcmp_filter]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: None,
            data_slice: None,
            min_context_slot,
        },
        with_context: None,
        sort_results: None,
    };

    let accounts = rpc_client
        .get_program_accounts_with_config(&program_id, config)
        .await?;

    let mut clusters = Vec::new();
    for (pubkey, account) in accounts {
        // Discriminator check is done server-side, so we can directly deserialize
        match Cluster::try_deserialize(&mut account.data.as_slice()) {
            Ok(cluster) => clusters.push((pubkey, cluster)),
            Err(_) => continue, // Silently skip malformed accounts
        }
    }

    Ok(clusters)
}

async fn get_mxe_count(
    rpc_client: &AsyncRpcClient,
    min_context_slot: Option<u64>,
    cluster_offset: u32,
) -> Result<usize, Box<dyn std::error::Error>> {
    let program_id = crate::idl::arcium::ID;
    let discriminator = MXEAccount::DISCRIMINATOR;
    // We want to search for MXE accounts that have us as their cluster. MXE accounts store the
    // cluster they're assigned to in the first field which is an Option<u32> serialized to
    // borsh. This means the first 13 bytes for valid mxes are:
    // - 8 bytes of discriminator to specify mxe
    // - 1 bytes set to 1 to specify Some(..)
    // - 4 bytes matching the cluster offset
    let mut comparative_bytes = Vec::with_capacity(13);
    comparative_bytes.extend_from_slice(discriminator);
    comparative_bytes.push(1);
    comparative_bytes.extend_from_slice(&cluster_offset.to_le_bytes());

    let memcmp_filter =
        RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Bytes(comparative_bytes)));

    // Fetch 0 length of data, as we only care about the count of accounts but not their content
    let data_slice_config = UiDataSliceConfig {
        offset: 0,
        length: 0,
    };

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![memcmp_filter]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: None,
            data_slice: Some(data_slice_config),
            min_context_slot,
        },
        with_context: None,
        sort_results: None,
    };

    let accounts = rpc_client
        .get_program_accounts_with_config(&program_id, config)
        .await?;

    Ok(accounts.len())
}

/// Enriched cluster information for display and decision-making.
/// Wraps the IDL-generated Cluster account with additional metadata.
#[derive(Debug, Clone)]
pub struct ClusterInfo {
    pub pubkey: Pubkey,
    /// Cluster offset lookup state
    pub offset: ClusterOffsetState,
    pub cluster: Cluster,
    /// Number of MXEs in the cluster, optional because we don't know it yet
    pub mxe_count: usize,
}

impl ClusterInfo {
    /// Get the number of nodes currently in the cluster
    pub fn node_count(&self) -> usize {
        self.cluster.nodes.len()
    }

    /// Get the maximum number of nodes allowed
    pub fn max_nodes(&self) -> u16 {
        self.cluster.cluster_size
    }

    /// Get the number of pending nodes
    pub fn pending_node_count(&self) -> usize {
        self.cluster.pending_nodes.len()
    }

    /// Calculate node utilization as a percentage
    pub fn node_utilization_percent(&self) -> f32 {
        let max = self.max_nodes();
        if max > 0 {
            (self.node_count() as f32 / max as f32) * 100.0
        } else {
            0.0
        }
    }
}

/// Fetches all clusters and enriches with computed metrics.
///
/// Cluster offsets are resolved by querying node cluster memberships.
/// Empty clusters (no nodes) will have `offset: None`.
///
/// # Arguments
/// * `rpc_client` - The RPC client to use for fetching accounts
/// * `_current_epoch` - Current epoch (reserved for future use)
/// * `min_context_slot` - Optional minimum context slot to filter out stale accounts from old
///   program versions
///
/// # Performance
/// O(N) RPC calls instead of O(N × 1000) PDA derivations.
pub async fn get_cluster_discovery_info(
    rpc_client: &AsyncRpcClient,
    _current_epoch: u64,
    min_context_slot: Option<u64>,
) -> Result<Vec<ClusterInfo>, Box<dyn std::error::Error>> {
    let clusters = get_all_cluster_accounts(rpc_client, min_context_slot).await?;

    let mut infos = Vec::with_capacity(clusters.len());
    for (pubkey, cluster) in clusters {
        // Look up cluster offset and validate all nodes are active
        let offset = find_cluster_offset_via_node(rpc_client, &cluster).await;
        let mxe_count = if let ClusterOffsetState::Available(offset) = offset {
            let mxe_count = get_mxe_count(rpc_client, min_context_slot, offset).await?;
            mxe_count
        } else {
            0
        };

        infos.push(ClusterInfo {
            pubkey,
            offset,
            cluster,
            mxe_count,
        });
    }

    Ok(infos)
}

/// Efficiently finds cluster offset by validating all nodes are active.
/// Returns error if ANY node is inactive - we only recommend fully healthy clusters.
/// Also returns error for empty clusters, fetch failures, or missing cluster membership.
async fn find_cluster_offset_via_node(
    rpc_client: &AsyncRpcClient,
    cluster: &Cluster,
) -> ClusterOffsetState {
    if cluster.nodes.is_empty() {
        return ClusterOffsetState::Unavailable(ClusterOffsetError::EmptyCluster);
    }

    let node_pubkeys: Vec<Pubkey> = cluster
        .nodes
        .iter()
        .map(|node_ref| arx_acc(node_ref.offset))
        .collect();

    let accounts = match rpc_client.get_multiple_accounts(&node_pubkeys).await {
        Ok(accounts) => accounts,
        Err(e) => {
            return ClusterOffsetState::Unavailable(ClusterOffsetError::AccountFetchFailed(
                e.to_string(),
            ))
        }
    };

    // Safety: Ensure returned accounts match requested nodes
    debug_assert_eq!(
        accounts.len(),
        cluster.nodes.len(),
        "RPC returned {} accounts but requested {} nodes",
        accounts.len(),
        cluster.nodes.len()
    );

    let mut found_offset = None;

    for (i, maybe_account) in accounts.iter().enumerate() {
        let node_offset = cluster.nodes[i].offset;

        let account = match maybe_account.as_ref() {
            Some(acc) => acc,
            None => {
                return ClusterOffsetState::Unavailable(ClusterOffsetError::AccountFetchFailed(
                    format!("Node account not found: {}", node_offset),
                ))
            }
        };

        let node = match ArxNode::try_deserialize(&mut account.data.as_slice()) {
            Ok(node) => node,
            Err(e) => {
                return ClusterOffsetState::Unavailable(ClusterOffsetError::DeserializationFailed(
                    e.to_string(),
                ))
            }
        };

        if !node.is_active {
            return ClusterOffsetState::Unavailable(ClusterOffsetError::InactiveNode(node_offset));
        }

        if found_offset.is_none() {
            if let ClusterMembership::Active(cluster_offset) = &node.cluster_membership {
                found_offset = Some(*cluster_offset);
            }
        }
    }

    match found_offset {
        Some(offset) => ClusterOffsetState::Available(offset),
        None => ClusterOffsetState::Unavailable(ClusterOffsetError::NoClusterMembership),
    }
}

/// Gets the current epoch from the on-chain clock account
pub async fn get_current_epoch(
    rpc_client: &AsyncRpcClient,
) -> Result<u64, Box<dyn std::error::Error>> {
    let clock_pubkey = clock_acc();
    let account = rpc_client.get_account(&clock_pubkey).await?;
    let clock_data = ClockAccount::try_deserialize(&mut account.data.as_slice())?;

    Ok(clock_data.current_epoch.0)
}

pub async fn get_mempool_acc_data(
    rpc: &AsyncRpcClient,
    mempool_acc: &Pubkey,
) -> Result<MempoolWrapper, ComputationPoolError> {
    let mempool_data = rpc
        .get_account_data(mempool_acc)
        .await
        .map_err(ComputationPoolError::new_solana_error)?;
    MempoolWrapper::from_raw(&mempool_data)
}

// This is a zero-copy account with different size variants, so we leave the deserialization to the
// caller.
pub async fn get_mempool_acc_data_raw(
    rpc: &AsyncRpcClient,
    mempool_acc: &Pubkey,
) -> Result<Vec<u8>, SolanaClientError> {
    let mempool_data = rpc.get_account_data(mempool_acc).await?;
    Ok(mempool_data)
}

pub async fn get_execpool_acc_data(
    rpc: &AsyncRpcClient,
    execpool_acc: &Pubkey,
) -> Result<ExecpoolWrapper, ComputationPoolError> {
    let execpool_data = rpc
        .get_account_data(execpool_acc)
        .await
        .map_err(ComputationPoolError::new_solana_error)?;
    ExecpoolWrapper::from_raw(&execpool_data)
}

// This is a zero-copy account with different size variants, so we leave the deserialization to the
// caller.
pub async fn get_execpool_acc_data_raw(
    rpc: &AsyncRpcClient,
    execpool_acc: &Pubkey,
) -> Result<Vec<u8>, SolanaClientError> {
    let execpool_data = rpc.get_account_data(execpool_acc).await?;
    Ok(execpool_data)
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct MempoolInfo {
    pub cluster: Pubkey,
    pub mxe: Pubkey,
    pub mempool: Pubkey,
}

// TODO: Check support for medium and large mempools (current problem is they're too big and
// cause a stack overflow) We should do an optimization where we only fetch/receive the
// heap at current_slot (since that's the only thing that changes), and not the entire mempool
pub enum MempoolWrapper {
    Tiny(Box<TinyMempool>),
    Small(Box<SmallMempool>),
    Medium(Box<MediumMempool>),
    Large(Box<LargeMempool>),
}

#[derive(Debug)]
pub enum ComputationPoolError {
    InvalidDiscriminator,
    InvalidSize,
    ClientError(Box<SolanaClientError>),
}

impl ComputationPoolError {
    pub fn new_solana_error(err: SolanaClientError) -> Self {
        ComputationPoolError::ClientError(Box::new(err))
    }
}

macro_rules! extract_computations {
    ($inner:expr) => {{
        let start_index = $inner.computations.start_index as usize;
        let buffer_size = $inner.computations.elems.len();

        $inner
            .computations
            .elems
            .into_iter()
            .enumerate()
            .filter(|(i, _)| {
                // This is a circular buffer, so we need to normalize the index
                let normalized_i = if *i >= start_index {
                    *i - start_index
                } else {
                    buffer_size - start_index + *i
                };
                Self::is_valid(&$inner.computations.valid_bits, normalized_i)
                    && normalized_i < $inner.computations.length as usize
            })
            .flat_map(|(_, h)| h.entries.into_iter())
            .filter(|computation| !is_empty_computation_ref(computation))
            .collect()
    }};
}

macro_rules! extract_computations_highest_prio {
    ($inner:expr) => {{
        let start_index = $inner.computations.start_index as usize;
        let buffer_size = $inner.computations.elems.len();

        $inner
            .computations
            .elems
            .into_iter()
            .enumerate()
            .filter_map(|(i, h)| {
                // Normalize circular buffer index
                let normalized_i = if i >= start_index {
                    i - start_index
                } else {
                    buffer_size - start_index + i
                };

                // Check validity and bounds
                if Self::is_valid(&$inner.computations.valid_bits, normalized_i)
                    && normalized_i < $inner.computations.length as usize
                {
                    // Pick only the first entry if non-empty
                    let mut entries = h.entries.into_iter();
                    let first = entries.next()?;
                    if !is_empty_computation_ref(&first) {
                        return Some(first);
                    }
                }
                None
            })
            .collect()
    }};
}
macro_rules! deserialize_mempool {
    ($raw:expr, $mempool:ty, $variant:ident) => {{
        let offset = <$mempool as Discriminator>::DISCRIMINATOR.len();
        if offset + std::mem::size_of::<$mempool>() > $raw.len() {
            return Err(ComputationPoolError::InvalidSize);
        }
        let data = bytemuck::pod_read_unaligned::<$mempool>(
            &$raw[offset..offset + std::mem::size_of::<$mempool>()],
        );
        Ok(MempoolWrapper::$variant(Box::new(data)))
    }};
}

impl MempoolWrapper {
    pub fn computations_raw(self) -> Vec<(bool, Vec<ComputationReference>, usize, usize)> {
        match self {
            MempoolWrapper::Tiny(tm) => {
                let tm = *tm;
                let start_index = tm.inner.computations.start_index as usize;
                let mut res = Vec::with_capacity(180);
                tm.inner
                    .computations
                    .elems
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, h)| {
                        let normalized_i = if i >= start_index {
                            i - start_index
                        } else {
                            tm.inner.computations.elems.len() - start_index + i
                        };
                        res[normalized_i] = (
                            Self::is_valid(&tm.inner.computations.valid_bits, normalized_i),
                            h.entries
                                .into_iter()
                                .filter(|c| !is_empty_computation_ref(c))
                                .collect(),
                            normalized_i,
                            i,
                        );
                    });
                res
            }
            MempoolWrapper::Small(sm) => {
                let sm = *sm;
                let start_index = sm.inner.computations.start_index as usize;
                let mut res = Vec::with_capacity(180);
                sm.inner
                    .computations
                    .elems
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, h)| {
                        let normalized_i = if i >= start_index {
                            i - start_index
                        } else {
                            sm.inner.computations.elems.len() - start_index + i
                        };
                        res[normalized_i] = (
                            Self::is_valid(&sm.inner.computations.valid_bits, normalized_i),
                            h.entries
                                .into_iter()
                                .filter(|c| !is_empty_computation_ref(c))
                                .collect(),
                            normalized_i,
                            i,
                        );
                    });
                res
            }
            MempoolWrapper::Medium(mm) => {
                let mm = *mm;
                let start_index = mm.inner.computations.start_index as usize;
                let mut res = Vec::with_capacity(180);
                mm.inner
                    .computations
                    .elems
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, h)| {
                        let normalized_i = if i >= start_index {
                            i - start_index
                        } else {
                            mm.inner.computations.elems.len() - start_index + i
                        };
                        res[normalized_i] = (
                            Self::is_valid(&mm.inner.computations.valid_bits, normalized_i),
                            h.entries
                                .into_iter()
                                .filter(|c| !is_empty_computation_ref(c))
                                .collect(),
                            normalized_i,
                            i,
                        );
                    });
                res
            }
            MempoolWrapper::Large(lm) => {
                let lm = *lm;
                let start_index = lm.inner.computations.start_index as usize;
                let mut res = Vec::with_capacity(180);
                lm.inner
                    .computations
                    .elems
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, h)| {
                        let normalized_i = if i >= start_index {
                            i - start_index
                        } else {
                            lm.inner.computations.elems.len() - start_index + i
                        };
                        res[normalized_i] = (
                            Self::is_valid(&lm.inner.computations.valid_bits, normalized_i),
                            h.entries
                                .into_iter()
                                .filter(|c| !is_empty_computation_ref(c))
                                .collect(),
                            normalized_i,
                            i,
                        );
                    });
                res
            }
        }
    }

    pub fn computations(self) -> Vec<ComputationReference> {
        match self {
            MempoolWrapper::Tiny(tm) => extract_computations!(tm.inner),
            MempoolWrapper::Small(sm) => extract_computations!(sm.inner),
            MempoolWrapper::Medium(mm) => extract_computations!(mm.inner),
            MempoolWrapper::Large(lm) => extract_computations!(lm.inner),
        }
    }

    pub fn computations_highest_prio(self) -> Vec<ComputationReference> {
        match self {
            MempoolWrapper::Tiny(tm) => extract_computations_highest_prio!(tm.inner),
            MempoolWrapper::Small(sm) => extract_computations_highest_prio!(sm.inner),
            MempoolWrapper::Medium(mm) => extract_computations_highest_prio!(mm.inner),
            MempoolWrapper::Large(lm) => extract_computations_highest_prio!(lm.inner),
        }
    }
    // Returns None if the mempool wasn't properly initialized (i.e. has the incorrect length)
    pub fn from_raw(raw_mempool: &[u8]) -> Result<Self, ComputationPoolError> {
        match &raw_mempool[0..8] {
            TinyMempool::DISCRIMINATOR => deserialize_mempool!(raw_mempool, TinyMempool, Tiny),
            SmallMempool::DISCRIMINATOR => deserialize_mempool!(raw_mempool, SmallMempool, Small),
            MediumMempool::DISCRIMINATOR => {
                deserialize_mempool!(raw_mempool, MediumMempool, Medium)
            }
            LargeMempool::DISCRIMINATOR => deserialize_mempool!(raw_mempool, LargeMempool, Large),
            _ => Err(ComputationPoolError::InvalidDiscriminator),
        }
    }

    // Returns true if the heap at index idx is valid (i.e. not stale)
    fn is_valid(valid_bits: &[u8], idx: usize) -> bool {
        let byte = idx / 8;
        let bit = idx - (byte * 8);

        if byte >= valid_bits.len() {
            return false;
        }

        (valid_bits[byte] & (1 << bit)) != 0
    }
}

pub fn is_empty_computation_ref(c: &ComputationReference) -> bool {
    *c == ComputationReference::zeroed()
}

impl std::fmt::Display for ComputationReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Computation offset: {}, priority fee: {}",
            self.computation_offset, self.priority_fee
        )
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct ExecpoolInfo {
    pub cluster: Pubkey,
    pub mxe: Pubkey,
    pub execpool: Pubkey,
}

pub enum ExecpoolWrapper {
    Tiny(Box<TinyExecPool>),
    Small(Box<SmallExecPool>),
    Medium(Box<MediumExecPool>),
    Large(Box<LargeExecPool>),
}

impl ExecpoolWrapper {
    pub fn computations_unfiltered(self) -> Vec<ComputationReferenceWIndex> {
        match self {
            ExecpoolWrapper::Tiny(te) => {
                let te = *te;
                te.inner
                    .currently_executing
                    .into_iter()
                    .enumerate()
                    .map(|(i, reference)| ComputationReferenceWIndex {
                        reference,
                        index: te.inner.execpool_index[i],
                    })
                    .collect()
            }
            ExecpoolWrapper::Small(se) => {
                let se = *se;
                se.inner
                    .currently_executing
                    .into_iter()
                    .enumerate()
                    .map(|(i, reference)| ComputationReferenceWIndex {
                        reference,
                        index: se.inner.execpool_index[i],
                    })
                    .collect()
            }
            ExecpoolWrapper::Medium(me) => {
                let me = *me;
                me.inner
                    .currently_executing
                    .into_iter()
                    .enumerate()
                    .map(|(i, reference)| ComputationReferenceWIndex {
                        reference,
                        index: me.inner.execpool_index[i],
                    })
                    .collect()
            }
            ExecpoolWrapper::Large(le) => {
                let le = *le;
                le.inner
                    .currently_executing
                    .into_iter()
                    .enumerate()
                    .map(|(i, reference)| ComputationReferenceWIndex {
                        reference,
                        index: le.inner.execpool_index[i],
                    })
                    .collect()
            }
        }
    }

    pub fn computations(self) -> Vec<ComputationReferenceWIndex> {
        self.computations_unfiltered()
            .into_iter()
            .filter(|computation| !is_empty_computation_ref(&computation.reference))
            .collect()
    }

    pub fn from_raw(raw_mempool: &[u8]) -> Result<Self, ComputationPoolError> {
        match &raw_mempool[0..8] {
            TinyExecPool::DISCRIMINATOR => {
                let offset = TinyExecPool::DISCRIMINATOR.len();
                if offset + std::mem::size_of::<TinyExecPool>() > raw_mempool.len() {
                    return Err(ComputationPoolError::InvalidSize);
                }
                let te = bytemuck::pod_read_unaligned::<TinyExecPool>(
                    &raw_mempool[offset..offset + std::mem::size_of::<TinyExecPool>()],
                );
                Ok(ExecpoolWrapper::Tiny(Box::new(te)))
            }
            SmallExecPool::DISCRIMINATOR => {
                let offset = SmallExecPool::DISCRIMINATOR.len();
                if offset + std::mem::size_of::<SmallExecPool>() > raw_mempool.len() {
                    return Err(ComputationPoolError::InvalidSize);
                }
                let se = bytemuck::pod_read_unaligned::<SmallExecPool>(
                    &raw_mempool[offset..offset + std::mem::size_of::<SmallExecPool>()],
                );
                Ok(ExecpoolWrapper::Small(Box::new(se)))
            }
            MediumExecPool::DISCRIMINATOR => {
                let offset = MediumExecPool::DISCRIMINATOR.len();
                if offset + std::mem::size_of::<MediumExecPool>() > raw_mempool.len() {
                    return Err(ComputationPoolError::InvalidSize);
                }
                let me = bytemuck::pod_read_unaligned::<MediumExecPool>(
                    &raw_mempool[offset..offset + std::mem::size_of::<MediumExecPool>()],
                );
                Ok(ExecpoolWrapper::Medium(Box::new(me)))
            }
            LargeExecPool::DISCRIMINATOR => {
                let offset = LargeExecPool::DISCRIMINATOR.len();
                if offset + std::mem::size_of::<LargeExecPool>() > raw_mempool.len() {
                    return Err(ComputationPoolError::InvalidSize);
                }
                let le = bytemuck::pod_read_unaligned::<LargeExecPool>(
                    &raw_mempool[offset..offset + std::mem::size_of::<LargeExecPool>()],
                );
                Ok(ExecpoolWrapper::Large(Box::new(le)))
            }
            _ => Err(ComputationPoolError::InvalidDiscriminator),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ComputationReferenceWIndex {
    pub reference: ComputationReference,
    pub index: u64,
}
