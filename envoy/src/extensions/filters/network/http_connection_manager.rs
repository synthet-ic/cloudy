/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/extensions/filters/network/http_connection_manager/v3/http_connection_manager.proto>
*/

type Any = String;

use std::{
    num::NonZeroU32,
    time::Duration
};

use crate::{
    config::{
        accesslog::{AccessLog, AccessLogFilter},
        core::{
            address::CIDRRange,
            base::{DataSource, HeaderValueOption},
            config_source::{ConfigSource, ExtensionConfigSource},
            extension::TypedExtensionConfig,
            protocol::{HTTPProtocolOptions, HTTP1ProtocolOptions, HTTP2ProtocolOptions, HTTP3ProtocolOptions, SchemeHeaderTransformation},
            substitution_format_string::SubstitutionFormatString
        },
        route::{
            route::RouteConfiguration,
            scoped_route::ScopedRouteConfiguration
        },
        trace::http_tracer::HTTP
    },
    types::{
        http::path_transformation::PathTransformation,
        tracing::custom_tag::CustomTag,
        percent::Percent
    }
};

pub struct HTTPConnectionManager {  
    /// Supplies the type of codec that the connection manager should use.
    // [(validate.rules).enum = {defined_only: true}];
    codec_type: CodecType,
  
    /**
    The human readable prefix to use when emitting statistics for the connection manager. See the :ref:`statistics documentation <config_http_conn_man_stats>` for more information.

    [(validate.rules).String = {min_len: 1}];
    */
    stat_prefix: String,
  
    route_specifier: RouteSpecifier,
  
    /**
    A list of individual HTTP filters that make up the filter chain for
    requests made to the connection manager. :ref:`Order matters <arch_overview_http_filters_ordering>`
    as the filters are processed sequentially as request events happen.
    */
    http_filters: Vec<HTTPFilter>,
  
    /**
    Whether the connection manager manipulates the :ref:`config_http_conn_man_headers_user-agent`
    and :ref:`config_http_conn_man_headers_downstream-service-cluster` headers. See the linked
    documentation for more information. Defaults to false.
    */
    add_user_agent: bool,
  
    /**
    Presence of the object defines whether the connection manager
    emits :ref:`tracing <arch_overview_tracing>` data to the [configured tracing provider][crate::config::trace::http_tracer::Tracing].
    */
    tracing: Tracing,
  
    /**
    Additional settings for HTTP requests handled by the connection manager. These will be applicable to both HTTP1 and HTTP2 requests.
    */
    // [(udpa.annotations.security).configure_for_untrusted_downstream = true];
    common_http_protocol_options: HTTPProtocolOptions,
  
    /**
    Additional HTTP/1 settings that are passed to the HTTP/1 codec.
    
    > TODO: The following fields are ignored when the [`header validation configuration`][Self::typed_header_validation_config] is present:
    > 1. [`allow_chunked_length`][HTTP1ProtocolOptions::allow_chunked_length]
    */
    http_protocol_options: HTTP1ProtocolOptions,
  
    /// Additional HTTP/2 settings that are passed directly to the HTTP/2 codec.
    // [(udpa.annotations.security).configure_for_untrusted_downstream = true];
    http2_protocol_options: HTTP2ProtocolOptions,
  
    /// Additional HTTP/3 settings that are passed directly to the HTTP/3 codec.
    http3_protocol_options: HTTP3ProtocolOptions,
  
    /// An optional override that the connection manager will write to the server header in responses. If not set, the default is `envoy`.
    // [(validate.rules).String = {well_known_regex: HTTP_HEADER_VALUE strict: false}];
    server_name: String,
        
  
    /**
    Defines the action to be applied to the Server header on the response path.
    By default, Envoy will overwrite the header with the value specified in `server_name`.
    */
    // [(validate.rules).enum = {defined_only: true}];
    server_header_transformation: ServerHeaderTransformation,
        
  
    /**
    Allows for explicit transformation of the :scheme header on the request path.
    If not set, Envoy's default :ref:`scheme  <config_http_conn_man_headers_scheme>` handling applies.
    */
    scheme_header_transformation: SchemeHeaderTransformation,
  
    /**
    The maximum request headers size for incoming connections.
    If unconfigured, the default max request headers allowed is 60 KiB.
    Requests that exceed this limit will receive a 431 response.
    */
    // [(validate.rules).u32 = {lte: 8192}];
    max_request_headers_kb: NonZeroU32,
  
    /**
    The stream idle timeout for connections managed by the connection manager.
    If not specified, this defaults to 5 minutes. The default value was selected so as not to interfere with any smaller configured timeouts that may have existed in configurations prior to the introduction of this feature, while introducing robustness to TCP connections that terminate without a FIN.

    This idle timeout applies to new streams and is overridable by the
    [`route-level idle_timeout][crate::config::route::route_components::RouteAction::idle_timeout]. Even on a stream in which the override applies, prior to receipt of the initial request headers, the [`stream_idle_timeout`][Self::stream_idle_timeout] applies. Each time an encode/decode event for headers or data is processed for the stream, the timer will be reset. If the timeout fires, the stream is terminated with a 408 Request Timeout error code if no upstream response header has been received, otherwise a stream reset occurs.

    This timeout also specifies the amount of time that Envoy will wait for the peer to open enough window to write any remaining stream data once the entirety of stream data (local end stream is true) has been buffered pending available window. In other words, this timeout defends against a peer that does not release enough window to completely write the stream, even though all data has been proxied within available flow control windows. If the timeout is hit in this case, the :ref:`tx_flush_timeout <config_http_conn_man_stats_per_codec>` counter will be incremented. Note that [`max_stream_duration`][HTTPProtocolOptions::max_stream_duration] does not apply to this corner case.

    If the :ref:`overload action <config_overload_manager_overload_actions>` "envoy.overload_actions.reduce_timeouts" is configured, this timeout is scaled according to the value for [`HTTPDownstreamConnectionIdle`][crate::config::overload::TimerType::HTTPDownstreamConnectionIdle].

    Note that it is possible to idle timeout even if the wire traffic for a stream is non-idle, due to the granularity of events presented to the connection manager. For example, while receiving very large request headers, it may be the case that there is traffic regularly arriving on the wire while the connection manage is only able to observe the end-of-headers event, hence the stream may still idle timeout.

    A value of 0 will completely disable the connection manager stream idle
    timeout, although per-route idle timeout overrides will continue to apply.

    [(udpa.annotations.security).configure_for_untrusted_downstream = true];
    */
    stream_idle_timeout: Duration,
  
