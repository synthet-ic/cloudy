//! Reference <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/>

use std::{
    collections::HashMap,
    time::Duration
};

use kfl::Decode;

use crate::{
    core::node::Taint,
    time::Time
};

// #[derive(Debug, Decode)]
// pub enum Kubeadm {
//     ClusterConfiguration(ClusterConfiguration),
//     InitConfiguration(InitConfiguration),
//     JoinConfiguration(JoinConfiguration)
// }

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-ClusterConfiguration>
#[derive(Debug, Decode)]
pub struct ClusterConfiguration {
    etcd: Option<Etcd>,
    networking: Option<Networking>,
    kubernetes_version: Option<String>,
    control_plane_endpoint: Option<String>,
    api_server: Option<ApiServer>,
    controller_manager: Option<ControlPlaneComponent>,
    scheduler: Option<ControlPlaneComponent>,
    dns: Option<DNS>,
    certificates_dir: Option<String>,
    image_repository: Option<String>,
    feature_gates: Option<HashMap<String, bool>>,
    cluster_name: Option<String>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-Etcd>
#[derive(Debug, Decode)]
pub struct Etcd {
    local: Option<LocalEtcd>,
    external: Option<ExternalEtcd>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-LocalEtcd>
#[derive(Debug, Decode)]
pub struct LocalEtcd {
    image_meta: ImageMeta,
    data_dir: String,
    extra_args: HashMap<String, String>,
    server_cert_sans: Vec<String>,
    peer_cert_sans: Vec<String>,
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-ImageMeta>
#[derive(Debug, Decode)]
pub struct ImageMeta {
    image_repository: Option<String>,
    image_tag: Option<String>
}

#[derive(Debug, Decode)]
pub struct ExternalEtcd {
    endpoints: Vec<String>,
    ca_file: String,
    cert_file: String,
    key_file: String
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-Networking>
#[derive(Debug, Decode)]
pub struct Networking {
    service_subnet: Option<String>,
    pod_subnet: Option<String>,
    dns_domain: Option<String>,
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-ApiServer>
#[derive(Debug, Decode)]
pub struct ApiServer {
    control_plane_component: ControlPlaneComponent,
    cert_sans: Vec<String>,
    timeout_for_control_plane: Option<Duration>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-ControlPlaneComponent>
#[derive(Debug, Decode)]
pub struct ControlPlaneComponent {
    extra_args: HashMap<String, String>,
    extra_volumes: Vec<HostPathMount>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-HostPathMount>
#[derive(Debug, Decode)]
pub struct HostPathMount {
    name: String,
    host_path: String,
    mount_path: String,
    read_only: Option<bool>,
    path_type: Option<String>  // TODO
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-DNS>
#[derive(Debug, Decode)]
pub struct DNS {
    image_meta: ImageMeta
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-InitConfiguration>
#[derive(Debug, Decode)]
pub struct InitConfiguration {
    bootstrap_tokens: Vec<BootstrapToken>,
    node_registration: Option<NodeRegistrationOptions>,
    local_api_endpoint: Option<APIEndpoint>,
    certificate_key: Option<String>,
    skip_phases: Vec<String>,
    patches: Option<Patches>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#BootstrapToken>
#[derive(Debug, Decode)]
pub struct BootstrapToken {
    token: BootstrapTokenString,
    description: Option<String>,
    ttl: Option<Duration>,
    expires: Option<Time>,
    usages: Vec<String>,
    groups: Vec<String>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#BootstrapTokenString>
#[derive(Debug, Decode)]
pub struct BootstrapTokenString {
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-NodeRegistrationOptions>
#[derive(Debug, Decode)]
pub struct NodeRegistrationOptions {
    name: Option<String>,
    cri_socket: Option<String>,
    taints: Vec<Taint>,
    kubelet_extra_args: HashMap<String, String>,
    ignore_preflight_errors: Vec<String>,
    image_pull_policy: Option<PullPolicy>
}

#[derive(Debug, Decode, Default)]
pub enum PullPolicy {
    Always,
    #[default]
    IfNotPresent,
    Never
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-APIEndpoint>
#[derive(Debug, Decode)]
pub struct APIEndpoint {
    advertise_address: Option<String>,
    bind_port: Option<i32>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-Patches>
#[derive(Debug, Decode)]
pub struct Patches {
    directory: Option<String>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-JoinConfiguration>
#[derive(Debug, Decode)]
pub struct JoinConfiguration {
    node_registration: Option<NodeRegistrationOptions>,
    ca_cert_path: Option<String>,
    discovery: Discovery,
    control_plane: Option<JoinControlPlane>,
    skip_phases: Vec<String>,
    patches: Option<Patches>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-Discovery>
#[derive(Debug, Decode)]
pub struct Discovery {
    bootstrap_token: Option<BootstrapTokenDiscovery>,
    file: Option<FileDiscovery>,
    tls_bootstrap_token: Option<String>,
    timeout: Option<Duration>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-BootstrapTokenDiscovery>
#[derive(Debug, Decode)]
pub struct BootstrapTokenDiscovery {
    token: String,
    api_server_endpoint: Option<String>,
    ca_cert_hashes: Vec<String>,
    unsafe_skip_ca_verification: Option<bool>
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-FileDiscovery>
#[derive(Debug, Decode)]
pub struct FileDiscovery {
    kube_config_path: String
}

/// <https://kubernetes.io/docs/reference/config-api/kubeadm-config.v1beta3/#kubeadm-k8s-io-v1beta3-JoinControlPlane>
#[derive(Debug, Decode)]
pub struct JoinControlPlane {
    local_api_endpoint: Option<APIEndpoint>,
    certificate_key: Option<String>
}
