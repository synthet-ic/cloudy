//! - Concepts <https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/daemon-set-v1/>

use kfl::Decode;

use crate::{
    core::pod_template::PodTemplateSpec,
    meta::{
        condition::Condition,
        label_selector::Selector,
        metadata::Metadata
    },
    IntOrString
};

/// DaemonSet represents the configuration of a daemon set.
///
/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/daemon-set-v1/#DaemonSet>
#[derive(Debug, Decode)]
pub struct DaemonSet {
    metadata: Option<Metadata>,
    spec: Spec,
    #[kfl(child)]
    status: Option<DaemonSetStatus>
}

/// Spec is the specification of a daemon set.
///
/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/daemon-set-v1/#DaemonSetSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    /// A label query over pods that are managed by the daemon set. Must match in order to be controlled. It must match the pod template's labels. More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors>.
    selector: Selector,
    /// An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template's node selector (or on every node if no node selector is specified). More info: <https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template>.
    template: PodTemplateSpec,
    /// Minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
    min_ready_seconds: Option<i32>,
    /// Update strategy to replace existing DaemonSet pods with new pods.
    update_strategy: Option<UpdateStrategy>,
    /// Number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
    #[kfl(default = 10)]
    revision_history_limit: u16
}

/// UpdateStrategy is a struct used to control the update strategy for a DaemonSet.
#[derive(Debug, Decode)]
pub struct UpdateStrategy {
    #[kfl(default)]
    r#type: update_strategy::Type,
    rolling_update: Option<RollingUpdateDaemonSet>
}

pub mod update_strategy {
    use kfl::DecodeScalar;

    #[derive(Debug, DecodeScalar, Default)]
    pub enum Type {
        #[default]
        RollingUpdate,
        OnDelete
    }
}

#[derive(Debug, Decode)]
pub struct RollingUpdateDaemonSet {
    max_surge: Option<IntOrString>,
    max_unavailable: Option<IntOrString>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/daemon-set-v1/#DaemonSetStatus>
#[derive(Debug, Decode)]
pub struct DaemonSetStatus {
    number_ready: i32,
    number_available: Option<i32>,
    number_unavailable: Option<i32>,
    number_misscheduled: i32,
    desired_number_scheduled: i32,
    current_number_scheduled: i32,
    updated_number_scheduled: Option<i32>,
    collision_count: Option<i32>,
    conditions: Vec<Condition>,
    observed_generation: i64
}
