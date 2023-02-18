//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/validating-webhook-configuration-v1/>

use kfl::Decode;

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/validating-webhook-configuration-v1/#ValidatingWebhookConfiguration>
#[derive(Debug, Decode)]
pub struct ValidatingWebhookConfiguration {
    metadata: Metadata
}
