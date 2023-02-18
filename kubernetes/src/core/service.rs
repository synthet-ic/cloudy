//! - Concepts
//! <https://kubernetes.io/docs/concepts/services-networking/service/>
//! - Reference
//! <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/>

use std::{
    collections::HashMap,
    net::IpAddr
};

use kfl::Decode;

use crate::{
    meta::{condition::Condition, metadata::Metadata},
    port_status::PortStatus,
    protocol::Protocol,
    IntOrString
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Service>
#[derive(Debug, Decode)]
pub struct Service {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// Spec describes the attributes that a user creates on a service.
///
/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Spec>
#[derive(Debug, Decode)]
pub struct Spec {
    /// Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types `ClusterIp`, `NodePort`, and `LoadBalancer`. Ignored if type is `ExternalName`.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/>
    selector: HashMap<String, String>,
    /// List of ports that are exposed by this service.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    ports: Vec<Port>,
    /// `type` determines how the Service is exposed. Defaults to `ClusterIp`. Valid options are `ExternalName`, `ClusterIp`, `NodePort`, and `LoadBalancer`.
    /// - `ClusterIp` allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is `None`, no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP.
    /// - `NodePort` builds on ClusterIp and allocates a port on every node which routes to the same endpoints as the clusterIP.
    /// - `LoadBalancer` builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP.
    /// - `ExternalName` aliases this service to the specified [`external_name`][Self::external_name]. Several other fields do not apply to `ExternalName` services.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types>
    #[kfl(default = Type::ClusterIp)]
    r#type: Type,
    /// List of IP families (e.g. Ipv4, Ipv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the [`ip_family_policy`][Self::ip_family_policy] field. If this field is specified manually, the requested family is available in the cluster, and [`ip_family_policy`][Self::ip_family_policy] allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are `Ipv4` and `Ipv6`. This field only applies to Services of types `ClusterIp`, `NodePort`, and `LoadBalancer`, and does apply to 'headless' services. This field will be wiped when updating a Service to type `ExternalName`.
    ///
    /// This field may hold a maximum of two entries (dual-stack families, in either order). These families must correspond to the values of the [`cluster_ips`][Self::cluster_ips] field, if specified. Both [`cluster_ips`][Self::cluster_ips] and [`ip_families`][Self::ip_families] are governed by the [`ip_family_policy`][Self::ip_family_policy] field.
    ip_families: Vec<IpFamily>,
    /// `ip_family_policy` represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to `SingleStack`. Services can be `SingleStack` (a single IP family), `PreferDualStack` (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or `RequireDualStack` (two IP families on dual-stack configured clusters, otherwise fail). The [`ip_families`][Self::ip_families] and [`cluster_ips`][Self::cluster_ips] fields depend on the value of this field. This field will be wiped when updating a service to type `ExternalName`.
    ip_family_policy: Option<IpFamilyPolicy>,
    /// IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the [`type`][Self::type] field is also being changed to `ExternalName` (which requires this field to be blank) or the [`type`][Self::type] field is being changed from `ExternalName` (in which case this field may optionally be specified, as describe above). Valid values are `None`, empty string (`""`), or a valid IP address. Setting this to `None` makes a 'headless service' (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required. Only applies to types `ClusterIp`, `NodePort`, and `LoadBalancer`. If this field is specified when creating a Service of type `ExternalName`, creation will fail. This field will be wiped when updating a Service to type `ExternalName`.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    // #[kfl(rename(serialize = "clusterIP"))]
    cluster_ip: Option<ClusterIp>,
    /// `cluster_ips` is a list of IP addresses assigned to this service, and are usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the [`type`][Self::type] field is also being changed to `ExternalName` (which requires this field to be empty) or the [`type`][Self::type] field is being changed from `ExternalName` (in which case this field may optionally be specified, as describe above). Valid values are `None`, empty string (`""`), or a valid IP address. Setting this to `None` makes a 'headless service' (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required. Only applies to types `ClusterIp`, `NodePort`, and `LoadBalancer`. If this field is specified when creating a Service of type `ExternalName`, creation will fail. This field will be wiped when updating a Service to type `ExternalName`. If this field is not specified, it will be initialised from the [`cluster_ip`][Self::cluster_ip] field. If this field is specified, clients must ensure that `cluster_ips[0]` and [`cluster_ip`][Self::cluster_ip] have the same value.
    ///
    /// This field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the [`ip_families`][Self::ip_families] field. Both [`cluster_ips`][Self::cluster_ips] and [`ip_families`][Self::ip_families] are governed by the [`ip_family_policy`][Self::ip_family_policy] field.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    // #[kfl(rename(serialize = "clusterIP"))]
    cluster_ips: Vec<ClusterIp>,
    /**
    `external_ips` is a list of IP addresses for which nodes in the cluster will also accept traffic for this service. These IPs are not managed by Kubernetes. The user is responsible for ensuring that traffic arrives at a node with this IP. A common example is external load-balancers that are not part of the Kubernetes system.
    */
    // #[kfl(rename(serialize = "externalIPs"))]
    external_ips: Vec<String>,
    /**
    Supports `ClientIp` and `None`. Used to maintain session affinity. Enable client IP based session affinity. Must be `ClientIp` or `None`. Defaults to `None`.
    
    More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    */
    #[kfl(default = SessionAffinity::None)]
    session_affinity: SessionAffinity,
    /// If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature.
    ///
    /// More info: <https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/>
    load_balancer_source_ranges: Vec<String>,
    /// `load_balancer_class` is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. `"internal-vip"` or `"example.com/internal-vip"`. Unprefixed names are reserved for end-users. This field can only be set when the Service type is `LoadBalancer`. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type `LoadBalancer`. Once set, it can not be changed. This field will be wiped when a service is updated to a non `LoadBalancer` type.
    load_balancer_class: Option<String>,
    /// `external_name` is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved. Must be a lowercase [RFC 1123](https://www.rfc-editor.org/rfc/rfc1123) hostname and requires [`type`][Self::type] to be `ExternalName`.
    external_name: Option<String>,
    /// `external_traffic_policy` describes how nodes distribute service traffic they receive on one of the Service's 'externally-facing' addresses (NodePorts, ExternalIPs, and LoadBalancer IPs). If set to `Local`, the proxy will configure the service in a way that assumes that external load balancers will take care of balancing the service traffic between nodes, and so each node will deliver traffic only to the node-local endpoints of the service, without masquerading the client source IP. (Traffic mistakenly sent to a node with no endpoints will be dropped.) The default value, `Cluster`, uses the standard behaviour of routing to all endpoints evenly (possibly modified by topology and other features). Note that traffic sent to an External IP or LoadBalancer IP from within the cluster will always get 'Cluster' semantics, but clients sending to a NodePort from within the cluster may need to take traffic policy into account when picking a node.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/services-networking/service/#external-traffic-policy>
    #[kfl(default = TrafficPolicy::Cluster)]
    external_traffic_policy: TrafficPolicy,
    /// Describes how nodes distribute service traffic they receive on the ClusterIp. If set to `Local`, the proxy will assume that pods only want to talk to endpoints of the service on the same node as the pod, dropping the traffic if there are no local endpoints. The default value, `Cluster`, uses the standard behaviour of routing to all endpoints evenly (possibly modified by topology and other features).
    #[kfl(default = TrafficPolicy::Cluster)]
    internal_traffic_policy: TrafficPolicy,
    /// Specifies the healthcheck nodePort for the service. This only applies when [`type`][Self::type] is set to `LoadBalancer` and [`external_traffic_policy`][Self::external_traffic_policy] is set to `Local`. If a value is specified, is in-range, and is not in use, it will be used. If not specified, a value will be automatically allocated. External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not. If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type). This field cannot be updated once set.
    health_check_node_port: Option<u16>,
    /// Indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered 'ready' even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behaviour.
    publish_not_ready_addresses: Option<bool>,
    /// Contains the configurations of session affinity.
    session_affinity_config: Option<SessionAffinityConfig>,
    /// `allocate_load_balancer_node_ports` defines if NodePorts will be automatically allocated for services with type `LoadBalancer`. Default is `true`. It may be set to `false` if the cluster load-balancer does not rely on NodePorts. If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type `LoadBalancer` and will be cleared if the [`type`][Self::type] is changed to any other type.
    #[kfl(default = true)]
    allocate_load_balancer_node_ports: bool
}

/// Port contains information on service's port.
#[derive(Debug, Decode)]
pub struct Port {
    /// Port that will be exposed by this service.
    port: u16,
    /// Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an `IANA_SVC_NAME`. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the `port` field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the `port` field. More info: <https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service>
    target_port: Option<IntOrString>,
    /// IP protocol for this port. Supports `TCP`, `UDP`, and `SCTP`. Default is `TCP`.
    #[kfl(default = Protocol::Tcp)]
    protocol: Protocol,
    /// name of this port within the service. This must be a `DNS_LABEL`. All ports within a Spec must have unique names. When considering the endpoints for a Service, this must match the `name` field in the `EndpointPort`. Optional if only one Port is defined on this service.
    name: Option<String>,
    /// Port on each node on which this service is exposed when type is NodePort or LoadBalancer. Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail. If not specified, a port will be allocated if this Service requires one. If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIp). More info: <https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport>
    node_port: Option<u16>,
    /// Application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and <https://www.iana.org/assignments/service-names>). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol.
    app_protocol: Option<String>
}

#[derive(Debug, Decode)]
pub enum Type {
    ExternalName,
    ClusterIp,
    NodePort,
    LoadBalancer
}

#[derive(Debug, Decode)]
pub enum IpFamily {
    Ipv4,
    Ipv6,
}

#[derive(Debug, Decode, Default)]
pub enum IpFamilyPolicy {
    #[default]
    SingleStack,
    PreferDualStack,
    RequireDualStack
}

#[derive(Debug, Decode)]
pub enum ClusterIp {
    None,
    Empty,
    Ip(String)
}

#[derive(Debug, Decode, Default)]
pub enum SessionAffinity {
    ClientIp,
    #[default]
    None,
}

#[derive(Debug, Decode, Default)]
pub enum TrafficPolicy {
    Local,
    #[default]
    Cluster
}

#[derive(Debug, Decode, Default)]
pub struct SessionAffinityConfig {
    // #[kfl(rename(serialize = "clientIP"))]
    client_ip: Option<ClientIpConfig>
}

#[derive(Debug, Decode, Default)]
pub struct ClientIpConfig {
    /// Specifies the seconds of `ClientIp` type session sticky time. The value must be > 0 && <= 86400 (for 1 day) if [`session_affinity`][Spec::session_affinity] = `ClientIp`. Default value is `10800` (for 3 hours).
    timeout_seconds: Option<u16>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Status>
#[derive(Debug, Decode)]
pub struct Status {
    conditions: Vec<Condition>,
    load_balancer: LoadBalancerStatus
}
#[derive(Debug, Decode)]
pub struct LoadBalancerStatus {
    ingress: Vec<LoadBalancerIngress>,
}

#[derive(Debug, Decode)]
pub struct LoadBalancerIngress {
    hostname: String,
    ip: String,
    ports: Vec<PortStatus>
}
