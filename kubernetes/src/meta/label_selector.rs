//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/>
//! - Concepts <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/>

use std::collections::HashMap;

use kfl::{Decode, DecodeScalar};

#[derive(Debug, Decode)]
pub struct LabelSelector {
    match_expressions: Vec<LabelSelectorRequirement>,
    match_labels: HashMap<String, String>
}

#[derive(Debug, Decode)]
pub struct LabelSelectorRequirement {
    key: String,
    operator: Operator,
    values: Vec<String>
}

#[derive(Debug, DecodeScalar)]
pub enum Operator {
    NotIn,
    Exists,
    DoesNotExist
}
