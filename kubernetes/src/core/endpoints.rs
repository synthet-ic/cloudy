/*!
Reference <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/endpoints-v1/>
*/

use kfl::Decode;

use crate::{
    core::reference::Reference,
    meta::metadata::Metadata,
    protocol::Protocol,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/endpoints-v1/#Endpoints>
#[derive(Debug, Decode)]
pub struct Endpoints {
    metadata: Metadata,
    subsets: Vec<EndpointSubset>
}

#[derive(Debug, Decode)]
pub struct EndpointSubset {
    addresses: Vec<EndpointAddress>,
    not_ready_addresses: Vec<EndpointAddress>,
    ports: Vec<EndpointPort>,
}

#[derive(Debug, Decode)]
pub struct EndpointAddress {
    ip: String,
    hostname: Option<String>,
    node_name: Option<String>,
    target_ref: Option<Reference>
}

#[derive(Debug, Decode)]
pub struct EndpointPort {
    port: i32,
    protocol: Option<Protocol>,
    name: Option<String>,
    app_protocol: Option<String>
}
