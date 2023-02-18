/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/health_check.proto>
*/

type Any = String;
type Struct = String;

use std::time::Duration;

use xds::types::matcher::StringMatcher;

use crate::{
    config::core::{
        base::{HeaderValueOption, RequestMethod},
        event_service_config::EventServiceConfig,
    },
    types::{
        http::CodecClientType,
        range::I64Range
    }
};

/// Endpoint health status.
pub enum HealthStatus {
    // The health status is not known. This is interpreted by Envoy as ``HEALTHY``.
    Unknown,

    // Healthy.
    Healthy,

    // Unhealthy.
    Unhealthy,

    /**
    Connection draining in progress. E.g.,
    `<https://aws.amazon.com/blogs/aws/elb-connection-draining-remove-instances-from-service-with-care/>`_
    or
    `<https://cloud.google.com/compute/docs/load-balancing/enabling-connection-draining>`_.
    This is interpreted by Envoy as ``UNHEALTHY``.
    */
    Draining,

    // Health check timed out. This is part of HDS and is interpreted by Envoy as
    // ``UNHEALTHY``.
    Timeout,

    // Degraded.
    Degraded,
}

pub struct HealthStatusSet {
    /// An order-independent set of health status.
    // [(validate.rules).repeated = {items {enum {defined_only: true}}}];
    statuses: Vec<HealthStatus>
}

pub struct HealthCheck {
    /**
    The time to wait for a health check response. If the timeout is reached the health check attempt will be considered a failure.

    [(validate.rules).duration = {
      required: true
      gt {}
    }]
    */
    timeout: Duration,

    /**
    The interval between health checks.

    [(validate.rules).duration = {
      required: true
      gt {}
    }]
    */
    interval: Duration,

    /// An optional jitter amount in milliseconds. If specified, Envoy will start health checking after for a random time in ms between 0 and initial_jitter. This only applies to the first health check.
    initial_jitter: Duration,

    /// An optional jitter amount in milliseconds. If specified, during every interval Envoy will add interval_jitter to the wait time.
    interval_jitter: Duration,

    /**
    An optional jitter amount as a percentage of interval_ms. If specified, during every interval Envoy will add `interval_ms` * `interval_jitter_percent` / 100 to the wait time.

    If interval_jitter_ms and interval_jitter_percent are both set, both of them will be used to increase the wait time.
    */
    interval_jitter_percent: u32,

    /**
    The number of unhealthy health checks required before a host is marked
    unhealthy. Note that for `http` health checking if a host responds with a code not in [`expected_statuses`][HTTPHealthCheck.expected_statuses] or [`retriable_statuses`][HTTPHealthCheck.retriable_statuses], this threshold is ignored and the host is considered immediately unhealthy.

    [(validate.rules).message = {required: true}]
    */
    unhealthy_threshold: u32,

    /**
    The number of healthy health checks required before a host is marked healthy. Note that during startup, only a single successful health check is required to mark a host healthy.

    [(validate.rules).message = {required: true}]
    */
    healthy_threshold: u32,

    /// [#not-implemented-hide:] Non-serving port for health checking.
    alt_port: u32,

    /// Reuse health check connection between health checks. Default is true.
    reuse_connection: bool,

    health_checker: HealthChecker,

    /**
    The "no traffic interval" is a special health check interval that is used when a cluster has never had traffic routed to it. This lower interval allows cluster information to be kept up to date, without sending a potentially large amount of active health checking traffic for no reason. Once a cluster has been used for traffic routing, Envoy will shift back to using the standard health check interval that is defined. Note that this interval takes precedence over any other.

    The default value for "no traffic interval" is 60 seconds.

    [(validate.rules).duration = {gt {}}]
    */
    no_traffic_interval: Duration,

    /**
    The "no traffic healthy interval" is a special health check interval that
    is used for hosts that are currently passing active health checking
    (including new hosts) when the cluster has received no traffic.

    This is useful for when we want to send frequent health checks with `no_traffic_interval` but then revert to lower frequency `no_traffic_healthy_interval` once a host in the cluster is marked as healthy.

    Once a cluster has been used for traffic routing, Envoy will shift back to using the standard health check interval that is defined.

    If no_traffic_healthy_interval is not set, it will default to the no traffic interval and send that interval regardless of health state.

    [(validate.rules).duration = {gt {}}]
    */
    no_traffic_healthy_interval: Duration,

    /**
    The "unhealthy interval" is a health check interval that is used for hosts that are marked as unhealthy. As soon as the host is marked as healthy, Envoy will shift back to using the standard health check interval that is defined.

    The default value for "unhealthy interval" is the same as "interval".

    [(validate.rules).duration = {gt {}}]
    */
    unhealthy_interval: Duration,

    /**
    The "unhealthy edge interval" is a special health check interval that is used for the first health check right after a host is marked as unhealthy. For subsequent health checks Envoy will shift back to using either "unhealthy interval" if present or the standard health check interval that is defined.

    The default value for "unhealthy edge interval" is the same as "unhealthy interval".

    [(validate.rules).duration = {gt {}}]
    */
    unhealthy_edge_interval: Duration,

