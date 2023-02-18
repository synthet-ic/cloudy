//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-v1/>

use kfl::Decode;

use crate::{
    meta::metadata::Metadata,
    time::MicroTime
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-v1/#Lease>
#[derive(Debug, Decode)]
pub struct Lease {
    metadata: Option<Metadata>,
    spec: Spec
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-v1/#Spec>
#[derive(Debug, Decode)]
pub struct Spec {
    acquire_time: Option<MicroTime>,
    holder_identity: Option<String>,
    lease_duration_seconds: Option<i32>,
    lease_transitions: Option<i32>,
    renew_time: Option<MicroTime>
}
