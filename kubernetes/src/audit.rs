/*!
Reference <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/>
*/

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    meta::{
        metadata::Metadata,
        status::Status
    },
    time::MicroTime
};

// #[derive(Debug, Decode)]
// pub enum Audit {
//     Event(Event),
//     Policy(Policy)
// }

/// <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Event>
#[derive(Debug, Decode)]
pub struct Event {
    level: Level,
    // #[kfl(rename(serialize = "auditID"))]
    audit_id: String,
    stage: Stage,
    // #[kfl(rename(serialize = "requestURI"))]
    request_uri: String,
    verb: String,
    user: UserInfo,
    impersonated_user: UserInfo,
    // #[kfl(rename(serialize = "sourceIPs"))]
    source_ips: Vec<String>,
    user_agent: Option<String>,
    object_ref: Option<ObjectReference>,
    response_status: Option<Status>,
    request_object: Option<String>,
    response_pbject: Option<String>,
    request_received_timestamp: Option<MicroTime>,
    stage_timestamp: Option<MicroTime>,
    annotations: HashMap<String, String>,
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Level>
pub type Level = String;

/// <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Stage>
pub type Stage = String;

/// <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-ObjectReference>
#[derive(Debug, Decode)]
pub struct ObjectReference {
    resource: Option<String>,
    namespace: Option<String>,
    name: Option<String>,
    uid: Option<String>,
    api_group: Option<String>,
    api_version: Option<String>,
    resource_version: Option<String>,
    subresource: Option<String>,
}

/// <https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.25/#userinfo-v1-authentication-k8s-io>
#[derive(Debug, Decode)]
pub struct UserInfo {
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Policy>
#[derive(Debug, Decode)]
pub struct Policy {
    metadata: Metadata,
    rules: Vec<PolicyRule>,
    omit_stages: Vec<Stage>,
    omit_managed_fields: Option<bool>
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-PolicyRule>
#[derive(Debug, Decode)]
pub struct PolicyRule {
    level: Level,
    users: Vec<String>,
    user_groups: Vec<String>,
    verbs: Vec<String>,
    resources: Vec<GroupResources>,
    namespaces: Vec<String>,
    #[kfl(rename(serialize = "nonResourceURLs"))]
    non_resource_urls: Vec<String>,
    omit_stages: Vec<Stage>,
    omit_managed_fields: Option<bool>
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-GroupResources>
#[derive(Debug, Decode)]
pub struct GroupResources {
    group: String,
    resources: Vec<String>,
    resource_names: Vec<String>
}