    /**
    The amount of time that Envoy will wait for the entire request to be received.
    The timer is activated when the request is initiated, and is disarmed when the last byte of the request is sent upstream (i.e. all decoding filters have processed the request), OR when the response is initiated. If not specified or set to 0, this timeout is disabled.

    [(udpa.annotations.security).configure_for_untrusted_downstream = true];
    */
    request_timeout: Duration,
  
    /**
    The amount of time that Envoy will wait for the request headers to be received. The timer is activated when the first byte of the headers is received, and is disarmed when the last byte of the headers has been received. If not specified or set to 0, this timeout is disabled.

    [
      (validate.rules).duration = {gte {}},
      (udpa.annotations.security).configure_for_untrusted_downstream = true
    ];
    */
    request_headers_timeout: Duration,
  
    /**
    The time that Envoy will wait between sending an HTTP/2 “shutdown notification” (GOAWAY frame with max stream ID) and a final GOAWAY frame.
    This is used so that Envoy provides a grace period for new streams that race with the final GOAWAY frame. During this grace period, Envoy will continue to accept new streams. After the grace period, a final GOAWAY frame is sent and Envoy will start refusing new streams. Draining occurs both when a connection hits the idle timeout or during general server draining. The default grace period is 5000 milliseconds (5 seconds) if this option is not specified.
    */
    drain_timeout: Duration,
  
    /**
    The delayed close timeout is for downstream connections managed by the HTTP connection manager.
    It is defined as a grace period after connection close processing has been locally initiated during which Envoy will wait for the peer to close (i.e., a TCP FIN/RST is received by Envoy from the downstream connection) prior to Envoy closing the socket associated with that
    connection.
    NOTE: This timeout is enforced even when the socket associated with the downstream connection is pending a flush of the write buffer. However, any progress made writing data to the socket will restart the timer associated with this timeout. This means that the total grace period for a socket in this state will be <total_time_waiting_for_write_buffer_flushes>+<delayed_close_timeout>.

    Delaying Envoy's connection close and giving the peer the opportunity to initiate the close sequence mitigates a race condition that exists when downstream clients do not drain/process data in a connection's receive buffer after a remote close has been detected via a socket write(). This race leads to such clients failing to process the response code sent by Envoy, which could result in erroneous downstream processing.

    If the timeout triggers, Envoy will close the connection's socket.

    The default timeout is 1000 ms if this option is not specified.

    > NOTE: To be useful in avoiding the race condition described above, this timeout must be set to *at least* `<max round trip time expected between clients and Envoy>+<100ms to account for a reasonable "worst" case processing time for a full iteration of Envoy's event loop>`.

    > WARNING: A value of 0 will completely disable delayed close processing. When disabled, the downstream connection's socket will be closed immediately after the write flush is completed or will never close if the write flush does not complete.
    */
    delayed_close_timeout: Duration,
  
    /**
    Configuration for :ref:`HTTP access logs <arch_overview_access_logs>` emitted by the connection manager.
    */
    access_log: AccessLog,
  
    /**
    If set to true, the connection manager will use the real remote address of the client connection when determining internal versus external origin and manipulating various headers. If set to false or absent, the connection manager will use the :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header. See the documentation for :ref:`config_http_conn_man_headers_x-forwarded-for`, :ref:`config_http_conn_man_headers_x-envoy-internal`, and :ref:`config_http_conn_man_headers_x-envoy-external-address` for more information.

    [(udpa.annotations.security).configure_for_untrusted_downstream = true];
    */
    use_remote_address: bool,
  
    /**
    The number of additional ingress proxy hops from the right side of the :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header to trust when determining the origin client's IP address. The default is zero if this option is not specified. See the documentation for :ref:`config_http_conn_man_headers_x-forwarded-for` for more information.
    */
    xff_num_trusted_hops: u32,
  
    /**
    The configuration for the original IP detection extensions.

    When configured the extensions will be called along with the request headers and information about the downstream connection, such as the directly connected address.
    Each extension will then use these parameters to decide the request's effective remote address.
    If an extension fails to detect the original IP address and isn't configured to reject the request, the HCM will try the remaining extensions until one succeeds or rejects the request. If the request isn't rejected nor any extension succeeds, the HCM will fallback to using the remote address.

    > WARNING: Extensions cannot be used in conjunction with [`use_remote_address][Self::use_remote_address] nor [xff_num_trusted_hops][Self::xff_num_trusted_hops].

    [#extension-category: envoy.http.original_ip_detection]
    */
    original_ip_detection_extensions: Vec<TypedExtensionConfig>,
  
    /**
    Configures what network addresses are considered internal for stats and header sanitation purposes. If unspecified, only RFC1918 IP addresses will be considered internal.
    See the documentation for :ref:`config_http_conn_man_headers_x-envoy-internal` for more information about internal/external addresses.
    */
    internal_address_config: InternalAddressConfig,
  
