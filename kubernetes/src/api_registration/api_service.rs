/*!
Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/>
*/

use kfl::Decode;

use crate::meta::{
    condition::Condition,
    metadata::Metadata,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#ApiService>
#[derive(Debug, Decode)]
pub struct ApiService {
    metadata: Option<Metadata>,
    spec: ApiServiceSpec,
    status: Option<ApiServiceStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#ApiServiceSpec>
#[derive(Debug, Decode)]
pub struct ApiServiceSpec {
    group_priority_minimum: i32,
    version_priority: i32,
    ca_bundle: Vec<u8>,
    group: Option<String>,
    insecure_skip_tls_verify: Option<bool>,
    service: Option<ServiceReference>,
    version: Option<String>
}

#[derive(Debug, Decode)]
pub struct ServiceReference {
    name: Option<String>,
    namespace: Option<String>,
    port: Option<i32>,
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#ApiServiceStatus>
#[derive(Debug, Decode)]
pub struct ApiServiceStatus {
    #[kfl(children)]
    conditions: Vec<Condition>
}
