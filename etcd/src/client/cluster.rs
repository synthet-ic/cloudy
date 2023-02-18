/*!
<https://github.com/etcd-io/etcd/blob/main/client/v3/cluster.go>
*/

use std::error::Error;
use std::task::Context;

use crate::{
    api::server::cluster::{
        MemberAddRequest, MemberAddResponse, MemberRemoveRequest, MemberRemoveResponse, MemberUpdateRequest, MemberUpdateResponse, MemberListRequest, MemberListResponse, MemberPromoteRequest, MemberPromoteResponse
    },
    client::types::urls::new_urls
};

pub struct Cluster {
    remote: ClusterClient,
    call_opts: Option<grpc::CallOption>
}

impl Cluster {
    /// Lists the current cluster membership.
    fn member_list(&self, context: Context) -> Result<MemberListResponse, dyn Error> {
        // It is safe to retry on list.
        let r = MemberListRequest { linearisable: true };
        match self.remote.member_list(context, r, self.call_opts) {
            Ok(r) => Ok(MemberListResponse(r)),
            Err(err) => toErr(context, err)
        }
    }

    /// Adds a new member into the cluster.
    fn member_add(&self, context: Context, peer_addrs: Vec<String>) -> Result<MemberAddResponse, dyn Error> {
        self._member_add(context, peer_addrs, false)
    }

    /// Adds a new learner member into the cluster.
    fn member_add_as_learner(&self, context: Context, peer_addrs: Vec<String>) -> Result<MemberAddResponse, dyn Error> {
        self._member_add(context, peer_addrs, true)
    }

    fn _member_add(&self, context: Context, peer_addrs: Vec<String>, is_learner: bool) -> Result<MemberAddResponse, dyn Error> {
        // Fail-fast before panic in rafthttp.
        new_urls(peer_addrs)?;

        let r = MemberAddRequest {
            peer_urls: peer_addrs,
            is_learner,
        };
        match self.remote.member_add(context, r, self.call_opts) {
            Ok(r) => Ok(MemberAddResponse(r)),
            Err(err) => Err(err)
        }
    }

    /// Removes an existing member from the cluster.
    fn member_remove(&self, context: Context, id: u64) -> MemberRemoveResponse {
        let r = MemberRemoveRequest { id };
        match self.remote.member_remove(context, r, self.call_opts) {
            Ok(r) => Ok(MemberRemoveResponse(r)),
            Err(err) => toErr(context, err)
        }
    }

    /// `member_update` updates the peer addresses of the member.
    fn member_update(&self, context: Context, id: u64, peer_addrs: Vec<String>) -> Result<MemberUpdateResponse, dyn Error> {
        // Fail-fast before panic in rafthttp.
        new_urls(peer_addrs)?;

        // It is safe to retry on update.
        let r = MemberUpdateRequest { id, peer_urls: peer_addrs };
        match self.remote.member_update(context, r, self.call_opts) {
            Ok(r) => Ok(MemberUpdateResponse(r)),
            Err(err) => toErr(context, err)
        }
    }

    /// `member_promote` promotes a member from raft learner (non-voting) to raft voting member.
    fn member_promote(&self, context: Context, id: u64) -> Result<MemberPromoteResponse, dyn Error> {
        let r = MemberPromoteRequest { id };
        match self.remote.member_promote(context, r, self.call_opts) {
            Ok(r) => Ok(MemberPromoteResponse(r)),
            Err(err) => Err(err)
        }
    }
}
