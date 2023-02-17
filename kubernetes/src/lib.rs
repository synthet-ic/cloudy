#![allow(rustdoc::private_intra_doc_links)]

pub mod admission_registration;
pub mod api_extensions;
pub mod api_registration;
pub mod api_server;
pub mod apps;
pub mod audit;
pub mod authentication;
pub mod authorisation;
pub mod autoscaling;
pub mod batch;
pub mod certificates;
pub mod config;
pub mod coordination;
pub mod core;
// pub mod cri;
pub mod discovery;
pub mod events;
pub mod kind;
pub mod kubeadm;
pub mod kubectl;
pub mod kubelet;
pub mod kustomization;
pub mod meta;
pub mod networking;
pub mod node;
pub mod node_selector;
pub mod policy;
pub mod port_status;
pub mod protocol;
pub mod quantity;
pub mod sigs;
pub mod storage;
pub mod time;

use kfl::Decode;

use authorisation::rbac::Rbac;
// use config::{
//     api_server::AdmissionConfiguration
// };
use self::core::Core;
use kubeadm::Kubeadm;

// #[derive(Debug, Decode)]
// #[kfl(tag = "api-version")]
// pub enum Kubernetes {
//     #[kfl(rename = "admissionregistration.k8s.io/v1")]
//     AdmissionRegistration(AdmissionRegistration),
//     #[kfl(rename = "apiregistration.k8s.io/v1")]
//     APIRegistration(APIRegistration),
//     #[kfl(rename = "apps/v1")]
//     Apps(Apps),
//     #[kfl(rename = "audit.k8s.io/v1")]
//     Audit(Audit),
//     #[kfl(rename = "authentication.k8s.io/v1")]
//     Authentication(Authentication),
//     #[kfl(rename = "rbac.authorization.k8s.io/v1")]
//     Authorisation(Rbac),
//     #[kfl(rename = "autoscaling/v2")]
//     Autoscaling(Autoscaling),
//     #[kfl(rename = "batch/v1")]
//     Batch(Batch),
//     #[kfl(rename = "v1")]
//     Core(Core),
//     #[kfl(rename = "discovery.k8s.io/v1")]
//     Discovery(Discovery),
//     #[kfl(rename = "events.k8s.io/v1")]
//     Events(Events),
//     #[kfl(rename = "kubeadm.k8s.io/v1beta3")]
//     Kubeadm(Kubeadm),
//     #[kfl(rename = "networking.k8s.io/v1")]
//     Networking(Networking),
//     #[kfl(rename = "storage.k8s.io/v1")]
//     Storage(Storage)
// }

// #[derive(Debug, Decode)]
// pub enum IntOrString {
//     Int(i32),
//     String(String)
// }

pub fn default_true() -> bool {
    true
}
