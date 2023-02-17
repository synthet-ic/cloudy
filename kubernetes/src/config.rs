pub mod api_server;
pub mod kube_proxy;
pub mod kube_scheduler;
pub mod kubelet;

// use kfl::Decode;

pub use api_server::ApiServer;
pub use kube_proxy::KubeProxy;
pub use kube_scheduler::KubeScheduler;

// #[derive(Debug, Decode)]
// pub enum Config {
//     ApiServer(ApiServer),
//     KubeProxy(KubeProxy),
//     #[kfl(rename = "kubescheduler.config.k8s.io/v1")]
//     KubeScheduler(KubeScheduler)
// }
