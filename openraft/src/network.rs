//! The Raft network interface.

use anyhow::Result;
use async_trait::async_trait;

use crate::raft::AppendEntriesRequest;
use crate::raft::AppendEntriesResponse;
use crate::raft::InstallSnapshotRequest;
use crate::raft::InstallSnapshotResponse;
use crate::raft::VoteRequest;
use crate::raft::VoteResponse;
use crate::AppData;
use crate::NodeId;

/// A trait defining the interface for a Raft network between cluster members.
///
/// See the [network chapter of the guide](https://datafuselabs.github.io/openraft/getting-started.html#3-impl-raftnetwork)
/// for details and discussion on this trait and how to implement it.
#[async_trait]
pub trait RaftNetwork<D>: Send + Sync + 'static
where D: AppData
{
    /// Send an AppendEntries RPC to the target Raft node (§5).
    async fn send_append_entries(&self, target: NodeId, rpc: AppendEntriesRequest<D>) -> Result<AppendEntriesResponse>;

    /// Send an InstallSnapshot RPC to the target Raft node (§7).
    async fn send_install_snapshot(
        &self,
        target: NodeId,
        rpc: InstallSnapshotRequest,
    ) -> Result<InstallSnapshotResponse>;

    /// Send a RequestVote RPC to the target Raft node (§5).
    async fn send_vote(&self, target: NodeId, rpc: VoteRequest) -> Result<VoteResponse>;
}