    /**
    The "healthy edge interval" is a special health check interval that is used for the first health check right after a host is marked as healthy. For subsequent health checks
    Envoy will shift back to using the standard health check interval that is defined.

    The default value for "healthy edge interval" is the same as the default interval.

    [(validate.rules).duration = {gt {}}]
    */
    healthy_edge_interval: Duration,

    /**
    Specifies the path to the :ref:`health check event log <arch_overview_health_check_logging>`.
    If empty, no event log will be written.
    */
    event_log_path: String,

    /**
    The gRPC service for the health check event service.
    If empty, health check events won't be sent to a remote endpoint.
    */
    event_service: EventServiceConfig,

    /**
    If set to true, health check failure events will always be logged. If set to false, only the initial health check failure event will be logged.
    The default value is `false`.
    */
    always_log_health_check_failures: bool,

    /// This allows overriding the cluster TLS settings, just for health check connections.
    tls_options: TLSOptions,

    /**
    Optional key/value pairs that will be used to match a transport socket from those specified in the cluster's
    [tranport socket matches][crate::config::cluster::cluster::Cluster.transport_socket_matches].
    For example, the following match criteria

    ```yaml
    transport_socket_match_criteria:
      useMTLS: true
    ```

    Will match the following [cluster socket match][crate::config::cluster::cluster::Cluster.TransportSocketMatch]

    ```yaml
    transport_socket_matches:
    - name: "useMTLS"
      match:
        useMTLS: true
      transport_socket:
        name: envoy.transport_sockets.tls
        config: { ... } # tls socket configuration
    ```

    If this field is set, then for health checks it will supersede an entry of `envoy.transport_socket` in the
    [`LBEndpoint::metadata`][crate::config::endpoint::endpoint_components::LBEndpoint::metadata].
    This allows using different transport socket capabilities for health checking versus proxying to the endpoint.

    If the key/values pairs specified do not match any [`transport_socket_matches`][crate::config::cluster::cluster::Cluster::transport_socket_matches], the cluster's [`transport socket`][crate::config::cluster::cluster::Cluster::transport_socket] will be used for health check socket configuration.
    */
    transport_socket_match_criteria: Struct,
}

pub enum HealthChecker {
    // option (validate.required) = true;

    /// HTTP health check.
    HTTPHealthCheck(HTTPHealthCheck),

    /// TCP health check.
    TCPHealthCheck(TCPHealthCheck),

    /// gRPC health check.
    GRPCHealthCheck(GRPCHealthCheck),

    /// Custom health check.
    CustomHealthCheck(CustomHealthCheck),
}

pub struct TCPHealthCheck {
    /// Empty payloads imply a connect-only health check.
    send: Payload,

    /// When checking the response, “fuzzy” matching is performed such that each payload block must be found, and in the order specified, but not necessarily contiguous.
    receive: Vec<Payload>,
}

pub struct RedisHealthCheck {
    /**
    If set, optionally perform `EXISTS <key>` instead of `PING`. A return value from Redis of 0 (does not exist) is considered a passing healthcheck. A return value other than 0 is considered a failure. This allows the user to mark a Redis instance for maintenance by setting the specified key to any value and waiting for traffic to drain.
    */
    key: String,
}

/**
`grpc.health.v1.Health <https://github.com/grpc/grpc/blob/master/src/proto/grpc/health/v1/health.proto>`_-based healthcheck. See `gRPC doc <https://github.com/grpc/grpc/blob/master/doc/health-checking.md>`_ for details.
*/
pub struct GRPCHealthCheck {
    /**
    An optional service name parameter which will be sent to gRPC service in
    [grpc.health.v1.HealthCheckRequest](https://github.com/grpc/grpc/blob/master/src/proto/grpc/health/v1/health.proto#L20).
    message. See [gRPC health-checking overview](https://github.com/grpc/grpc/blob/master/doc/health-checking.md) for more information.
    */
    service_name: String,

    /**
    The value of the :authority header in the gRPC health check request. If left empty (default value), the name of the cluster this health check is associated with will be used. The authority header can be customized for a specific endpoint by setting the [`hostname`][crate::config::endpoint::endpoint_components::Endpoint::hostname] field.

    */
    // [(validate.rules).String = {well_known_regex: HTTP_HEADER_VALUE strict: false}]
    authority: String,

    /**
    Specifies a list of key-value pairs that should be added to the metadata of each GRPC call that is sent to the health checked cluster. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    */
    // [initial_metadata.len() <= 1000]
    initial_metadata: Vec<HeaderValueOption>,
}

/// Custom health check.
pub struct CustomHealthCheck {
    /// The registered name of the custom health checker.
    /// [!name.is_empty()]
    name: String,

    /// A custom health checker specific configuration which depends on the custom health checker being instantiated. See :api:`envoy/config/health_checker` for reference.
    /// [#extension-category: envoy.health_checkers]
    config_type: ConfigType
}

