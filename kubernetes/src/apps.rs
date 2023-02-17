pub mod daemon_set;
pub mod deployment;
pub mod replica_set;
pub mod stateful_set;

// use kfl::Decode;

pub use daemon_set::DaemonSet;
pub use deployment::Deployment;
pub use replica_set::ReplicaSet;
pub use stateful_set::StatefulSet;

// #[derive(Debug, Decode)]
// pub enum Apps {
//     DaemonSet(DaemonSet),
//     Deployment(Deployment),
//     ReplicaSet(ReplicaSet),
//     StatefulSet(StatefulSet)
// }
