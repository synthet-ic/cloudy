pub struct ClusterNodeStatus {
    /// ID that should be used by the client to receive a diff from the previous request
    client_id: i64,

    /// List of known nodes
    nodes_added: Vec<NodeElement>,

    /// List of known nodes
    nodes_removed: Vec<NodeElement>,

    /// Name of local node (if available)
    r#self: String
}