    /**
    If set, Envoy will not append the remote address to the
    :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header. This may be used in conjunction with HTTP filters that explicitly manipulate XFF after the HTTP connection manager has mutated the request headers. While [`use_remote_address`][Self::use_remote_address] will also suppress XFF addition, it has consequences for logging and other Envoy uses of the remote address, so `skip_xff_append` should be used when only an elision of XFF addition is intended.
    */
    skip_xff_append: bool,
  
    /**
    Via header value to append to request and response headers. If this is empty, no via header will be appended.

    [(validate.rules).String = {well_known_regex: HTTP_HEADER_VALUE strict: false}];
    */
    via: String,
  
    /**
    Whether the connection manager will generate the :ref:`x-request-id
    <config_http_conn_man_headers_x-request-id>` header if it does not exist. This defaults to `true`. Generating a random UUID4 is expensive so in high throughput scenarios where this feature is not desired it can be disabled.
    */
    generate_request_id: bool,
  
    /**
    Whether the connection manager will keep the :ref:`x-request-id
    <config_http_conn_man_headers_x-request-id>` header if passed for a request that is edge (Edge request is the request from external clients to front Envoy) and not reset it, which is the current Envoy behaviour. This defaults to `false`.
    */
    preserve_external_request_id: bool,
  
    /**
    If set, Envoy will always set :ref:`x-request-id <config_http_conn_man_headers_x-request-id>` header in response.
    If this is false or not set, the request ID is returned in responses only if tracing is forced using :ref:`x-envoy-force-trace <config_http_conn_man_headers_x-envoy-force-trace>` header.
    */
    always_set_request_id_in_response: bool,
  
    /**
    How to handle the :ref:`config_http_conn_man_headers_x-forwarded-client-cert` (XFCC) HTTP header.

    [(validate.rules).enum = {defined_only: true}];
    */
    forward_client_cert_details: ForwardClientCertDetails,
  
    /**
    This field is valid only when [`forward_client_cert_details`][Self::forward_client_cert_details] is APPEND_FORWARD or SANITIZE_SET and the client connection is mTLS. It specifies the fields in the client certificate to be forwarded. Note that in the :ref:`config_http_conn_man_headers_x-forwarded-client-cert` header, `Hash` is always set, and `By` is always set when the client certificate presents the URI type Subject Alternative Name value.
    */
    set_current_client_cert_details: SetCurrentClientCertDetails,
  
    /**
    If proxy_100_continue is true, Envoy will proxy incoming "Expect: 100-continue" headers upstream, and forward "100 Continue" responses downstream. If this is false or not set, Envoy will instead strip the "Expect: 100-continue" header, and send a "100 Continue" response itself.
    */
    proxy_100_continue: bool,
  
    /**
    If [`use_remote_address`][Self::use_remote_address] is `true` and `represent_ipv4_remote_address_as_ipv4_mapped_ipv6` is `true` and the remote address is an IPv4 address, the address will be mapped to IPv6 before it is appended to `x-forwarded-for`.
    This is useful for testing compatibility of upstream services that parse the header value. For example, 50.0.0.1 is represented as ::FFFF:50.0.0.1. See [IPv4-Mapped IPv6 Addresses](https://www.rfc-editor.org/rfc/rfc4291#section-2.5.5.2) for details. This will also affect the
    :ref:`config_http_conn_man_headers_x-envoy-external-address` header. See
    :ref:`http_connection_manager.represent_ipv4_remote_address_as_ipv4_mapped_ipv6
    <config_http_conn_man_runtime_represent_ipv4_remote_address_as_ipv4_mapped_ipv6>` for runtime
    control.
    [#not-implemented-hide:]
    */
    represent_ipv4_remote_address_as_ipv4_mapped_ipv6: bool,
  
    upgrade_configs: Vec<UpgradeConfig>,
  
    /**
    Should paths be normalised according to RFC 3986 before any processing of requests by HTTP filters or routing? This affects the upstream `:path` header as well. For paths that fail this check, Envoy will respond with 400 to paths that are malformed. This defaults to false currently but will default `true` in the future. When not specified, this value may be overridden by the runtime variable
    :ref:`http_connection_manager.normalise_path <config_http_conn_man_runtime_normalise_path>`.
    See [Normalisation and Comparison](https://www.rfc-editor.org/rfc/rfc3986#section-6) for details of normalisation.
    Note that Envoy does not perform [case normalisation](https://www.rfc-editor.org/rfc/rfc3986#section-6.2.2.1)
    
    > #comment:TODO: This field is ignored when the [header validation configuration][Self::typed_header_validation_config]
    is present.
    */
    normalise_path: bool,
  
    /**
    Determines if adjacent slashes in the path are merged into one before any processing of requests by HTTP filters or routing. This affects the upstream `:path` header as well. Without setting this option, incoming requests with path `//dir///file` will not match against route with `prefix` match set to `/dir`. Defaults to `false`. Note that slash merging is not part of [HTTP spec](https://www.rfc-editor.org/rfc/rfc3986) and is provided for convenience.

    > #comment:TODO: This field is ignored when the [header validation configuration][Self::typed_header_validation_config] is present.
    */
    merge_slashes: bool,
  
    /**
    Action to take when request URL path contains escaped slash sequences (%2F, %2f, %5C and %5c).
    The default value can be overridden by the :ref:`http_connection_manager.path_with_escaped_slashes_action <config_http_conn_man_runtime_path_with_escaped_slashes_action>` runtime variable.
    The :ref:`http_connection_manager.path_with_escaped_slashes_action_sampling<config_http_conn_man_runtime_path_with_escaped_slashes_action_enabled>` runtime variable can be used to apply the action to a portion of all requests.

    > #comment:TODO: This field is ignored when the
    [header validation configuration][Self::typed_header_validation_config] is present.
    */
    path_with_escaped_slashes_action: PathWithEscapedSlashesAction,
  
