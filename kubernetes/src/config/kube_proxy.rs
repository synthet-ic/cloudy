//! Reference <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/>

use std::{
    collections::HashMap,
    time::Duration
};

use kfl::Decode;

// #[derive(Debug, Decode)]
// pub enum KubeProxy {
//     KubeProxyConfiguration(KubeProxyConfiguration)
// }

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration>
#[derive(Debug, Decode)]
pub struct KubeProxyConfiguration {
    feature_gates: HashMap<String, bool>,
    bind_address: String,
    healthz_bind_address: String,
    metrics_bind_address: String,
    bind_address_hard_fail: bool,
    enable_profiling: bool,
    cluster_cidr: String,
    hostname_override: String,
    client_connection: ClientConnection,
    iptables: IpTables,
    ipvs: IPVS,
    oom_score_adj: i32,
    mode: ProxyMode,
    port_range: String,
    udp_idle_timeout: Duration,
    conntrack: Conntrack,
    config_sync_period: Duration,
    node_port_addresses: Vec<String>,
    winkernel: Winkernel,
    show_hidden_metrics_for_version: String,
    detect_local_mode: LocalMode,
    detect_local: DetectLocal
}

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#ClientConnectionConfiguration>
#[derive(Debug, Decode)]
pub struct ClientConnection {
    kubeconfig: String,
    accept_content_types: String,
    content_type: String,
    qps: f32,
    burst: i32
}

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-KubeProxyIPTablesConfiguration>
#[derive(Debug, Decode)]
pub struct IpTables {
    masquerade_bit: i32,
    masquerade_all: bool,
    sync_period: Duration,
    min_sync_period: Duration
}

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-KubeProxyIPVSConfiguration>
#[derive(Debug, Decode)]
pub struct IPVS {
    sync_period: Duration,
    min_sync_period: Duration,
    scheduler: String,
    exclude_cidrs: Vec<String>,
    strict_arp: bool,
    tcp_timeout: Duration,
    tcp_fin_timeout: Duration,
    udp_timeout: Duration
}

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-ProxyMode>
#[derive(Debug, Decode)]
pub enum ProxyMode {
    // #[kfl(rename(serialize = "ipvs"))]
    IPVirtualServer
}

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConntrackConfiguration>
#[derive(Debug, Decode)]
pub struct Conntrack {
    max_per_core: i32,
    min: i32,
    tcp_established_timeout: Duration,
    tcp_close_wait_timeout: Duration
}

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-KubeProxyWinkernelConfiguration>
#[derive(Debug, Decode)]
pub struct Winkernel {
    network_name: String,
    source_vip: String,
    enable_dsr: bool,
    root_hns_endpoint_name: String,
    forward_health_check_vip: bool
}

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-LocalMode>
pub type LocalMode = String;

/// <https://kubernetes.io/docs/reference/config-api/kube-proxy-config.v1alpha1/#kubeproxy-config-k8s-io-v1alpha1-DetectLocalConfiguration>
#[derive(Debug, Decode)]
pub struct DetectLocal {
    bridge_interface: String,
    interface_name_prefix: String
}
