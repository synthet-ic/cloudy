//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/cluster-role-binding-v1/>

use kfl::Decode;

use crate::meta::Metadata;
use super::{RoleRef, Subject};

/// <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/cluster-role-binding-v1/#ClusterRoleBinding>
#[derive(Debug, Decode)]
pub struct ClusterRoleBinding {
    metadata: Option<Metadata>,
    /// RoleRef can only reference a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error.
    role_ref: RoleRef,
    /// Subjects holds references to the objects the role applies to.
    subjects: Vec<Subject>
}
