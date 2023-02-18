//! Reference <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/>

use std::time::Duration;

use kfl::Decode;

/// <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/#apiserver-config-k8s-io-v1-EncryptionConfiguration>
#[derive(Debug, Decode)]
pub struct EncryptionConfiguration {
    resources: Vec<Resource>
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/#apiserver-config-k8s-io-v1-Resource>
#[derive(Debug, Decode)]
pub struct Resource {
    resources:  Vec<String>,
    providers: Vec<Provider>
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/#apiserver-config-k8s-io-v1-ProviderConfiguration>
#[derive(Debug, Decode)]
pub struct Provider {
    aesgcm: Aes,
    aescbc: Aes,
    secretbox: Secretbox,
    identity: Identity,
    kms: Kms
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/#apiserver-config-k8s-io-v1-Aes>
#[derive(Debug, Decode)]
pub struct Aes {
    keys: Vec<Key>
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/#apiserver-config-k8s-io-v1-Key>
#[derive(Debug, Decode)]
pub struct Key {
    name: String,
    secret: String,
}

#[derive(Debug, Decode)]
pub struct Secretbox {
    keys: Vec<Key>
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/#apiserver-config-k8s-io-v1-Identity>
#[derive(Debug, Decode)]
pub struct Identity {}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-encryption.v1/#apiserver-config-k8s-io-v1-Kms>
#[derive(Debug, Decode)]
pub struct Kms {
    name: String,
    cache_size: Option<i32>,
    endpoint: String,
    timeout: Option<Duration>
}
