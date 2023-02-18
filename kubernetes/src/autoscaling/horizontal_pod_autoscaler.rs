//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/horizontal-pod-autoscaler-v2/>
//! - Tasks
//!   - <https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/>
//!   - <https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale-walkthrough/>

use kfl::Decode;

use crate::{
    meta::{condition::Condition, metadata::Metadata},
    time::Time
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/horizontal-pod-autoscaler-v2/#HorizontalPodAutoscaler>
#[derive(Debug, Decode)]
pub struct HorizontalPodAutoscaler {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/horizontal-pod-autoscaler-v2/#HorizontalPodAutoscalerSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    max_replicas: i32,
    scale_target_ref: CrossVersionObjectReference,
    min_replicas: Option<i32>,
    behaviour: Option<Behaviour>,
    metrics: Vec<Metric>,

}

#[derive(Debug, Decode)]
pub struct CrossVersionObjectReference {
}

#[derive(Debug, Decode)]
pub struct Behaviour {
}

#[derive(Debug, Decode)]
pub struct Metric {
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/horizontal-pod-autoscaler-v2/#StatHorizontalPodAutoscalerStatusus>
#[derive(Debug, Decode)]
pub struct Status {
    desired_replicas: i32,
    conditions: Vec<Condition>,
    current_metrics: Vec<status::Metric>,
    current_replicas: Option<i32>,
    last_scale_time: Option<Time>,
    observed_generation: Option<i64>
}

pub mod status {
    use kfl::Decode;

    #[derive(Debug, Decode)]
    pub struct Metric {
    }
}
