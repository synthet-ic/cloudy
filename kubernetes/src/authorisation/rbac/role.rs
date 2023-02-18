//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/role-v1/>

use kfl::Decode;

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/role-v1/#Role>
#[derive(Debug, Decode)]
pub struct Role {
    metadata: Metadata,
    rules: Vec<Rule>
}

#[derive(Debug, Decode)]
pub struct Rule {
    api_groups: Vec<String>,
    resources: Vec<String>,
    verbs: Vec<String>,
    resource_names: Vec<String>,
    non_resource_urls: Vec<String>,
}
