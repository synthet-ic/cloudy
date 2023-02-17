//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-class-v1/>

use kfl::Decode;

use crate::meta::metadata::Metadata;

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-class-v1/#IngressClass>
#[derive(Debug, Decode)]
pub struct IngressClass {
    metadata: Metadata,
    spec: IngressClassSpec
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-class-v1/#IngressClassSpec>
#[derive(Debug, Decode)]
pub struct IngressClassSpec {
    controller: Option<String>,
    parameters: Option<IngressClassParametersReference>
}

#[derive(Debug, Decode)]
pub struct IngressClassParametersReference {
    kind: String,
    name: String,
    api_group: Option<String>,
    namespace: Option<String>,
    scope: Option<IngressClassParametersReferenceScope>
}

#[derive(Debug, Decode, Default)]
pub enum IngressClassParametersReferenceScope {
    #[default]
    Cluster,
    Namespace
}
