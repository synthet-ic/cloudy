//! - Getting Started <https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/>
//! - Tasks <https://kubernetes.io/docs/tasks/administer-cluster/kubelet-config-file/>
//! - Reference <https://kubernetes.io/docs/reference/config-api/kubelet-config.v1beta1/>

use std::{
    collections::HashMap,
    net::IpAddr,
    path::PathBuf,
    time::Duration,
};

use kfl::Decode;

#[derive(Debug, Decode)]
pub struct CredentialProviderConfig {
}

/// <https://kubernetes.io/docs/reference/config-api/kubelet-config.v1beta1/#kubelet-config-k8s-io-v1beta1-KubeletConfiguration>
#[derive(Debug, Decode)]
pub struct KubeletConfiguration {
    enable_server: bool,
    static_pod_path: Option<PathBuf>,
    sync_frequency: Option<Duration>,
    file_check_frequency: Option<Duration>,
    http_check_frequency: Option<Duration>,
    // #[kfl(rename(serialize = "staticPodURL"))]
    static_pod_url: Option<String>,
    // #[kfl(rename(serialize = "staticPodURLHeader"))]
    static_pod_url_header: HashMap<String, String>,
    address: Option<IpAddr>,
    port: Option<i32>,
    read_only_port: Option<i32>,
    tls_cert_file: Option<String>,
    tls_private_key_file: Option<String>,
    tls_cipher_suites: Vec<String>,
    tls_min_version: Option<String>,
    rotate_certificates: Option<bool>,
    // #[kfl(rename(serialize = "serverTLSBootstrap"))]
    server_tls_bootstrap: Option<bool>,
    authentication: Option<KubeletAuthentication>,
    // #[kfl(rename(serialize = "authorization"))]
    authorisation: Option<KubeletAuthorisation>,
    // #[kfl(rename(serialize = "registryPullQPS"))]
    registry_pull_qps: Option<i32>,
    registry_burst: Option<i32>,
    // #[kfl(rename(serialize = "eventRecordQPS"))]
    event_record_qps: Option<i32>,
    event_burst: Option<i32>,
    enable_debugging_handlers: Option<bool>,
    enable_contention_profiling: Option<bool>,
    healthz_port: Option<i32>,
    healthz_bind_address: Option<String>,
    oom_score_adj: Option<i32>,
    cluster_domain: Option<String>,
    // #[kfl(rename(serialize = "clusterDNS"))]
    cluster_dns: Vec<String>,
    streaming_connection_idle_timeout: Option<Duration>,
    node_status_update_frequency: Option<Duration>,
    node_status_report_frequency: Option<Duration>,
    node_lease_duration_seconds: Option<i32>,
    // #[kfl(rename(serialize = "imageMinimumGCAge"))]
    image_minimum_gc_age: Option<Duration>,
    // #[kfl(rename(serialize = "imageGCHighThresholdPercent"))]
    image_gc_high_threshold_percent: Option<i32>,
    // #[kfl(rename(serialize = "imageGCLowThresholdPercent"))]
    image_gc_low_threshold_percent: Option<i32>,
    volume_stats_agg_period: Option<Duration>,
    kubelet_cgroups: Option<String>,
    system_cgroups: Option<String>,
    cgroup_root: Option<String>,
    // #[kfl(rename(serialize = "cgroupsPerQOS"))]
    cgroups_per_qos: Option<bool>,
    cgroup_driver: Option<String>,
    cpu_manager_policy: Option<String>,
    cpu_manager_policy_options: HashMap<String, String>,
    cpu_manager_reconcile_period: Option<Duration>,
    memory_manager_policy: Option<String>,
    topology_manager_policy: Option<String>,
    topology_manager_scope: Option<String>,
    qos_reserved: HashMap<String, String>,
    runtime_request_timeout: Option<Duration>,
    hairpin_mode: Option<String>,
    max_pods: Option<i32>,
    // #[kfl(rename(serialize = "podCIDR"))]
    pod_cidr: Option<String>,
    pod_pids_limit: Option<i64>,
    resolv_conf: Option<String>,
    run_once: Option<String>,
    // #[kfl(rename(serialize = "cpuCFSQuota"))]
    cpu_cfs_quota: Option<bool>,
    // #[kfl(rename(serialize = "cpuCFSQuota"))]
    cpu_cfs_quota_period: Option<Duration>,
    node_status_max_images: Option<i32>,
    max_open_files: Option<i64>,
    content_type: Option<String>,
    // #[kfl(rename(serialize = "cpuCFSQuota"))]
    kube_api_qps: Option<i32>,
    // #[kfl(rename(serialize = "kubeAPIBurst"))]
    kube_api_burst: Option<i32>,
    // #[kfl(rename(serialize = "serializeImagePulls"))]
    serialise_image_pulls: Option<bool>,
    eviction_hard: HashMap<String, String>,
    eviction_soft: HashMap<String, String>,
    eviction_soft_grace_period: HashMap<String, String>,
    eviction_pressure_transition_period: Option<Duration>,
    eviction_max_pod_grace_period: Option<i32>
}

/// <https://kubernetes.io/docs/reference/config-api/kubelet-config.v1beta1/#kubelet-config-k8s-io-v1beta1-KubeletAuthentication>
#[derive(Debug, Decode)]
pub struct KubeletAuthentication {
}

/// <https://kubernetes.io/docs/reference/config-api/kubelet-config.v1beta1/#kubelet-config-k8s-io-v1beta1-KubeletAuthorization>
#[derive(Debug, Decode)]
pub struct KubeletAuthorisation {

}

/// <https://kubernetes.io/docs/reference/config-api/kubelet-config.v1beta1/#kubelet-config-k8s-io-v1beta1-SerializedNodeConfigSource>
#[derive(Debug, Decode)]
pub struct SerialisedNodeConfigSource {
}
