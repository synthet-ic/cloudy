//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume-attachment-v1/>

use kfl::Decode;

use crate::{
    core::persistent_volume::PersistentVolumeSpec,
    meta::metadata::Metadata,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume-attachment-v1/#VolumeAttachment>
#[derive(Debug, Decode)]
pub struct VolumeAttachment {
    metadata: Metadata,
    spec: VolumeAttachmentSpec,
    status: Option<VolumeAttachmentStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume-attachment-v1/#VolumeAttachmentSpec>
#[derive(Debug, Decode)]
pub struct VolumeAttachmentSpec {
    attacher: String,
    node_name: String,
    source: VolumeAttachmentSource
}

#[derive(Debug, Decode)]
pub struct VolumeAttachmentSource {
    inline_volume_spec: Option<PersistentVolumeSpec>,
    persistent_volume_name: Option<String>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume-attachment-v1/#VolumeAttachmentStatus>
#[derive(Debug, Decode)]
pub struct VolumeAttachmentStatus {
    
}
