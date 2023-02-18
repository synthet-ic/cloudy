//! - Concepts <https://kubernetes.io/docs/concepts/storage/persistent-volumes/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-v1/>

use std::{
    collections::HashMap,
    path::PathBuf
};

use kfl::Decode;

use crate::{
    core::Reference,
    meta::Metadata,
    node_selector::NodeSelector,
    quantity::Quantity,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-v1/#PersistentVolume>
#[derive(Debug, Decode)]
pub struct PersistentVolume {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-v1/#Spec>
#[derive(Debug, Decode)]
pub struct Spec {
    /// `access_modes` contains all ways the volume can be mounted.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes>
    access_modes: Vec<AccessMode>,
    /// Description of the persistent volume's resources and capacity.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#capacity>
    capacity: HashMap<String, Quantity>,
    /// Part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-null when bound. [`claim.volume_name`][crate::core::persistent_volume_claim::PersistentVolumeClaimSpec::volume_name] is the authoritative bind between PV and PVC.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#binding>
    claim_ref: Option<Reference>,
    /// The list of mount options, e.g. ["ro", "soft"]. Not validated - mount will simply fail if one is invalid.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options>
    mount_options: Vec<MountOption>,
    /// `node_affinity` defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.
    node_affinity: Option<NodeAffinity>,
    persistent_volume_reclaim_policy: Option<ReclaimPolicy>,
    storage_class_name: Option<String>,
    volume_mode: Option<VolumeMode>,
    // Local
    // <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-v1/#local>
    host_path: Option<HostPath>,
    local: Option<Local>,

    /*
    Persistent Volumes
    <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-v1/#persistent-volumes>
    */

    // aws_elastic_block_store: Option<AWSElasticBlockStoreVolumeSource>,
    // azure_disc: Option<AzureDiscVolumeSource>,
    // azure_file: Option<AzureFilePersistentVolumeSource>,
    // cephfs: Option<CephFSPersistentVolumeSource>,
    // cinder: Option<CinderPersistentVolumeSource>,
    // csi: Option<CSIPersistentVolumeSource>,
    // fc: Option<FCVolumeSource>,
    // flex_volume: Option<FlexPersistentVolumeSource>,
    // flocker: Option<FlockerVolumeSource>,
    // gce_persistent_disc: Option<GCEPersistentDiscVolumeSource>,
    // glusterfs: Option<GlusterfsPersistentVolumeSource>,
    // iscsi: Option<ISCSIPersistentVolumeSource>,
    // nfs: Option<NFSVolumeSource>,
    // photon_persistent_disc: Option<PhotonPersistentDiscVolumeSource>,
    // portworx_volume: Option<PortworxVolumeSource>,
    // quobyte: Option<QuobyteVolumeSource>,
    // rbd: Option<RBDPersistentVolumeSource>,
    // scale_io: Option<ScaleIOPersistentVolumeSource>,
    // storageos: Option<StorageOSPersistentVolumeSource>,
    // vsphere_volume: Option<VsphereVirtualDiscVolumeSource>
}

/// <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes>
#[derive(Debug, DecodeScalar)]
pub enum AccessMode {
    ReadWriteOnce,
    ReadOnlyMany,
    ReadWriteMany,
    ReadWriteOncePod
}

/// <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options>
#[derive(Debug, DecodeScalar)]
pub enum MountOption {
    Ro,
    Soft
}

#[derive(Debug, Decode)]
pub struct NodeAffinity {
    required: NodeSelector,
}

/// <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#reclaiming>
#[derive(Debug, Decode, Default)]
pub enum ReclaimPolicy {
    Retain,
    #[default]
    Delete
}

/// <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#volume-mode>
#[derive(Debug, DecodeScalar, Default)]
pub enum VolumeMode {
    #[default]
    Filesystem,
    Block
}

#[derive(Debug, Decode)]
pub struct HostPath {
    path: PathBuf,
    r#type: Option<String>
}

#[derive(Debug, Decode)]
pub struct Local {
    path: String,
    fs_type: Option<String>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-v1/#Status>
#[derive(Debug, Decode)]
pub struct Status {
    message: Option<String>,
    phase: Option<String>,
    reason: Option<String>
}
