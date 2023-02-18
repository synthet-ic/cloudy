//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/>
//! - Concepts <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/>

use std::collections::HashMap;

use kfl::{Decode, DecodeScalar};

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Debug, Decode)]
pub struct Selector {
    /// List of label selector requirements. The requirements are ANDed.
    match_expressions: Vec<LabelSelectorRequirement>,
    /// Map of {key, value} pairs. A single {key, value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    match_labels: HashMap<String, String>
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Debug, Decode)]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    operator: Operator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    values: Vec<String>
}

#[derive(Debug, DecodeScalar)]
pub enum Operator {
    In,
    NotIn,
    Exists,
    DoesNotExist
}
