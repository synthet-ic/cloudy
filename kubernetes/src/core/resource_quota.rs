/*!
- Concepts <https://kubernetes.io/docs/concepts/policy/resource-quotas/>
- References <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/resource-quota-v1/>
*/

use std::collections::HashMap;

use kfl::{Decode, DecodeScalar};

use crate::{
   meta::metadata::Metadata,
   quantity::Quantity
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/resource-quota-v1/#ResourceQuota>
#[derive(Debug, Decode)]
pub struct ResourceQuota {
   metadata: Metadata,
   spec: ResourceQuotaSpec,
   status: Option<ResourceQuotaStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/resource-quota-v1/#ResourceQuotaSpec>
#[derive(Debug, Decode)]
pub struct ResourceQuotaSpec {
   hard: HashMap<String, Quantity>,
   scope_selector: Option<ScopeSelector>,
   scopes: Vec<String>
}

#[derive(Debug, Decode)]
pub struct ScopeSelector {
   match_expressions: Vec<ScopedResourceSelectorRequirement>
}

#[derive(Debug, Decode)]
pub struct ScopedResourceSelectorRequirement {
   operator: ScopedResourceSelectorRequirementOperator,
   scope_name: String,
   values: Vec<String>
}

#[derive(Debug, DecodeScalar)]
pub enum ScopedResourceSelectorRequirementOperator {
   In,
   NotIn,
   Exists,
   DoesNotExist
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/resource-quota-v1/#ResourceQuotaStatus>
#[derive(Debug, Decode)]
pub struct ResourceQuotaStatus {
   hard: HashMap<String, Quantity>,
   used: HashMap<String, Quantity>
}
