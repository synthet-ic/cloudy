//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-driver-v1/>

use kfl::Decode;

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-driver-v1/#CsiDriver>
#[derive(Debug, Decode)]
pub struct CsiDriver {
    metadata: Metadata,
    spec: CsiDriverSpec
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-driver-v1/#CsiDriverSpec>
#[derive(Debug, Decode)]
pub struct CsiDriverSpec {
    attach_required: Option<bool>,
    fs_group_policy: Option<FSGroupPolicy>
}

#[derive(Debug, Decode)]
pub enum FSGroupPolicy {}
