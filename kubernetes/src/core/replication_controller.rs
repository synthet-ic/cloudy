//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replication-controller-v1/>

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    core::pod_template::PodTemplateSpec,
    meta::{condition::Condition, metadata::Metadata},
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replication-controller-v1/#ReplicationController>
#[derive(Debug, Decode)]
pub struct ReplicationController {
    metadata: Metadata,
    spec: Spec,
    status: Status
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replication-controller-v1/#Spec>
#[derive(Debug, Decode)]
pub struct Spec {
    selector: HashMap<String, String>,
    template: PodTemplateSpec,
    replicas: i32,
    min_ready_seconds: i32
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/replication-controller-v1/#Status>
#[derive(Debug, Decode)]
pub struct Status {
    replicas: i32,
    available_replicas: Option<i32>,
    ready_replicas: Option<i32>,
    fully_labeled_replicas: Option<i32>,
    conditions: Vec<Condition>,
    observed_generation: Option<i64>
}
