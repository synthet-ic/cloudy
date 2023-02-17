use kfl::Decode;

use crate::time::Time;

#[derive(Debug, Decode)]
pub struct Condition<T = ConditionType> {
    /// Status of the condition, one of True, False, Unknown.
    status: ConditionStatus,
    /// Type of the condition.
    r#type: T,
    /// Last time the condition transit from one status to another.
    last_transition_time: Time,
    last_update_time: Time,
    /// Human readable message indicating details about last transition.
    message: String,
    /// (brief) reason for the condition's last transition.
    reason: Reason,
}

#[derive(Debug, Decode)]
pub enum ConditionStatus {
    True,
    False,
    Unknown
}

#[derive(Debug, Decode)]
pub enum Reason {
    ReplicaSetUpdated,
    MinimumReplicasAvailable,
    FailedCreate
}

#[derive(Debug, Decode)]
pub enum ConditionType {
    Progressing,
    Available,
    ReplicaFailure
}
