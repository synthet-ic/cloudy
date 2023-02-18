//! - Concepts <https://kubernetes.io/docs/concepts/scheduling-eviction/kube-scheduler/>
//! - Reference
//!   - <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/>
//!   - <https://kubernetes.io/docs/reference/scheduling/config/>

use std::time::Duration;

use kfl::Decode;

use crate::core::pod::NodeAffinity;

// #[derive(Debug, Decode)]
// pub enum KubeScheduler {
//     KubeSchedulerConfiguration(KubeSchedulerConfiguration)
// }

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-DefaultPreemptionArgs>
#[derive(Debug, Decode)]
pub struct DefaultPreemptionArgs {
    min_candidate_nodes_percentage: i32,
    min_candidate_nodes_absolute: i32
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-InterPodAffinityArgs>
#[derive(Debug, Decode)]
pub struct InterPodAffinityArgs {
    hard_pod_affinity_weight: i32
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-KubeSchedulerConfiguration>
#[derive(Debug, Decode)]
pub struct KubeSchedulerConfiguration {
    parallelism: i32,
    leader_election: LeaderElection,
    client_connection: ClientConnection,
    debugging: Debugging,
    percentage_of_nodes_to_score: i32,
    pod_initial_backoff_seconds: i64,
    pod_max_backoff_seconds: i64,
    profiles: Vec<Profile>,
    extenders: Vec<Extender>
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#LeaderElectionConfiguration>
#[derive(Debug, Decode)]
pub struct LeaderElection {
    leader_elect: bool,
    lease_duration: Duration,
    renew_deadline: Duration,
    retry_period: Duration,
    resource_lock: String,
    resource_name: String,
    resource_namespace: String
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#ClientConnectionConfiguration>
#[derive(Debug, Decode)]
pub struct ClientConnection {
    kubeconfig: String,
    accept_content_types: String,
    content_type: String,
    qps: f32,
    burst: i32
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#DebuggingConfiguration>
#[derive(Debug, Decode)]
pub struct Debugging {
    enable_profiling: bool,
    enable_contention_profiling: bool
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-KubeSchedulerProfile>
#[derive(Debug, Decode)]
pub struct Profile {
    scheduler_name: String,
    plugins: Plugins,
    plugin_config: Vec<PluginConfig>
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-Plugins>
#[derive(Debug, Decode)]
pub struct Plugins {
    queue_sort: PluginSet,
    pre_filter: PluginSet,
    filter: PluginSet,
    post_filter: PluginSet,
    pre_score: PluginSet,
    score: PluginSet,
    reserve: PluginSet,
    permit: PluginSet,
    pre_bind: PluginSet,
    bind: PluginSet,
    post_bind: PluginSet,
    multi_point: PluginSet
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-PluginSet>
#[derive(Debug, Decode)]
pub struct PluginSet {
    enabled: Vec<Plugin>,
    disabled: Vec<Plugin>
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-Plugin>
#[derive(Debug, Decode)]
pub struct Plugin {
    name: String,
    weight: i32
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-PluginConfig>
#[derive(Debug, Decode)]
pub struct PluginConfig {
    name: String,
    args: Vec<u8>
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-Extender>
#[derive(Debug, Decode)]
pub struct Extender {
    url_prefix: String,
    filter_verb: String,
    preempt_verb: String,
    // prioritize_verb
    prioritise_verb: String,
    weight: i64,
    bind_verb: String,
    enable_https: bool,
    tls_config: ExtenderTLSConfig,
    http_timeout: Duration,
    node_cache_capable: bool,
    managed_resources: Vec<ExtenderManagedResource>,
    ignorable: bool
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-ExtenderTLSConfig>
#[derive(Debug, Decode)]
pub struct ExtenderTLSConfig {
    insecure: bool,
    server_name: String,
    cert_file: String,
    key_file: String,
    ca_file: String,
    cert_data: Vec<u8>,
    key_data: Vec<u8>,
    ca_data: Vec<u8>,
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-ExtenderManagedResource>
#[derive(Debug, Decode)]
pub struct ExtenderManagedResource {
    name: String,
    ignored_by_scheduler: bool
}

/// <https://kubernetes.io/docs/reference/config-api/kube-scheduler-config.v1/#kubescheduler-config-k8s-io-v1-NodeAffinityArgs>
#[derive(Debug, Decode)]
pub struct NodeAffinityArgs {
    added_affinity: Option<NodeAffinity>,
}