    /**
    The configuration of the request ID extension. This includes operations such as generation, validation, and associated tracing operations. If empty, the [`UUIDRequestIdConfig`][crate::extensions::request_id::uuid::UUIDRequestIdConfig] default extension is used with default parameters. See the documentation for that extension for details on what it does. Customizing the configuration for the default extension can be achieved by configuring it explicitly here. For example, to disable trace reason packing, the following configuration can be used:

    .. validated-code-block:: yaml
      :type-name: envoy.extensions.filters.network.http_connection_manager::RequestIdExtension

      typed_config:
        "@type": type.googleapis.com/envoy.extensions.request_id.uuid::UuidRequestIdConfig
        pack_trace_reason: false

    [#extension-category: envoy.request_id]
    */
    request_id_extension: RequestIdExtension,
  
    /**
    The configuration to customize local reply returned by Envoy. It can customise status code, body text and response content type. If not specified, status code and text body are hard coded in Envoy, the response content type is plain text.
    */
    local_reply_config: LocalReplyConfig,
  
    /**
    Determines if the port part should be removed from host/authority header before any processing of request by HTTP filters or routing. The port would be removed only if it is equal to the [listener's][crate::config::listener::listener::Listener::address] local port. This affects the upstream host header unless the method is CONNECT in which case if no filter adds a port the original port will be restored before headers are
    sent upstream.
    Without setting this option, incoming requests with host `example:443` will not match against route with [`domains`][crate::config::route::route_components::VirtualHost::domains] match set to `example`. Defaults to `false`. Note that port removal is not part of [HTTP spec](https://www.rfc-editor.org/rfc/rfc3986) and is provided for convenience.
    Only one of `strip_matching_host_port` or `strip_any_host_port` can be set.

    [(udpa.annotations.field_migrate).oneof_promotion = "strip_port_mode"];
    */
    strip_matching_host_port: bool,
  
    strip_port_mode: StripPortMode,
  
    /**
    Governs Envoy's behaviour when receiving invalid HTTP from downstream.
    If this option is false (default), Envoy will err on the conservative side handling HTTP
    errors, terminating both HTTP/1.1 and HTTP/2 connections when receiving an invalid request.
    If this option is set to true, Envoy will be more permissive, only resetting the invalid
    stream in the case of HTTP/2 and leaving the connection open where possible (if the entire
    request is read for HTTP/1.1)
    In general this should be true for deployments receiving trusted traffic (L2 Envoys,
    company-internal mesh) and false when receiving untrusted traffic (edge deployments).

    If different behaviours for invalid_http_message for HTTP/1 and HTTP/2 are
    desired, one should use the new HTTP/1 option [override_stream_error_on_invalid_http_message][crate::config::core::HTTP1ProtocolOptions.override_stream_error_on_invalid_http_message] or the new HTTP/2 option
    [override_stream_error_on_invalid_http_message][crate::config::core::HTTP2ProtocolOptions.override_stream_error_on_invalid_http_message]
    `not` the deprecated but similarly named [stream_error_on_invalid_http_messaging][crate::config::core::HTTP2ProtocolOptions.stream_error_on_invalid_http_messaging]
    */
    stream_error_on_invalid_http_message: bool,
  
    /**
    Path normalisation configuration. This includes configurations for transformations (e.g. RFC 3986 normalisation or merge adjacent slashes) and the policy to apply them. The policy determines whether transformations affect the forwarded `:path` header. [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986) path normalisation is enabled by default and the default policy is that the normalised header will be forwarded. See [PathNormalisationOptions][crate::extensions::filters::network::http_connection_manager::PathNormalisationOptions] for details.
    */
    path_normalisation_options: PathNormalisationOptions,
  
    /**
    Determines if trailing dot of the host should be removed from host/authority header before any processing of request by HTTP filters or routing.
    This affects the upstream host header.
    Without setting this option, incoming requests with host `example.com.` will not match against route with [`domains`][crate::config::route::route_components::VirtualHost::domains] match set to `example.com`. Defaults to `false`.
    When the incoming request contains a host/authority header that includes a port number, setting this option will strip a trailing dot, if present, from the host section, leaving the port as is (e.g. host value `example.com.:443` will be updated to `example.com:443`).
    */
    strip_trailing_host_dot: bool,
  
    /**
    Proxy-Status HTTP response header configuration.
    If this config is set, the Proxy-Status HTTP response header field is
    populated. By default, it is not.
    */
    proxy_status_config: ProxyStatusConfig,
  
    /**
    Configuration options for Header Validation (UHV).
    UHV is an extensible mechanism for checking validity of HTTP requests as well as providing
    normalisation for request attributes, such as URI path.
    If the typed_header_validation_config is present it overrides the following options:
    `normalise_path`, `merge_slashes`, `path_with_escaped_slashes_action`
    `http_protocol_options.allow_chunked_length`.

    The default UHV checks the following:

    #. HTTP/1 header map validity according to `RFC 7230 section 3.2<https://datatracker.ietf.org/doc/html/rfc7230#section-3.2>`_
    #. Syntax of HTTP/1 request target URI and response status
    #. HTTP/2 header map validity according to `RFC 7540 section 8.1.2<https://datatracker.ietf.org/doc/html/rfc7540#section-8.1.2`_
    #. Syntax of HTTP/2 pseudo headers
    #. HTTP/3 header map validity according to `RFC 9114 section 4.3 <https://www.rfc-editor.org/rfc/rfc9114.html>`_
    #. Syntax of HTTP/3 pseudo headers
    #. Syntax of `Content-Length` and `Transfer-Encoding`
    #. Validation of HTTP/1 requests with both `Content-Length` and `Transfer-Encoding` headers
    #. Normalisation of the URI path according to `Normalisation and Comparison <https://datatracker.ietf.org/doc/html/rfc3986#section-6>`_
       without `case normalisation <https://datatracker.ietf.org/doc/html/rfc3986#section-6.2.2.1>`_

    [#not-implemented-hide:]
    [#extension-category: envoy.http.header_validators]
    */
    typed_header_validation_config: TypedExtensionConfig,
  
