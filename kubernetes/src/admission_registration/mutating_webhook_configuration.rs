//! References <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/mutating-webhook-configuration-v1/>

use std::path::PathBuf;

use kfl::Decode;

use crate::meta::{
    label_selector::LabelSelector,
    metadata::Metadata,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/mutating-webhook-configuration-v1/>
#[derive(Debug, Decode)]
pub struct MutatingWebhookConfiguration {
    metadata: Metadata,
    #[kfl(children)]
    webhooks: Vec<MutatingWebhook>
}

#[derive(Debug, Decode)]
pub struct MutatingWebhook {
    admission_review_versions: Vec<String>,
    client_config: WebhookClientConfig,
    name: String,
    side_effects: String,
    failure_policy: Option<String>,
    match_policy: Option<String>,
    namespace_selector: Option<LabelSelector>,
    object_selector: Option<LabelSelector>,
    reinvocation_policy: Option<String>,
    rules: Vec<RuleWithOperations>,
    timeout_seconds: Option<i32>
}

#[derive(Debug, Decode)]
pub struct WebhookClientConfig {
    ca_bundle: Vec<u8>,
    service: Option<ServiceReference>,
    url: Option<String>
}

#[derive(Debug, Decode)]
pub struct ServiceReference {
    name: String,
    namespace: String,
    path: Option<PathBuf>,
    port: Option<i32>
}

#[derive(Debug, Decode)]
pub struct RuleWithOperations {
    api_groups: Vec<String>
}
