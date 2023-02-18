pub struct ClusterMeshStatus {
    /// List of remote clusters
    clusters: Vec<RemoteCluster>,

    /// Number of global services
    num_global_services: u32
}
