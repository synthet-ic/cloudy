//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/binding-v1/>

use kfl::Decode;

use crate::{
    core::reference::Reference,
    meta::metadata::Metadata,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/binding-v1/#Binding>
#[derive(Debug, Decode)]
pub struct Binding {
    metadata: Option<Metadata>,
    target: Reference
}