    /**
    Append the `x-forwarded-port` header with the port value client used to connect to Envoy. It will be ignored if the `x-forwarded-port` header has been set by any trusted proxy in front of Envoy.
    */
    append_x_forwarded_port: bool,
}

pub enum RouteSpecifier {
    // option (validate.required) = true;

    /// The connection manager’s route table will be dynamically loaded via the RDS API.
    RDS(RDS),

    /// The route table for the connection manager is static and is specified in this property.
    RouteConfiguration(RouteConfiguration),

    /**
    A route table will be dynamically assigned to each request based on request attributes (e.g., the value of a header). The "routing scopes" (i.e., route tables) and "scope keys" are specified in this message.
    */
    ScopedRoutes(ScopedRoutes),
}

pub enum StripPortMode {
    /**
    Determines if the port part should be removed from host/authority header before any processing of request by HTTP filters or routing.
    This affects the upstream host header unless the method is CONNECT in which case if no filter adds a port the original port will be restored before headers are sent upstream.
    Without setting this option, incoming requests with host `example:443` will not match against route with [`domains`][crate::config::route::route_components::VirtualHost::domains] match set to `example`. Defaults to `false`. Note that port removal is not part
    of [HTTP spec](https://www.rfc-editor.org/rfc/rfc3986) and is provided for convenience.
    Only one of `strip_matching_host_port` or `strip_any_host_port` can be set.
    */
    StripAnyHostPort(bool),
}

pub enum CodecType {
    /**
    For every new connection, the connection manager will determine which
    codec to use. This mode supports both ALPN for TLS listeners as well as
    protocol inference for plaintext listeners. If ALPN data is available, it
    is preferred, otherwise protocol inference is used. In almost all cases,
    this is the right option to choose for this setting.
    */
    Auto,

    /// The connection manager will assume that the client is speaking HTTP/1.1.
    HTTP1,

    /**
    The connection manager will assume that the client is speaking HTTP/2
    (Envoy does not require HTTP/2 to take place over TLS or to use ALPN.
    Prior knowledge is allowed).
    */
    HTTP2,

    /**
    [#not-implemented-hide:] QUIC implementation is not production ready yet. Use this enum with
    caution to prevent accidental execution of QUIC code. I.e. `!= HTTP2` is no longer sufficient
    to distinguish HTTP1 and HTTP2 traffic.
    */
    HTTP3,
}

pub enum ServerHeaderTransformation {
    /// Overwrite any Server header with the contents of server_name.
    Overwrite,

    /**
    If no Server header is present, append Server server_name
    If a Server header is present, pass it through.
    */
    AppendIfAbsent,

    /**
    Pass through the value of the server header, and do not append a header if none is present.
    */
    PassThrough
}

/**
How to handle the :ref:`config_http_conn_man_headers_x-forwarded-client-cert` (XFCC) HTTP
header.
*/
pub enum ForwardClientCertDetails {
    /// Do not send the XFCC header to the next hop. This is the default value.
    Sanitise,

    /**
    When the client connection is mTLS (Mutual TLS), forward the XFCC header in the request.
    */
    ForwardOnly,

    /**
    When the client connection is mTLS, append the client certificate
    information to the request’s XFCC header and forward it.
    */
    AppendForward,

    /**
    When the client connection is mTLS, reset the XFCC header with the client certificate information and send it to the next hop.
    */
    SanitiseSet,

    /**
    Always forward the XFCC header in the request, regardless of whether the client connection is mTLS.
    */
    AlwaysForwardOnly
}

/**
Determines the action for request that contain %2F, %2f, %5C or %5c sequences in the URI path.
This operation occurs before URL normalisation and the merge slashes transformations if they were enabled.
*/
pub enum PathWithEscapedSlashesAction {
    /**
    Default behaviour specific to implementation (i.e. Envoy) of this configuration option.
    Envoy, by default, takes the KEEP_UNCHANGED action.
    NOTE: the implementation may change the default behaviour at-will.
    */
    ImplementationSpecificDefault,

    /// Keep escaped slashes.
    KeepUnchanged,

    /**
    Reject client request with the 400 status. gRPC requests will be rejected with the INTERNAL (13) error code.
    The "httpN.downstream_rq_failed_path_normalisation" counter is incremented for each rejected request.
    */
    RejectRequest,

    /**
    Unescape %2F and %5C sequences and redirect request to the new path if these sequences were present.
    Redirect occurs after path normalisation and merge slashes transformations if they were configured.
    NOTE: gRPC requests will be rejected with the INTERNAL (13) error code.
    This option minimizes possibility of path confusion exploits by forcing request with unescaped slashes to traverse all parties: downstream client, intermediate proxies, Envoy and upstream server.
    The "httpN.downstream_rq_redirected_with_normalised_path" counter is incremented for each
    redirected request.
    */
    UnescapeAndRedirect,

    /**
    Unescape %2F and %5C sequences.
    Note: this option should not be enabled if intermediaries perform path based access control as
    it may lead to path confusion vulnerabilities.
    */
    UnescapeAndForward,
}

pub struct Tracing {
    /**
    Target percentage of requests managed by this HTTP connection manager that will be force
    traced if the :ref:`x-client-trace-id <config_http_conn_man_headers_x-client-trace-id>`
    header is set. This field is a direct analog for the runtime variable
    'tracing.client_sampling' in the :ref:`HTTP Connection Manager
    <config_http_conn_man_runtime>`.
    Default: 100%
    */
    client_sampling: Percent,

