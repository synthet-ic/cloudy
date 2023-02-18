/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/endpoint/v3/endpoint_components.proto>
*/

use crate::config::core::{
    address::Address,
    base::{Locality, Metadata},
    config_source::ConfigSource,
    health_check::HealthStatus,
};

/// Upstream host identifier.
pub struct Endpoint {
    /**
    The upstream host address.

    > ATTENTION: The form of host address depends on the given cluster type. For STATIC or EDS, it is expected to be a direct IP address (or something resolvable by the specified [`resolver`][crate::config::core::SocketAddress.resolver_name] in the Address). For LOGICAL or STRICT DNS, it is expected to be hostname, and will be resolved via DNS.
    */
    address: Address,

    /**
    The optional health check configuration is used as configuration for the health checker to contact the health checked host.

    > ATTENTION: This takes into effect only for upstream clusters with :ref:`active health checking <arch_overview_health_checking>` enabled.
    */
    health_check_config: HealthCheckConfig,

    /**
    The hostname associated with this endpoint. This hostname is not used for routing or address resolution. If provided, it will be associated with the endpoint, and can be used for features that require a hostname, like [`AutoHostRewrite`][crate::config::route::route_components::HostRewriteSpecifier::AutoHostRewrite].
    */
    hostname: String
}

/// The optional health check configuration.
pub struct HealthCheckConfig {
    /**
    Optional alternative health check port value.

    By default the health check address port of an upstream host is the same as the host's serving address port. This provides an alternative health check port. Setting this with a non-zero value allows an upstream host to have different health check address port.
    */
    port_value: u16,

    /**
    By default, the host header for L7 health checks is controlled by cluster level configuration (see: [`host`][crate::config::core::health_check::HTTPHealthCheck::host] and [`authority`][crate::config::core::health_check::GRPCHealthCheck::authority]). Setting this to a non-empty value allows overriding the cluster level configuration for a specific endpoint.
    */
    hostname: String,

    /**
    Optional alternative health check host address.

    > ATTENTION: The form of the health check host address is expected to be a direct IP address.
    */
    address: Address,

    /**
    Optional flag to control if perform active health check for this endpoint.
    Active health check is enabled by default if there is a health checker.
    */
    disable_active_health_check: bool
}

/// An Endpoint that Envoy can route traffic to.
pub struct LBEndpoint {
    /// Upstream host identifier or a named reference.
    host_identifier: HostIdentifier,

    /// Optional health status when known and supplied by EDS server.
    health_status: HealthStatus,

    /**
    The endpoint metadata specifies values that may be used by the load balancer to select endpoints in a cluster for a given request. The filter name should be specified as `envoy.lb`. An example boolean key-value pair is `canary`, providing the optional canary status of the upstream host.
    This may be matched against in a route's [`RouteAction`][crate::config::route::route_components::RouteAction] metadata_match field to subset the endpoints considered in cluster load balancing.
    */
    metadata: Metadata,

    /**
    The optional load balancing weight of the upstream host; at least 1.
    Envoy uses the load balancing weight in some of the built in load balancers. The load balancing weight for an endpoint is divided by the sum of the weights of all endpoints in the endpoint's locality to produce a percentage of traffic for the endpoint. This percentage is then further weighted by the endpoint's locality's load balancing weight from LocalityLBEndpoints. If unspecified, will be treated as 1. The sum of the weights of all endpoints in the endpoint's locality must not exceed u32_t maximal value (4294967295).

    [(validate.rules).u32 = {gte: 1}]
    */
    load_balancing_weight: u32
}

/// A configuration for a LEDS collection.
pub struct LEDSClusterLocalityConfig {
    /// Configuration for the source of LEDS updates for a Locality.
    leds_config: ConfigSource,

    /// The xDS transport protocol glob collection resource name.
    /// The service is only supported in delta xDS (incremental) mode.
    leds_collection_name: String
}

/**
A group of endpoints belonging to a Locality.
One can have multiple LocalityLBEndpoints for a locality, but only if they have different priorities.
*/
pub struct LocalityLBEndpoints {
    /// Identifies location of where the upstream hosts run.
    locality: Locality,

    /**
    The group of endpoints belonging to the locality specified.
    > #comment:TODO(adisuissa): Once LEDS is implemented this field needs to be deprecated and replaced by `load_balancer_endpoints`.
    */
    lb_endpoints: Vec<LBEndpoint>,

    lb_config: LBConfig,

    /**
    Optional: Per priority/region/zone/sub_zone weight; at least 1. The load balancing weight for a locality is divided by the sum of the weights of all localities  at the same priority level to produce the effective percentage of traffic for the locality. The sum of the weights of all localities at the same priority level must not exceed u32_t maximal value (4294967295).

    Locality weights are only considered when :ref:`locality weighted load balancing <arch_overview_load_balancing_locality_weighted_lb>` is configured. These weights are ignored otherwise. If no weights are specified when locality weighted load balancing is enabled, the locality is assigned no load.

    [(validate.rules).u32 = {gte: 1}]
    */
    load_balancing_weight: u32,

    /**
    Optional: the priority for this LocalityLBEndpoints. If unspecified this will default to the highest priority (0).

    Under usual circumstances, Envoy will only select endpoints for the highest priority (0). In the event all endpoints for a particular priority are unavailable/unhealthy, Envoy will fail over to selecting endpoints for the next highest priority group.

    Priorities should range from 0 (highest) to N (lowest) without skipping.

    [(validate.rules).u32 = {lte: 128}]
    */
    priority: u8,

    /**
    Optional: Per locality proximity value which indicates how close this locality is from the source locality. This value only provides ordering information (lower the value, closer it is to the source locality).
    This will be consumed by load balancing schemes that need proximity order to determine where to route the requests.
    */
    proximity: u32
}

pub enum LBConfig {
    /**
    The group of endpoints belonging to the locality.
    > #comment:TODO(adisuissa): Once LEDS is implemented the `lb_endpoints` field needs to be deprecated.
    */
    LoadBalancerEndpoints(LBEndpointList),

    /// LEDS Configuration for the current locality.
    LEDSClusterLocalityConfig(LEDSClusterLocalityConfig)
}

/// A list of endpoints of a specific locality.
pub struct LBEndpointList {
    lb_endpoints: Vec<LBEndpoint>
}

/// Upstream host identifier or a named reference.
pub enum HostIdentifier {
    Endpoint(Endpoint),
    EndpointName(String)
}
