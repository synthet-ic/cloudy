//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/role-binding-v1/>

use kfl::Decode;

use crate::meta::metadata::Metadata;
use super::{RoleRef, Subject};

/// <https://kubernetes.io/docs/reference/kubernetes-api/authorization-resources/role-binding-v1/#RoleBinding>
#[derive(Debug, Decode)]
pub struct RoleBinding {
    metadata: Metadata,
    role_ref: RoleRef,
    subjects: Vec<Subject>,

}
