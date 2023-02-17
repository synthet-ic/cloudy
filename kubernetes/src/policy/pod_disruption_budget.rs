//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/pod-disruption-budget-v1/>

use kfl::Decode;

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/pod-disruption-budget-v1/#PodDisruptionBudget>
#[derive(Debug, Decode)]
pub struct PodDisruptionBudget {
    metadata: Metadata,
    spec: PodDisruptionBudgetSpec,
    status: Option<PodDisruptionBudgetStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/pod-disruption-budget-v1/#PodDisruptionBudgetSpec>
#[derive(Debug, Decode)]
pub struct PodDisruptionBudgetSpec {
    
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/pod-disruption-budget-v1/#PodDisruptionBudgetStatus>
#[derive(Debug, Decode)]
pub struct PodDisruptionBudgetStatus {

}
