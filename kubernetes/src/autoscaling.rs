pub mod horizontal_pod_autoscaler;

use kfl::Decode;

pub use horizontal_pod_autoscaler::HorizontalPodAutoscaler;

// #[derive(Debug, Decode)]
// pub enum Autoscaling {
//     HorizontalPodAutoscaler(HorizontalPodAutoscaler)
// }
