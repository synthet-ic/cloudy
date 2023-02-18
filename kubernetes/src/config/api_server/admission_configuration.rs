//! Reference
//! - <https://kubernetes.io/docs/reference/config-api/apiserver-config.v1/>
//! - <https://kubernetes.io/docs/reference/command-line-tools-reference/kube-apiserver/>

use std::path::PathBuf;

use kfl::Decode;

/// <https://kubernetes.io/docs/reference/config-api/apiserver-config.v1/#apiserver-config-k8s-io-v1-AdmissionConfiguration>
#[derive(Debug, Decode)]
pub struct AdmissionConfiguration {
    plugins: Option<Plugin>
}

/// <https://kubernetes.io/docs/reference/config-api/apiserver-config.v1/#apiserver-config-k8s-io-v1-Plugin>
#[derive(Debug, Decode)]
pub struct Plugin {
    name: String,
    path: Option<PathBuf>,
    configuration: Option<String>
}
