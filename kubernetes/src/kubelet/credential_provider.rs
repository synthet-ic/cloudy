//! Reference <https://kubernetes.io/docs/reference/config-api/kubelet-credentialprovider.v1beta1/>

use std::{time::Duration, collections::HashMap};

use kfl::Decode;

/// <https://kubernetes.io/docs/reference/config-api/kubelet-credentialprovider.v1beta1/#credentialprovider-kubelet-k8s-io-v1beta1-CredentialProviderRequest>
#[derive(Debug, Decode)]
pub struct CredentialProviderRequest {
    image: Option<String>
}

/// <https://kubernetes.io/docs/reference/config-api/kubelet-credentialprovider.v1beta1/#credentialprovider-kubelet-k8s-io-v1beta1-CredentialProviderResponse>
#[derive(Debug, Decode)]
pub struct CredentialProviderResponse {
    cache_key_type: PluginCacheKeyType,
    cache_duration: Option<Duration>,
    auth: Option<HashMap<String, AuthConfig>>
}

#[derive(Debug, Decode)]
pub struct AuthConfig {
    username: String,
    password: String
}

type PluginCacheKeyType = String;
