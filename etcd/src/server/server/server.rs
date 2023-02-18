/*!
<https://github.com/etcd-io/etcd/blob/main/server/etcdserver/server.go>
*/

use std::error::Error;
use std::task::Context;

use crate::api::membership::Member;

trait Server {
    /**
    `add_member` attempts to add a member into the cluster. It will return `ErrIDRemoved` if member ID is removed from the cluster, or return `ErrIDExists` if member ID exists in the cluster.
    */
    fn add_member(context: Context, member: Member) -> Result<Vec<Member>, dyn Error>;
    /**
    `remove_member` attempts to remove a member from the cluster. It will return `ErrIDRemoved` if member ID is removed from the cluster, or return `ErrIDNotFound` if member ID is not in the cluster.
    */
    fn remove_member(context: Context, id: u64) -> Result<Vec<Member>, dyn Error>;
    /**
    `update_member` attempts to update an existing member in the cluster. It will return ErrIDNotFound if the member ID does not exist.
    */
    fn update_member(context: Context, updateMemb: Member) -> Result<Vec<Member>, dyn Error>;
    /**
    `promote_member` attempts to promote a non-voting node to a voting node. It will
    return `ErrIDNotFound` if the member ID does not exist.
    return `ErrLearnerNotReady` if the member are not ready.
    return `ErrMemberNotLearner` if the member is not a learner.
    */
    fn promote_member(context: Context, id: u64) -> Result<Vec<Member>, dyn Error>;

    /**
    `cluster_version` is the cluster-wide minimum major.minor version.
    Cluster version is set to the min version that an etcd member is compatible with when first bootstrap.

    cluster_version is nil until the cluster is bootstrapped (has a quorum).

    During a rolling upgrades, the cluster_version will be update automatically after a sync. (5 second by default)

    The API/raft component can utilize cluster_version to determine if it can accept a client request or a raft RPC.
    NOTE: cluster_version might be nil when etcd 2.1 works with etcd 2.0 and the leader is etcd 2.0. etcd 2.0 leader will not update clusterVersion since this feature is introduced post 2.0.
    */
    fn cluster_version() -> String;
    /*
    `storage_version` is the storage schema version. It's supported starting
    from 3.6.
    */
    fn storage_version() -> String;
    fn cluster() -> api::Cluster;
    fn alarms() -> Vec<pb::AlarmMember>;

    /**
    `leader_changed_notify` returns a channel for application level code to be notified when etcd leader changes, this function is intend to be used only in application which embed etcd.
    Caution:
    1. the returned channel is being closed when the leadership changes.
    2. so the new channel needs to be obtained for each raft term.
    3. user can loose some consecutive channel changes using this API.
    */
    fn leader_changed_notify() <-chan struct{}
}
