//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/node-selector-requirement/>

use kfl::Decode;

#[derive(Debug, Decode)]
pub struct NodeSelectorRequirement {
    key: String,
    operator: String,
    values: Vec<String>
}