    /**
    Target percentage of requests managed by this HTTP connection manager that will be randomly
    selected for trace generation, if not requested by the client or not forced. This field is
    a direct analog for the runtime variable 'tracing.random_sampling' in the
    :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
    Default: 100%
    */
    random_sampling: Percent,

    /**
    Target percentage of requests managed by this HTTP connection manager that will be traced after all other sampling checks have been applied (client-directed, force tracing, random
    sampling). This field functions as an upper limit on the total configured sampling rate. For
    instance, setting client_sampling to 100% but overall_sampling to 1% will result in only 1%
    of client requests with the appropriate headers to be force traced. This field is a direct
    analog for the runtime variable 'tracing.global_enabled' in the
    :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
    Default: 100%
    */
    overall_sampling: Percent,

    /**
    Whether to annotate spans with additional data. If true, spans will include logs for stream events.
    */
    verbose: bool,

    /**
    Maximum length of the request path to extract and include in the HTTPUrl tag. Used to truncate lengthy request paths to meet the needs of a tracing backend. Default: `256`.
    */
    max_path_tag_length: u32,

    /// A list of custom tags with unique tag name to create tags for the active span.
    custom_tags: Vec<CustomTag>,

    /**
    Configuration for an external tracing provider.
    If not specified, no tracing will be performed.

    > attention: Please be aware that `envoy.tracers.opencensus` provider can only be configured once in Envoy lifetime.
    > Any attempts to reconfigure it or to use different configurations for different HCM filters will be rejected.
    > Such a constraint is inherent to OpenCensus itself. It cannot be overcome without changes on OpenCensus side.
    */
    provider: HTTP,
}

pub enum OperationName {
    /// The HTTP listener is used for ingress/incoming requests.
    Ingress,

    /// The HTTP listener is used for egress/outgoing requests.
    Egress,
}

pub struct InternalAddressConfig {
    /// Whether unix socket addresses should be considered internal.
    unix_sockets: bool,

    /**
    List of CIDR ranges that are treated as internal. If unset, then RFC1918 / RFC4193
    IP addresses will be considered internal.
    */
    cidr_ranges: Vec<CIDRRange>,
}

pub struct SetCurrentClientCertDetails {
    /// Whether to forward the subject of the client cert. Defaults to false.
    subject: bool,

    /**
    Whether to forward the entire client cert in URL encoded PEM format. This will appear in the
    XFCC header comma separated from other values with the value Cert="PEM".
    Defaults to false.
    */
    cert: bool,

    /**
    Whether to forward the entire client cert chain (including the leaf cert) in URL encoded PEM format. This will appear in the XFCC header comma separated from other values with the value Chain="PEM". Defaults to `false.
    */
    chain: bool,

    /**
    Whether to forward the DNS type Subject Alternative Names of the client cert. Defaults to `false`.
    */
    dns: bool,

    /**
    Whether to forward the URI type Subject Alternative Name of the client cert. Defaults to `false`.
    */
    uri: bool,
}

/**
The configuration for HTTP upgrades.
For each upgrade type desired, an UpgradeConfig must be added.

> WARNING: The current implementation of upgrade headers does not handle multi-valued upgrade headers. Support for multi-valued headers may be added in the future if needed.

> WARNING: The current implementation of upgrade headers does not work with HTTP/2 upstreams.
*/
pub struct UpgradeConfig {
    /**
    The case-insensitive name of this upgrade, e.g. "websocket".
    For each upgrade type present in upgrade_configs, requests with
    Upgrade: `upgrade_type` will be proxied upstream.
    */
    upgrade_type: String,

    /**
    If present, this represents the filter chain which will be created for this type of upgrade. If no filters are present, the filter chain for
    HTTP connections will be used for this upgrade type.
    */
    filters: Vec<HTTPFilter>,

    /**
    Determines if upgrades are enabled or disabled by default. Defaults to true.
    This can be overridden on a per-route basis with [cluster][crate::config::route::route_components::RouteAction::upgrade_configs] as documented in the :ref:`upgrade documentation <arch_overview_upgrades>`.
    */
    enabled: bool,
}

/**
Transformations that apply to path headers. Transformations are applied before any processing of requests by HTTP filters, routing, and matching. Only the normalised path will be visible internally if a transformation is enabled. Any path rewrites that the router performs (e.g. [regex_rewrite][crate::config::route::route_components::RouteAction::regex_rewrite] or [`prefix_rewrite`][crate::config::route::route_components::RouteAction::prefix_rewrite]) will apply to the `:path` header destined for the upstream.

> Note: access logging and tracing will show the original `:path` header.
*/
pub struct PathNormalisationOptions {
    /**
    Normalisation applies internally before any processing of requests by HTTP filters, routing, and matching *and* will affect the forwarded `:path` header. Defaults to [`NormalisePathRFC3986`][crate::types::http::PathTransformation.Operation.NormalisePathRFC3986]. When not specified, this value may be overridden by the runtime variable :ref:`http_connection_manager.normalise_path<config_http_conn_man_runtime_normalise_path>`.
    Envoy will respond with 400 to paths that are malformed (e.g. for paths that fail [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986) normalisation due to disallowed characters.)
    */
    forwarding_transformation: PathTransformation,

    /**
    Normalisation only applies internally before any processing of requests by HTTP filters, routing, and matching. These will be applied after full transformation is applied. The `:path` header before this transformation will be restored in the router filter and sent upstream unless it was mutated by a filter. Defaults to no transformations.
    Multiple actions can be applied in the same Transformation, forming a sequential pipeline. The transformations will be performed in the order that they appear. Envoy will respond with 400 to paths that are malformed (e.g. for paths that fail RFC 3986 normalisation due to disallowed characters.)
    */
    http_filter_transformation: PathTransformation,
}

