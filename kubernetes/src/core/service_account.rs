//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/>

use kfl::Decode;

use crate::{
    core::{
        local_reference::LocalReference,
        reference::Reference
    },
    meta::metadata::Metadata,
};

/**
<https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount>

ServiceAccount binds together:

- a name, understood by users, and perhaps by peripheral systems, for an identity
- a principal that can be authenticated and authorised
- a set of secrets
*/
#[derive(Debug, Decode)]
pub struct ServiceAccount {
    metadata: Metadata,
    /**
    Indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level.
    */
    automount_service_account_token: Option<bool>,
    /**
    A list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. `image_pull_secrets` are distinct from Secrets because Secrets can be mounted in the pod, but `image_pull_secrets` are only accessed by the kubelet.
    
    More info: <https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod>
    */
    image_pull_secrets: Vec<LocalReference>,
    /**
    A list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use. Pods are only limited to this list if this service account has a `"kubernetes.io/enforce-mountable-secrets"` annotation set to `"true"`. This field should not be used to find auto-generated service account token secrets for use outside of pods. Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created.
    
    More info: <https://kubernetes.io/docs/concepts/configuration/secret>
    */
    secrets: Vec<Reference>
}
