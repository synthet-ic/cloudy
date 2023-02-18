//! - Concepts <https://kubernetes.io/docs/concepts/policy/limit-range/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/limit-range-v1/>

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    meta::metadata::Metadata,
    quantity::Quantity
};

/// LimitRange sets resource usage limits for each kind of resource in a Namespace.
/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/limit-range-v1/#LimitRange>
#[derive(Debug, Decode)]
pub struct LimitRange {
    metadata: Metadata,
    spec: LimitRangeSpec,
}

/// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/limit-range-v1/#LimitRangeSpec>
#[derive(Debug, Decode)]
pub struct LimitRangeSpec {
    /// Limits is the list of LimitRangeItem objects that are enforced.
    limits: Vec<LimitRangeItem>,
}

/// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
#[derive(Debug, Decode)]
pub struct LimitRangeItem {
    /// Type of resource that this limit applies to.
    r#type: String,
    /// Default resource requirement limit value by resource name if resource limit is omitted.
    default: HashMap<String, Quantity>,
    /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
    default_request: HashMap<String, Quantity>,
    /// Max usage constraints on this kind by resource name.
    max: HashMap<String, Quantity>,
    /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
    max_limit_request_ratio: HashMap<String, Quantity>,
    /// Min usage constraints on this kind by resource name.
    min: HashMap<String, Quantity>
}
