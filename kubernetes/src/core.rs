pub mod binding;
pub mod config_map;
pub mod endpoints;
pub mod field_selector;
pub mod limit_range;
pub mod local_reference;
pub mod namespace;
pub mod node;
pub mod node_selector_requirement;
pub mod persistent_volume;
pub mod persistent_volume_claim;
pub mod pod;
pub mod pod_template;
pub mod reference;
pub mod replication_controller;
pub mod resource_field_selector;
pub mod resource_quota;
pub mod secret;
pub mod service;
pub mod service_account;
pub mod typed_local_reference;
pub mod volume;

// use kfl::Decode;

pub use binding::Binding;
pub use config_map::ConfigMap;
pub use endpoints::Endpoints;
pub use field_selector::FieldSelector;
pub use limit_range::LimitRange;
pub use namespace::Namespace;
pub use node::Node;
pub use persistent_volume::PersistentVolume;
pub use persistent_volume_claim::PersistentVolumeClaim;
pub use pod::Pod;
pub use service::Service;
pub use service_account::ServiceAccount;
pub use volume::Volume;

// #[derive(Debug, Decode)]
// pub enum Core {
//     Binding(Binding),
//     ConfigMap(ConfigMap),
//     Endpoints(Endpoints),
//     FieldSelector(FieldSelector),
//     LimitRange(LimitRange),
//     Namespace(Namespace),
//     Node(Node),
//     PersistentVolume(PersistentVolume),
//     PersistentVolumeClaim(PersistentVolumeClaim),
//     Pod(Pod),
//     Service(Service),
//     ServiceAccount(ServiceAccount),
//     Volume(Volume)
// }