pub enum ConfigType {
    TypedConfig(Any)
}

/**
Health checks occur over the transport socket specified for the cluster. This implies that if a cluster is using a TLS-enabled transport socket, the health check will also occur over TLS.

This allows overriding the cluster TLS settings, just for health check connections.
*/
pub struct TLSOptions {
    /**
    Specifies the ALPN protocols for health check connections. This is useful if the corresponding upstream is using ALPN-based [`FilterChainMatch`][crate::config::listener::listener_components::FilterChainMatch] along with different protocols for health checks versus data connections. If empty, no ALPN protocols will be set on health check connections.
    */
    alpn_protocols: Vec<String>,
}

/// Describes the encoding of the payload bytes in the payload.
pub enum Payload {
    // option (validate.required) = true;

    /// Hex encoded payload. E.g., "000000FF".
    // [(validate.rules).String = {min_len: 1}]
    Text(String),
    /// Binary payload.
    Binary(Vec<u8>)
}

pub struct HTTPHealthCheck {
    /**
    The value of the host header in the HTTP health check request. If left empty (default value), the name of the cluster this health check is associated with will be used. The host header can be customized for a specific endpoint by setting the [`hostname`][crate::config::endpoint::endpoint_components::HealthCheckConfig::hostname] field.

    [(validate.rules).String = {well_known_regex: HTTP_HEADER_VALUE strict: false}]
    */
    host: String,

    /**
    Specifies the HTTP path that will be requested during health checking. For example
    `/healthcheck`.

    [(validate.rules).String = {min_len: 1 well_known_regex: HTTP_HEADER_VALUE strict: false}]
    */
    path: String,

    /// [#not-implemented-hide:] HTTP specific payload.
    send: Payload,

    /**
    Specifies a list of HTTP expected responses to match in the first `response_buffer_size` bytes of the response body.
    If it is set, both the expected response check and status code determine the health check.
    When checking the response, “fuzzy” matching is performed such that each payload block must be found,
    and in the order specified, but not necessarily contiguous.

    > NOTE: It is recommended to set `response_buffer_size` based on the total Payload size for efficiency.
      The default buffer size is 1024 bytes when it is not set.
    */
    receive: Vec<Payload>,

    /**
    Specifies the size of response buffer in bytes that is used to Payload match.
    The default value is 1024. Setting to 0 implies that the Payload will be matched against the entire response.

    [(validate.rules).uint64 = {gte: 0}]
    */
    response_buffer_size: u64,

    /**
    Specifies a list of HTTP headers that should be added to each request that is sent to the health checked cluster. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    [(validate.rules).repeated = {max_items: 1000}]
    */
    request_headers_to_add: Vec<HeaderValueOption>,

    /**
    Specifies a list of HTTP headers that should be removed from each request that is sent to the health checked cluster.

    [(validate.rules).repeated = {
      items {String {well_known_regex: HTTP_HEADER_NAME strict: false}}
    }]
    */
    request_headers_to_remove: Vec<String>,

    /**
    Specifies a list of HTTP response statuses considered healthy. If provided, replaces default 200-only policy - 200 must be included explicitly as needed. Ranges follow half-open semantics of [`I64Range`]. The start and end of each range are required. Only statuses in the range [100, 600) are allowed.
    */
    expected_statuses: Vec<I64Range>,

    /**
    Specifies a list of HTTP response statuses considered retriable. If provided, responses in this range will count towards the configured [`unhealthy_threshold`][HealthCheck::unhealthy_threshold], but will not result in the host being considered immediately unhealthy. Ranges follow half-open semantics of [`I64Range`]. The start and end of each range are required.
    Only statuses in the range [100, 600) are allowed. The [`expected_statuses`][Self::expected_statuses] field takes precedence for any range overlaps with this field i.e. if status code 200 is both retriable and expected, a 200 response will be considered a successful health check. By default all responses not in [`expected_statuses`][Self::expected_statuses] will result in the host being considered immediately unhealthy i.e. if status code 200 is expected and there are no configured retriable statuses, any non-200 response will result in the host being marked unhealthy.
    */
    retriable_statuses: Vec<I64Range>,

    /// Use specified application protocol for health checks.
    // [(validate.rules).enum = {defined_only: true}]
    codec_client_type: CodecClientType,

    /**
    An optional service name parameter which is used to validate the identity of the health checked cluster using a [`StringMatcher`][crate::types::matcher::string::StringMatcher]. See the :ref:`architecture overview <arch_overview_health_checking_identity>` for more information.
    */
    service_name_matcher: StringMatcher,

    /**
    HTTP Method that will be used for health checking, default is "GET".
    GET, HEAD, POST, PUT, DELETE, OPTIONS, TRACE, PATCH methods are supported, but making request body is not supported.
    CONNECT method is disallowed because it is not appropriate for health check request.
    If a non-200 response is expected by the method, it needs to be set in [`expected_statuses`][Self::expected_statuses].

    [(validate.rules).enum = {defined_only: true not_in: 6}]
    */
    method: RequestMethod
}
