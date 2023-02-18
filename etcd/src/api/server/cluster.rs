/*!
<https://github.com/etcd-io/etcd/blob/main/api/etcdserverpb/rpc.proto>
*/

trait Cluster {
    // member_add adds a member into the cluster.
    // fn member_add(MemberAddRequest) -> (MemberAddResponse) {
    //     option (google.api.http) = {
    //       post: "/v3/cluster/member/add"
    //       body: "*"
    //   };
    // }

    // // member_remove removes an existing member from the cluster.
    // fn member_remove(MemberRemoveRequest) -> (MemberRemoveResponse) {
    //     option (google.api.http) = {
    //       post: "/v3/cluster/member/remove"
    //       body: "*"
    //   };
    // }

    // // member_update updates the member configuration.
    // fn member_update(MemberUpdateRequest) -> (MemberUpdateResponse) {
    //     option (google.api.http) = {
    //       post: "/v3/cluster/member/update"
    //       body: "*"
    //   };
    // }

    // // member_list lists all the members in the cluster.
    // fn member_list(MemberListRequest) -> (MemberListResponse) {
    //     option (google.api.http) = {
    //       post: "/v3/cluster/member/list"
    //       body: "*"
    //   };
    // }

    // // member_promote promotes a member from raft learner (non-voting) to raft voting member.
    // fn member_promote(MemberPromoteRequest) -> (MemberPromoteResponse) {
    //     option (google.api.http) = {
    //       post: "/v3/cluster/member/promote"
    //       body: "*"
    //   };
    // }
}

pub struct ResponseHeader {
    /// cluster_id is the ID of the cluster which sent the response.
    cluster_id: u64,
    /// member_id is the ID of the member which sent the response.
    member_id: u64,
    /**
    revision is the key-value store revision when the request was applied, and it's
    unset (so 0) in case of calls not interacting with key-value store.
    For watch progress responses, the header.revision indicates progress. All future events
    received in this stream are guaranteed to have a higher revision number than the
    header.revision number.
    */
    revision: i64,
    /// raft_term is the raft term when the request was applied.
    raft_term: u64 
}

pub struct Member {
    /// `id` is the member ID for this member.
    id: u64,
    /// name is the human-readable name of the member. If the member is not started, the name will be an empty string.
    name: String,
    /// peerURLs is the list of URLs the member exposes to the cluster for communication.
    peer_urls: Vec<String>,
    /// clientURLs is the list of URLs the member exposes to clients for communication. If the member is not started, clientURLs will be empty.
    client_urls: Vec<String>,
    /// is_learner indicates if the member is raft learner.
    is_learner: bool
}

pub struct MemberAddRequest {
    /// `peer_urls` is the list of URLs the added member will use to communicate with the cluster.
    peer_urls: Vec<String>,
    /// `is_learner` indicates if the added member is raft learner.
    is_learner: bool
}

pub struct MemberAddResponse {
    header: ResponseHeader,
    /// `member` is the member information for the added member.
    member: Member,
    /// `members` is a list of all members after adding the new member.
    members: Vec<Member>
}

struct MemberRemoveRequest {
    /// `id` is the member ID of the member to remove.
    id: u64,
}

struct MemberRemoveResponse {
    header: ResponseHeader,
    /// `members` is a list of all members after removing the member.
    members: Vec<Member>,
}

struct MemberUpdateRequest {
    /// `id` is the member ID of the member to update.
    id: u64,
    /// `peer_urls` is the new list of URLs the member will use to communicate with the cluster.
    peer_urls: Vec<String>,
}

struct MemberUpdateResponse{
    header: ResponseHeader,
    /// members is a list of all members after updating the member.
    members: Vec<Member>
}

pub struct MemberListRequest {
    linearisable: bool
}

pub struct MemberListResponse {
    header: ResponseHeader,
    /// `members` is a list of all members associated with the cluster.
    members: Vec<Member>
}

pub struct MemberPromoteRequest {
    /// `id` is the member ID of the member to promote.
    id: u64,
}

pub struct MemberPromoteResponse {
    header: ResponseHeader,
    /// members is a list of all members after promoting the member.
    members: Vec<Member>,
}
