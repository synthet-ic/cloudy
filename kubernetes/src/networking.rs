pub mod cluster_cidr;
pub mod gateway;
pub mod ingress;
pub mod ingress_class;
pub mod network_policy;

// use kfl::Decode;

pub use cluster_cidr::ClusterCidr;
pub use ingress::Ingress;
pub use ingress_class::IngressClass;
pub use network_policy::NetworkPolicy;

// #[derive(Debug, Decode)]
// pub enum Networking {
//     ClusterCidr(ClusterCidr),
//     Ingress(Ingress),
//     IngressClass(IngressClass),
//     NetworkPolicy(NetworkPolicy)
// }
