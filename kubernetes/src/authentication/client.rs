//! Reference <https://kubernetes.io/docs/reference/config-api/client-authentication.v1/>

use kfl::Decode;

use crate::time::Time;

// #[derive(Debug, Decode)]
// pub enum Client {
//     ExecCredential(ExecCredential)
// }

/// <https://kubernetes.io/docs/reference/config-api/client-authentication.v1/#client-authentication-k8s-io-v1-ExecCredential>
#[derive(Debug, Decode)]
pub struct ExecCredential {
    spec: ExecCredentialSpec,
    status: Option<ExecCredentialStatus>
}

#[derive(Debug, Decode)]
pub struct ExecCredentialSpec {
    cluster: Option<Cluster>,
    interactive: bool
}

#[derive(Debug, Decode)]
pub struct Cluster {
    server: String,
    tls_server_name: Option<String>,
    insecure_skip_tls_verify: Option<bool>,
    certificate_authority_data: Vec<u8>,
    proxy_url: Option<String>,
    config: Option<String>
}

#[derive(Debug, Decode)]
pub struct ExecCredentialStatus {
    expiration_timestamp: Option<Time>,
    token: String,
    client_certificate_data: String,
    client_key_data: String
}
