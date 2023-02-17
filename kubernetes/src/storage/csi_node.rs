/*!
Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-node-v1/>
*/

use kfl::Decode;

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-node-v1/#CsiNode>
#[derive(Debug, Decode)]
pub struct CsiNode {
    metadata: Metadata,
    spec: CsiNodeSpec
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-node-v1/#CsiNodeSpec>
#[derive(Debug, Decode)]
pub struct CsiNodeSpec {
    drivers: Vec<CsiNodeDriver>
}

#[derive(Debug, Decode)]
pub struct CsiNodeDriver {
    name: String,
    node_id: String,
    allocatable: Option<VolumeNodeResources>,
    topology_keys: Vec<String>
}

#[derive(Debug, Decode)]
pub struct VolumeNodeResources {
    count: Option<i32>
}
