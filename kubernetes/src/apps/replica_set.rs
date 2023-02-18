/*!
- Concepts <https://kubernetes.io/docs/concepts/workloads/controllers/replicaset/>
- Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replica-set-v1/>
*/

use kfl::Decode;

use crate::{
    core::pod_template::PodTemplateSpec,
    meta::{
        condition::Condition,
        label_selector::Selector,
        metadata::Metadata
    },
};

#[derive(Debug, Decode)]
pub struct ReplicaSet {
    metadata: Metadata,
    spec: ReplicaSetSpec,
    status: Option<ReplicaSetStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replica-set-v1/#ReplicaSetSpec>
#[derive(Debug, Decode)]
pub struct ReplicaSetSpec {
    selector: Selector,
    template: Option<PodTemplateSpec>,
    replicas: Option<i32>,
    min_ready_seconds: Option<i32>,
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replica-set-v1/#ReplicaSetStatus>
#[derive(Debug, Decode)]
pub struct ReplicaSetStatus {
    replicas: i32,
    available_replicas: Option<i32>,
    ready_replicas: Option<i32>,
    fully_labeled_replicas: Option<i32>,
    conditions: Vec<Condition>,
    observed_generation: Option<i64>
}
