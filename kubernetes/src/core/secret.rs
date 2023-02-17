//! Concepts
//! - <https://kubernetes.io/docs/concepts/configuration/secret/>
//! - <https://kubernetes.io/docs/concepts/security/secrets-good-practices/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/>

use std::collections::HashMap;
use kfl::Decode;

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/#Secret>
#[derive(Debug, Decode)]
pub struct Secret {
    metadata: Metadata,
    data: Option<HashMap<String, Vec<u8>>>,
    immutable: Option<bool>,
    string_data: HashMap<String, String>,
    r#type: Option<SecretType>
}

/// <https://kubernetes.io/docs/concepts/configuration/secret/#secret-types>
#[derive(Debug, Decode)]
pub enum SecretType {
    Opaque,
    // #[kfl(rename(serialize = "kubernetes.io/service-account-token"))]
    ServiceAccountToken,
    // #[kfl(rename(serialize = "kubernetes.io/basic-auth"))]
    BasicAuth,
    // #[kfl(rename(serialize = "kubernetes.io/ssh-auth"))]
    SshAuth,
    // #[kfl(rename(serialize = "kubernetes.io/tls"))]
    Tls
}
