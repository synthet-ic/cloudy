/*!
- Concepts <https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/>
- Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/stateful-set-v1/>
*/

use kfl::Decode;

use crate::{
    core::{
        persistent_volume_claim::PersistentVolumeClaim,
        pod_template::PodTemplateSpec
    },
    meta::{
        condition::Condition,
        label_selector::LabelSelector,
        metadata::Metadata,
    },
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/stateful-set-v1/#StatefulSet>
#[derive(Debug, Decode)]
pub struct StatefulSet {
    metadata: Metadata,
    spec: StatefulSetSpec,
    status: Option<StatefulSetStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/stateful-set-v1/#StatefulSetSpec>
#[derive(Debug, Decode)]
pub struct StatefulSetSpec {
    /// `service_name` is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: `pod-specific-string.serviceName.default.svc.cluster.local` where `"pod-specific-string"` is managed by the StatefulSet controller.
    service_name: String,

    /// `selector` is a label query over pods that should match the replica count. It must match the pod template's labels.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors>
    selector: LabelSelector,
    /// `template` is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet.
    template: PodTemplateSpec,
    /// `replicas` is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to `1`.
    replicas: Option<u32>,
    /// `update_strategy` indicates the `StatefulSetUpdateStrategy` that will be employed to update Pods in the StatefulSet when a revision is made to Template.
    update_strategy: Option<StatefulSetUpdateStrategy>,
    /// `pod_management_policy` controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
    pod_management_policy: Option<PodManagementPolicy>,
    /// Maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is `10`.
    revision_history_limit: Option<u32>,
    /// List of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) `volume_mount` in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.
    volume_claim_templates: Vec<PersistentVolumeClaim>,
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to `0` (pod will be considered available as soon as it is ready)
    min_ready_seconds: Option<u32>,
    /// `persistent_volume_claim_retention_policy` describes the lifecycle of persistent volume claims created from [`volume_claim_templates`][Self::volume_claim_templates]. By default, all persistent volume claims are created as needed and retained until manually deleted. This policy allows the lifecycle to be altered, for example by deleting persistent volume claims when their stateful set is deleted, or when their pod is scaled down. This requires the `StatefulSetAutoDeletePVC` feature gate to be enabled, which is alpha. +optional
    persistent_volume_claim_retention_policy:
        Option<StatefulSetPersistentVolumeClaimRetentionPolicy>,
}

#[derive(Debug, Decode)]
pub struct StatefulSetUpdateStrategy {
    r#type: StatefulSetUpdateStrategyType,
    rolling_update: RollingUpdateStatefulSetStrategy,
}

#[derive(Debug, Decode, Default)]
pub enum StatefulSetUpdateStrategyType {
    #[default]
    RollingUpdate,
}

#[derive(Debug, Decode)]
pub struct RollingUpdateStatefulSetStrategy {
    max_unavailable: i32,
    partition: i32,
}

#[derive(Debug, Decode, Default)]
pub enum PodManagementPolicy {
    #[default]
    OrderedReady,
    Parallel
}

#[derive(Debug, Decode)]
pub struct StatefulSetPersistentVolumeClaimRetentionPolicy {
    when_deleted: String,
    when_scaled: String
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replication-controller-v1/#ReplicationControllerStatus>
#[derive(Debug, Decode)]
pub struct StatefulSetStatus {
    replicas: i32,
    available_replicas: Option<i32>,
    ready_replicas: Option<i32>,
    fully_labeled_replicas: Option<i32>,
    conditions: Vec<Condition>,
    observed_generation: Option<i64>
}
