//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-template-v1/>

use kfl::Decode;

use crate::{
    core::pod,
    meta::metadata::Metadata,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-template-v1/#PodTemplate>
#[derive(Debug, Decode)]
pub struct PodTemplate {
    metadata: Metadata,
    template: PodTemplateSpec
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-template-v1/#PodTemplateSpec>
#[derive(Debug, Decode)]
pub struct PodTemplateSpec {
    metadata: Option<Metadata>,
    spec: pod::Spec
}
