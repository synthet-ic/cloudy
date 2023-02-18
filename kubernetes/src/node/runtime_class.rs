/*!
Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/runtime-class-v1/>
*/

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    core::pod::Toleration,
    meta::metadata::Metadata,
    quantity::Quantity
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/runtime-class-v1/#RuntimeClass>
#[derive(Debug, Decode)]
pub struct RuntimeClass {
    metadata: Option<Metadata>,
    handler: String,
    overhead: Option<Overhead>,
    scheduling: Option<Scheduling>
}

#[derive(Debug, Decode)]
pub struct Overhead {
    pod_fixed: HashMap<String, Quantity>
}

#[derive(Debug, Decode)]
pub struct Scheduling {
    node_selector: HashMap<String, String>,
    tolerations: Vec<Toleration>
}