/**
Configures the manner in which the Proxy-Status HTTP response header is populated.

See the [Proxy-Status RFC](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-proxy-status-08).

> #comment:TODO: Update this with the non-draft URL when finalised.

The Proxy-Status header is a string of the form:

`"<server_name>; error=<error_type>; details=<details>"`
*/
pub struct ProxyStatusConfig {
    /**
    If `true`, the details field of the Proxy-Status header is not populated with `stream_info.response_code_details`.
    This value defaults to `false`, i.e. the `details` field is populated by default.
    */
    remove_details: bool,

    /**
    If `true`, the details field of the Proxy-Status header will not contain
    connection termination details. This value defaults to `false`, i.e. the `details` field will contain connection termination details by default.
    */
    remove_connection_termination_details: bool,

    /**
    If `true`, the details field of the Proxy-Status header will not contain an enumeration of the Envoy ResponseFlags. This value defaults to `false`, i.e. the `details` field will contain a list of ResponseFlags by default.
    */
    remove_response_flags: bool,

    /**
    If `true`, overwrites the existing Status header with the response code recommended by the Proxy-Status spec.
    This value defaults to `false`, i.e. the HTTP response code is not overwritten.
    */
    set_recommended_response_code: bool,

    /**
    The name of the proxy as it appears at the start of the Proxy-Status header.

    If neither of these values are set, this value defaults to `server_name`, which itself defaults to "envoy".
    */
    proxy_name: ProxyName
}

pub enum ProxyName {
    /// If `use_node_id` is set, Proxy-Status headers will use the Envoy's node ID as the name of the proxy.
    UseNodeId(bool),

    /// If `literal_proxy_name` is set, Proxy-Status headers will use this value as the name of the proxy.
    Literal(String),
}
  
/// The configuration to customize local reply returned by Envoy.
pub struct LocalReplyConfig {
    /**
    Configuration of list of mappers which allows to filter and change local response.
    The mappers will be checked by the specified order until one is matched.
    */
    mappers: Vec<ResponseMapper>,
  
    /**
    The configuration to form response body from the :ref:`command operators <config_access_log_command_operators>` and to specify response content type as one of: plain/text or application/json.

    Example one: "plain/text" `body_format`.

    .. validated-code-block:: yaml
      :type-name: envoy.config.core::SubstitutionFormatString

      text_format: "%LOCAL_REPLY_BODY%:%RESPONSE_CODE%:path=%REQ(:path)%\n"

    The following response body in "plain/text" format will be generated for a request with local reply body of "upstream connection error", response_code=503 and path=/foo.

    ```text
    upstream connect error:503:path=/foo
    ```
    
    Example two: "application/json" `body_format`.

    .. validated-code-block:: yaml
      :type-name: envoy.config.core::SubstitutionFormatString

      json_format:
        status: "%RESPONSE_CODE%"
        message: "%LOCAL_REPLY_BODY%"
        path: "%REQ(:path)%"

    The following response body in "application/json" format would be generated for a request with local reply body of "upstream connection error", response_code=503 and path=/foo.

    ```json
    {
      "status": 503,
      "message": "upstream connection error",
      "path": "/foo"
    }
    ```
    */
    body_format: SubstitutionFormatString,
}
  
/// The configuration to filter and change local response.
pub struct ResponseMapper {
    /// Filter to determine if this mapper should apply.
    // [(validate.rules).message = {required: true}];
    filter: AccessLogFilter,
  
    /// The new response status code if specified.
    // [(validate.rules).u32 = {lt: 600 gte: 200}];
    status_code: u16,
  
    /**
    The new local reply body text if specified. It will be used in the `%LOCAL_REPLY_BODY%` command operator in the `body_format`.
    */
    body: DataSource,
  
    /**
    A per mapper `body_format` to override the [`body_format`][LocalReplyConfig::body_format].
    It will be used when this mapper is matched.
    */
    body_format_override: SubstitutionFormatString,
  
    /// HTTP headers to add to a local reply. This allows the response mapper to append, to add or to override headers of any local reply before it is sent to a downstream client.
    // [(validate.rules).repeated = {max_items: 1000}];
    headers_to_add: Vec<HeaderValueOption>
        
}
  
pub struct RDS {
    /// Configuration source specifier for RDS.
    // [(validate.rules).message = {required: true}];
    config_source: ConfigSource,
  
    /**
    The name of the route configuration. This name will be passed to the RDS
    API. This allows an Envoy configuration with multiple HTTP listeners (and associated HTTP connection manager filters) to use different route configurations.
    */
    route_config_name: String,
}
  
/// This message is used to work around the limitations with 'oneof' and repeated fields.
pub struct ScopedRouteConfigurationsList {
    // [!is_empty()]
    scoped_route_configurations: Vec<ScopedRouteConfiguration>
        
}
  
pub struct ScopedRoutes {  
    /// The name assigned to the scoped routing configuration.
    // [!is_empty()]
    name: String,
  
    /// The algorithm to use for constructing a scope key for each request.
    // [(validate.rules).message = {required: true}];
    scope_key_builder: ScopeKeyBuilder,
  
    /**
    Configuration source specifier for RDS.
    This config source is used to subscribe to `RouteConfiguration` resources specified in `ScopedRouteConfiguration` messages.
    */
    rds_config_source: ConfigSource,
  
    config_specifier: ConfigSpecifier
}

pub enum ConfigSpecifier {
    // option (validate.required) = true;

    /// The set of routing scopes corresponding to the HCM. A scope is assigned to a request by matching a key constructed from the request's attributes according to the algorithm specified by the [`ScopeKeyBuilder`] in this message.
    ScopedRouteConfigurationsList(ScopedRouteConfigurationsList),

