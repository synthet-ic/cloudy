/*!
<https://github.com/etcd-io/etcd/blob/main/api/membershippb/membership.proto>
*/

/// RaftAttributes represents the raft related attributes of an etcd member.
pub struct RaftAttributes {
  // option (versionpb.etcd_version_msg) = "3.5";

  /// `peer_urls` is the list of peers in the raft cluster.
  peer_urls: Vec<String>,
  /// `is_learner` indicates if the member is raft learner.
  is_learner: bool
}

/// Attributes represents all the non-raft related attributes of an etcd member.
pub struct Attributes {
  // option (versionpb.etcd_version_msg) = "3.5";

  name: String,
  client_urls: Vec<String>
}

pub struct Member {
  // option (versionpb.etcd_version_msg) = "3.5";

  id: u64,
  raft_attributes: RaftAttributes,
  member_attributes: Attributes
}
