//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-class-v1/>

use kfl::{Decode, DecodeScalar};

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-class-v1/#IngressClass>
#[derive(Debug, Decode)]
pub struct IngressClass {
    metadata: Metadata,
    spec: Spec
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-class-v1/#IngressClassSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    controller: Option<String>,
    parameters: Option<Parameters>
}

#[derive(Debug, Decode)]
pub struct Parameters {
    kind: String,
    name: String,
    api_group: Option<String>,
    namespace: Option<String>,
    scope: Option<Scope>
}

#[derive(Debug, DecodeScalar, Default)]
pub enum Scope {
    #[default]
    Cluster,
    Namespace
}