    /// The set of routing scopes associated with the HCM will be dynamically loaded via the SRDS API. A scope is assigned to a request by matching a key constructed from the request's attributes according to the algorithm specified by the [`ScopeKeyBuilder`] in this message.
    ScopedRDS(ScopedRDS),
}

/**
Specifies the mechanism for constructing "scope keys" based on HTTP request attributes. These keys are matched against a set of :[`Key`][crate::config::route::scoped_route::Key] objects assembled from [`ScopedRouteConfiguration`][crate::config::route::scoped_route::ScopedRouteConfiguration] messages distributed via SRDS (the Scoped Route Discovery Service) or assigned statically via [`ScopedRouteConfigurationsList`][crate::extensions::filters::network::http_connection_manager::ScopedRouteConfigurationsList].

Upon receiving a request's headers, the Router will build a key using the algorithm specified by this message. This key will be used to look up the routing table (i.e., the [`RouteConfiguration`] [crate::config::route::route::RouteConfiguration]) to use for the request.
*/
pub struct ScopeKeyBuilder {
    /**
    The final(built) scope key consists of the ordered union of these fragments, which are compared in order with the fragments of a [`ScopedRouteConfiguration`][crate::config::route::scoped_route::ScopedRouteConfiguration].
    A missing fragment during comparison will make the key invalid, i.e., the computed key doesn't match any key.
    */
    // [!is_empty()]
    fragments: Vec<FragmentBuilder>
}

/// Specifies the mechanism for constructing key fragments which are composed into scope keys.
pub enum FragmentBuilder {
    // option (validate.required) = true;

    /// Specifies how a header field's value should be extracted.
    HeaderValueExtractor(HeaderValueExtractor),
}

/**
Specifies how the value of a header should be extracted.
The following example maps the structure of a header to the fields in this message.

```text
                <0> <1>   <-- index
    X-Header: a=b;c=d
    |         || |
    |         || \----> <element_separator>
    |         ||
    |         |\----> <element.separator>
    |         |
    |         \----> <element.key>
    |
    \----> <name>

    Each 'a=b' key-value pair constitutes an 'element' of the header field.
```
*/
pub struct HeaderValueExtractor {
    /**
    The name of the header field to extract the value from.

    > NOTE: If the header appears multiple times only the first value is used.
    */
    // [!is_empty()]
    name: String,

    /**
    The element separator (e.g., ';' separates 'a;b;c;d').
    Default: empty string. This causes the entirety of the header field to be extracted.
    If this field is set to an empty string and 'index' is used in the oneof below, 'index' must be set to 0.
    */
    element_separator: String,

    extract_type: ExtractType
}

pub enum ExtractType {
    /**
    Specifies the zero based index of the element to extract.
    Note Envoy concatenates multiple values of the same header key into a comma separated string, the splitting always happens after the concatenation.
    */
    Index(u32),

    /// Specifies the key value pair to extract the value from.
    Element(KVElement),
}

/// Specifies a header field's key value pair to match on.
pub struct KVElement {
    /**
    The separator between key and value (e.g., '=' separates 'k=v;...').
    If an element is an empty string, the element is ignored.
    If an element contains no separator, the whole element is parsed as key and the fragment value is an empty string.
    If there are multiple values for a matched key, the first value is returned.
    */
    // [!is_empty()]
    separator: String,

    /// The key to match on.
    // [!is_empty()]
    key: String,
}

pub struct ScopedRDS {
    /// Configuration source specifier for scoped RDS.
    // [(validate.rules).message = {required: true}];
    scoped_rds_config_source: ConfigSource,
        
  
    /**
    xdstp:// resource locator for scoped RDS collection.
    [#not-implemented-hide:]
    */
    srds_resources_locator: String,
}
  
pub struct HTTPFilter {
    /**
    The name of the filter configuration. It also serves as a resource name in ExtensionConfigDS.
    */
    // [!is_empty()]
    name: String,
  
    config_typ: ConfigType,
  
    /**
    If `true`, clients that do not support this filter may ignore the filter but otherwise accept the config.
    Otherwise, clients that do not support this filter must reject the config.
    This is also same with typed per filter config.
    */
    is_optional: bool,
}

pub enum ConfigType {
    /**
    Filter specific configuration which depends on the filter being instantiated. See the supported filters for further documentation.

    To support configuring a :ref:`match tree <arch_overview_matching_api>`, use an [`ExtensionWithMatcher`][crate::extensions::common.matching::ExtensionWithMatcher]  with the desired HTTP filter.
    [#extension-category: envoy.filters.http]
    */
    TypedConfig(Any),

    /**
    Configuration source specifier for an extension configuration discovery service.
    In case of a failure and without the default configuration, the HTTP listener responds with code 500.
    Extension configs delivered through this mechanism are not expected to require warming (see <https://github.com/envoyproxy/envoy/issues/12061>).

    To support configuring a :ref:`match tree <arch_overview_matching_api>`, use an [`ExtensionWithMatcher`][crate::extensions::common::matching::extension_matcher::ExtensionWithMatcher]
    with the desired HTTP filter. This works for both the default filter configuration as well as for filters provided via the API.
    */
    ConfigDiscovery(ExtensionConfigSource),
}
  
pub struct RequestIdExtension {
    /// Request ID extension specific configuration.
    typed_config: Any,
}
  
/**
[#protodoc-title: Envoy Mobile HTTP connection manager]
HTTP connection manager for use in Envoy mobile.
[#extension: envoy.filters.network.envoy_mobile_http_connection_manager]
*/
pub struct EnvoyMobileHTTPConnectionManager {
    /// The configuration for the underlying HTTPConnectionManager which will be instantiated for Envoy mobile.
    config: HTTPConnectionManager,
}
