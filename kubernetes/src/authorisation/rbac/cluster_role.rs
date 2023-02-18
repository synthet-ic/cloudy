//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/cluster-role-v1/>

use kfl::Decode;

use crate::{
    authorisation::rbac::role::PolicyRule,
    meta::{LabelSelector, Metadata},
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/cluster-role-v1/#ClusterRole>
#[derive(Debug, Decode)]
pub struct ClusterRole {
    metadata: Metadata,
    aggregation_rule: Option<AggregationRule>,
    rules: Vec<PolicyRule>
}

#[derive(Debug, Decode)]
pub struct AggregationRule {
    cluster_role_selectors: Vec<LabelSelector>,
}
