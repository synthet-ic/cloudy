//! Concepts <https://kubernetes.io/docs/concepts/storage/storage-classes/>
//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/>

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    core::persistent_volume::{MountOption, ReclaimPolicy},
    meta::metadata::Metadata
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass>
#[derive(Debug, Decode)]
pub struct StorageClass {
    metadata: Option<Metadata>,
    provisioner: String,
    allow_volume_expansion: Option<bool>,
    allowed_topologies: Vec<TopologySelectorTerm>,
    mount_options: Vec<MountOption>,
    parameters: HashMap<String, String>,
    reclaim_policy: Option<ReclaimPolicy>,
    volume_binding_mode: Option<VolumeBindingMode>
}

#[derive(Debug, Decode)]
pub struct TopologySelectorTerm {
    match_label_expressions: Vec<TopologySelectorLabelRequirement>
}

#[derive(Debug, Decode)]
pub struct TopologySelectorLabelRequirement {
    key: String,
    values: Vec<String>
}

#[derive(Debug, Decode, Default)]
pub enum VolumeBindingMode {
    #[default]
    VolumeBindingImmediate
}
