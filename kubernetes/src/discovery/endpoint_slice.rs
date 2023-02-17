//! - Concepts <https://kubernetes.io/docs/concepts/services-networking/endpoint-slices/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/endpoint-slice-v1/>

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    core::{
        endpoints::EndpointPort,
        reference::Reference
    },
    meta::metadata::Metadata
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/endpoint-slice-v1/#EndpointSlice>
#[derive(Debug, Decode)]
pub struct EndpointSlice {
    metadata: Metadata,
    address_type: AddressType,
    endpoints: Vec<Endpoint>,
    ports: Vec<EndpointPort>
}

#[derive(Debug, Decode)]
pub enum AddressType {
    Ipv4,
    Ipv6,
    Fqdn
}

#[derive(Debug, Decode)]
pub struct Endpoint {
    conditions: Option<EndpointConditions>,
    deprecated_topology: HashMap<String, String>,
    hints: Option<EndpointHints>,
    hostname: Option<String>,
    node_name: Option<String>,
    target_ref: Option<Reference>,
    zone: Option<String>
}

#[derive(Debug, Decode)]
pub struct EndpointConditions {
    ready: Option<bool>,
    serving: Option<bool>,
    terminating: Option<bool>,
}

#[derive(Debug, Decode)]
pub struct EndpointHints {
    for_zones: Vec<ForZone>,
}

#[derive(Debug, Decode)]
pub struct ForZone {
    name: String,
}
