//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/>

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    core::{
        persistent_volume::{AccessMode, VolumeMode},
        typed_local_reference::TypedLocalReference,
    },
    meta::{
        condition::Condition,
        label_selector::LabelSelector,
        metadata::Metadata
    },
    quantity::Quantity,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#PersistentVolumeClaim>
#[derive(Debug, Decode)]
pub struct PersistentVolumeClaim {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#Spec>
#[derive(Debug, Decode)]
pub struct Spec {
    access_modes: Vec<AccessMode>,
    selector: Option<LabelSelector>,
    resources: Option<Resource>,
    volume_name: Option<String>,
    storage_class_name: Option<String>,
    volume_mode: Option<VolumeMode>,
    // Beta Level
    /// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#beta-level>
    data_source: Option<TypedLocalReference>,
    data_source_ref: Option<TypedLocalReference>,
}

#[derive(Debug, Decode)]
pub struct Resource {
    limits: HashMap<String, Quantity>,
    requests: HashMap<String, Quantity>,
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#Status>
#[derive(Debug, Decode)]
pub struct Status {
    access_modes: Vec<AccessMode>,
    allocated_resources: HashMap<String, Quantity>,
    capacity: HashMap<String, Quantity>,
    conditions: Vec<Condition>,
    phase: Option<String>,
    resize_status: Option<String>
}
