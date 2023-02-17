//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-storage-capacity-v1/>

use kfl::Decode;

use crate::{
    meta::{
        label_selector::LabelSelector,
        metadata::Metadata,
    },
    quantity::Quantity
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-storage-capacity-v1/#CsiStorageCapacity>
#[derive(Debug, Decode)]
pub struct CsiStorageCapacity {
    metadata: Metadata,
    storage_class_name: String,
    capacity: Option<Quantity>,
    maximum_volume_size: Option<Quantity>,
    node_topology: Option<LabelSelector>
}
