//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/cluster-role-binding-v1/>

use kfl::Decode;

use crate::meta::Metadata;
use super::{RoleRef, Subject};

/// <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/cluster-role-binding-v1/#ClusterRoleBinding>
#[derive(Debug, Decode)]
pub struct ClusterRoleBinding {
    metadata: Option<Metadata>,
    role_ref: RoleRef,
    subjects: Vec<Subject>
}
