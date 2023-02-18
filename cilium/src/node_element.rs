pub struct NodeElement {
    /// Address used for probing cluster connectivity
    health_endpoint_address: NodeAddressing,

    /// Source address for Ingress listener
    ingress_address: Vec<NodeAddressing>,

    /// Name of the node including the cluster association. This is typically
    /// <clustername>/<hostname>.
    name: String,

    /// Primary address used for intra-cluster communication
    primary_address: Option<NodeAddressing>,

    /// Alternative addresses assigned to the node
    secondary_addresses: Option<Vec<NodeAddressingElement>>
}
