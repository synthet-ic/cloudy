//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/namespace-v1/>

use kfl::Decode;

use crate::meta::{condition::Condition, metadata::Metadata};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/namespace-v1/#Namespace>
#[derive(Debug, Decode)]
pub struct Namespace {
    metadata: Metadata,
    spec: Option<Spec>,
    status: Option<Status>
} 

#[derive(Debug, Decode)]
pub struct Spec {
    finalisers: Vec<String>
}

#[derive(Debug, Decode)]
pub struct Status {
    conditions: Vec<Condition>
}
