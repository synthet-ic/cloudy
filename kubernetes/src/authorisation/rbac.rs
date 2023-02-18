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

/// RoleRef contains information that points to the role being used.
#[derive(Debug, Decode)]
pub struct RoleRef {
    /// Group for the resource being referenced.
    api_group: String,
    /// Kind of resource being referenced.
    kind: String,
    /// Name of resource being referenced.
    name: String
}

/// Subject contains a reference to the object or user identities a role binding applies to. This can either hold a direct API object reference, or a value for non-objects such as user and group names.
#[derive(Debug, Decode)]
pub enum Subject {
    User {
        #[kfl(argument)]
        name: String,
        /// API group of the referenced subject. Defaults to "rbac.authorization.k8s.io".
        #[kfl(property, default = "rbac.authorization.k8s.io".into())]
        api_group: String,
    },
    Group {
        #[kfl(argument)]
        name: String,
        /// API group of the referenced subject. Defaults to "rbac.authorization.k8s.io".
        #[kfl(property, default = "rbac.authorization.k8s.io".into())]
        api_group: String,
    },
    ServiceAccount {
        #[kfl(argument)]
        name: String,
        /// API group of the referenced subject. Defaults to "".
        #[kfl(property, default)]
        api_group: Option<String>,
        /// Namespace of the referenced object.
        namespace: Option<String>,
    }
}
