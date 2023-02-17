//! - Concepts <https://kubernetes.io/docs/concepts/services-networking/network-policies/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/>

use kfl::Decode;

use crate::{
    meta::{
        condition::Condition,
        label_selector::LabelSelector,
        metadata::Metadata
    },
    protocol::Protocol
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy>
#[derive(Debug, Decode)]
pub struct NetworkPolicy {
    metadata: Metadata,
    spec: NetworkPolicySpec,
    status: Option<NetworkPolicyStatus>
}

/// 
#[derive(Debug, Decode)]
pub struct NetworkPolicySpec {
    pod_selector: LabelSelector,
    policy_types: Vec<String>,
    ingress: Vec<NetworkPolicyIngressRule>,
    egress: Vec<NetworkPolicyEgressRule>
}

#[derive(Debug, Decode)]
pub struct NetworkPolicyIngressRule {
    from: Vec<NetworkPolicyPeer>,
    ports: Vec<NetworkPolicyPort>
}

#[derive(Debug, Decode)]
pub struct NetworkPolicyPeer {
    ip_block: IPBlock,
    namespace_selector: LabelSelector,
    pod_selector: LabelSelector
}

#[derive(Debug, Decode)]
pub struct IPBlock {
    cidr: String,
    except: Vec<String>
}

#[derive(Debug, Decode)]
pub struct NetworkPolicyPort {
    port: i32,
    end_port: i32,
    protocol: Protocol
}

#[derive(Debug, Decode)]
pub struct NetworkPolicyEgressRule {
    to: Vec<NetworkPolicyPeer>,
    ports: Vec<NetworkPolicyPort>
}

#[derive(Debug, Decode)]
pub struct NetworkPolicyStatus {
    conditions: Vec<Condition>
}
