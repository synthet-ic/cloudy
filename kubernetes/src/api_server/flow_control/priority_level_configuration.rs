//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/priority-level-configuration-v1beta2/>

use kfl::Decode;

use crate::meta::{condition::Condition, metadata::Metadata};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/priority-level-configuration-v1beta2/#PriorityLevelConfiguration>
#[derive(Debug, Decode)]
pub struct PriorityLevelConfiguration {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/priority-level-configuration-v1beta2/#PriorityLevelConfigurationSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    r#type: Type,
    limited: Option<Limited>
}

#[derive(Debug, Decode)]
pub enum Type {
    Exempt,
    Limited
}

#[derive(Debug, Decode)]
pub struct Limited {
    assured_concurrency_shares: Option<i32>,
    limit_response: Option<LimitResponse>,   
}

#[derive(Debug, Decode)]
pub struct LimitResponse {
    r#type: limit_response::Type,
    queuing: Option<Queuing>
}

pub mod limit_response {
    use kfl::DecodeScalar;

    #[derive(Debug, DecodeScalar)]
    pub enum Type {
        Queue,
        Reject
    }
}

#[derive(Debug, Decode)]
pub struct Queuing {
    hand_size: Option<i32>,
    queue_length_limit: Option<i32>,
    queues: Option<i32>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/priority-level-configuration-v1beta2/#PriorityLevelConfigurationStatus>
#[derive(Debug, Decode)]
pub struct Status {
    conditions: Vec<Condition>
}
