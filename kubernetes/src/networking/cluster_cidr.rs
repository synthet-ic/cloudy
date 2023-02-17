//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/cluster-cidr-v1alpha1/>

use kfl::Decode;

use crate::{
    meta::metadata::Metadata,
    node_selector::NodeSelector
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/cluster-cidr-v1alpha1/#ClusterCidr>
#[derive(Debug, Decode)]
pub struct ClusterCidr {
    metadata: Option<Metadata>,
    spec: Option<ClusterCidrSpec>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/cluster-cidr-v1alpha1/#ClusterCidrSpec>
#[derive(Debug, Decode)]
pub struct ClusterCidrSpec {
    per_node_host_bits: i32,
    ipv4: Option<String>,
    ipv6: Option<String>,
    node_selector: Option<NodeSelector>,
}
