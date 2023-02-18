pub mod cluster_role;
pub mod cluster_role_binding;
pub mod role;
pub mod role_binding;

use kfl::Decode;

pub use cluster_role::ClusterRole;
pub use cluster_role_binding::ClusterRoleBinding;
pub use role::Role;
pub use role_binding::RoleBinding;

// #[derive(Debug, Decode)]
// pub enum Rbac {
//     ClusterRole(ClusterRole),
//     ClusterRoleBinding(ClusterRoleBinding),
//     Role(Role),
//     RoleBinding(RoleBinding)
// }

#[derive(Debug, Decode)]
pub struct RoleRef {
    api_group: String,
    kind: String,
    name: String
}

#[derive(Debug, Decode)]
pub struct Subject {
    kind: SubjectKind,
    name: String,
    api_group: Option<String>,
    namespace: Option<String>,
}

#[derive(Debug, Decode)]
pub enum SubjectKind {
    User,
    Group,
    ServiceAccount
}
