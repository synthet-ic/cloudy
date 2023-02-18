/*!
- <https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/route/v3/route_components.proto>
- <https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/route/v3/route_components.proto>
*/

type Any = String;
type Struct = String;

use std::{
    collections::HashMap,
    time::Duration,
};

use xds::types::matcher::Matcher;

use crate::{
    config::core::{
        base::{
            DataSource, HeaderValueOption, Metadata, RoutingPriority,
            RuntimeFractionalPercent
        },
        extension::TypedExtensionConfig,
        proxy_protocol::ProxyProtocolConfig,
    },
    types::{
        matcher::{
            metadata::MetadataMatcher,
            regex::{RegexMatchAndSubstitute, RegexMatcher},
            string::StringMatcher
        },
        metadata::MetadataKey,
        percent::FractionalPercent,
        range::I64Range,
        tracing::custom_tag::CustomTag,
    }
};

/// The top level element in the routing configuration is a virtual host. Each virtual host has a logical name as well as a set of domains that get routed to it based on the incoming request's host header. This allows a single listener to service multiple top level domain path trees. Once a virtual host is selected based on the domain, the routes are processed in order to see which upstream cluster to route to or whether to perform a redirect.
pub struct VirtualHost {
    /// The logical name of the virtual host. This is used when emitting certain statistics but is not relevant for routing.
    // [!is_empty()]
    name: String,

    /**
    A list of domains (host/authority header) that will be matched to this
    virtual host. Wildcard hosts are supported in the suffix or prefix form.

    Domain search order:
     1. Exact domain names: `www.foo.com`.
     2. Suffix domain wildcards: `*.foo.com` or `*-bar.foo.com`.
     3. Prefix domain wildcards: `foo.*` or `foo-*`.
     4. Special wildcard `*` matching any domain.

    > NOTE: The wildcard will not match the empty string.
      e.g. `*-bar.foo.com` will match `baz-bar.foo.com` but not `-bar.foo.com`.
      The longest wildcards match first.
      Only a single virtual host in the entire route configuration can match on `*`. A domain must be unique across all virtual hosts or the config will fail to load.

    Domains cannot contain control characters. This is validated by the well_known_regex HTTP_HEADER_VALUE.
    */
    // [(validate.rules).repeated = {
    //     min_items: 1
    //     items {String {well_known_regex: HTTP_HEADER_VALUE strict: false}}
    // }]
    domains: Vec<String>,

    /*
    The list of routes that will be matched, in order, for incoming requests.
    The first route that matches will be used.
    Only one of this and `matcher` can be specified.
    */
    routes: Vec<Route>,

    /*
    [#next-major-version: This should be included in a oneof with routes wrapped in a message.]
    The match tree to use when resolving route actions for incoming requests. Only one of this and `routes` can be specified.

    */
    // [(xds.annotations::field_status).work_in_progress = true];
    matcher: Matcher,
        

    /**
    Specifies the type of TLS enforcement the virtual host expects. If this option is not specified, there is no TLS requirement for the virtual host.
    */
    // [(validate.rules).enum = {defined_only: true}];
    require_tls: TLSRequirementType,

    /// A list of virtual clusters defined for this virtual host. Virtual clusters are used for additional statistics gathering.
    virtual_clusters: Vec<VirtualCluster>,

    /// Specifies a set of rate limit configurations that will be applied to the virtual host.
    rate_limits: Vec<RateLimit>,

    /**
    Specifies a list of HTTP headers that should be added to each request
    handled by this virtual host. Headers specified at this level are applied
    after headers from enclosed [`Route`] and before headers from the enclosing [crate::config::route::route::RouteConfiguration]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers
    <config_http_conn_man_headers_custom_request_headers>`.
    */
    // [(validate.rules).repeated = {max_items: 1000}];
    request_headers_to_add: Vec<HeaderValueOption>,
        

    /**
    Specifies a list of HTTP headers that should be removed from each request handled by this virtual host.
    */
    // [(validate.rules).repeated = {
    //     items {String {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}}
    // }];
    request_headers_to_remove: Vec<String>,

    /**
    Specifies a list of HTTP headers that should be added to each response handled by this virtual host. Headers specified at this level are applied after headers from enclosed [`Route`] and before headers from the enclosing [`crate::config::route::route::RouteConfiguration`]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    */
    // [(validate.rules).repeated = {max_items: 1000}]
    response_headers_to_add: Vec<HeaderValueOption>,

    /**
    Specifies a list of HTTP headers that should be removed from each response handled by this virtual host.
    */
    // [(validate.rules).repeated = {
    //     items {String {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}}
    // }];
    response_headers_to_remove: Vec<String>,

    /**
    The per_filter_config field can be used to provide virtual host-specific configurations for filters.
    The key should match the [filter config name][crate::extensions::filters::network::http_connection_manager::HTTPFilter.name].
    The canonical filter name (e.g., `envoy.filters.http.buffer` for the HTTP buffer filter) can also be used for the backwards compatibility. If there is no entry referred by the filter config name, the entry referred by the canonical filter name will be provided to the filters as fallback.

    Use of this field is filter specific; see the :ref:`HTTP filter documentation <config_http_filters>` for if and how it is utilised.
    [#comment: An entry's value may be wrapped in a [`FilterConfig`] message to specify additional options.]
    */
    typed_per_filter_config: HashMap<String, Any>,

    /**
    Decides whether the :ref:`x-envoy-attempt-count <config_http_filters_router_x-envoy-attempt-count>` header should be included in the upstream request. Setting this option will cause it to override any existing header value, so in the case of two Envoys on the request path with this option enabled, the upstream will see the attempt count as perceived by the second Envoy. Defaults to false.
    This header is unaffected by the [`suppress_envoy_headers`][crate::extensions::filters::http.router::Router.suppress_envoy_headers] flag.

    [#next-major-version: rename to include_attempt_count_in_request.]
    */
    include_request_attempt_count: bool,

    /**
    Decides whether the :ref:`x-envoy-attempt-count <config_http_filters_router_x-envoy-attempt-count>` header should be included in the downstream response. Setting this option will cause the router to override any existing header value, so in the case of two Envoys on the request path with this option enabled, the downstream will see the attempt count as perceived by the Envoy closest upstream from itself. Defaults to `false`.
    This header is unaffected by the [`suppress_envoy_headers`][crate::extensions::filters::http.router::Router.suppress_envoy_headers] flag.
    */
    include_attempt_count_in_response: bool,

    /**
    Indicates the retry policy for all routes in this virtual host. Note that setting a route level entry will take precedence over this config and it'll be treated independently (e.g.: values are not inherited).
    */
    retry_policy: RetryPolicy,

    /**
    Specifies the configuration for retry policy extension. Note that setting a route level entry will take precedence over this config and it'll be treated independently (e.g.: values are not inherited). [`retry_policy`][Self::retry_policy] should not be set if this field is used.
    */
    retry_policy_typed_config: Any,

    /// Indicates the hedge policy for all routes in this virtual host. Note that setting a route level entry will take precedence over this config and it'll be treated independently (e.g.: values are not inherited).
    hedge_policy: HedgePolicy,

    /**
    The maximum bytes which will be buffered for retries and shadowing.
    If set and a route-specific limit is not set, the bytes actually buffered will be the minimum value of this and the listener per_connection_buffer_limit_bytes.
    */
    per_request_buffer_limit_bytes: u32,

    /**
    Specify a set of default request mirroring policies for every route under this virtual host.
    It takes precedence over the route config mirror policy entirely.
    That is, policies are not merged, the most specific non-empty one becomes the mirror policies.
    */
    request_mirror_policies: Vec<RequestMirrorPolicy>
}

pub enum TLSRequirementType {
    /// No TLS requirement for the virtual host.
    None,

    /// External requests must use TLS. If a request is external and it is not using TLS, a 301 redirect will be sent telling the client to use HTTPS.
    ExternalOnly,

    /// All requests must use TLS. If a request is not using TLS, a 301 redirect will be sent telling the client to use HTTPS.
    All,
}

/// A filter-defined action type.
pub struct FilterAction {
    action: Any,
}

/**
A route is both a specification of how to match a request as well as an indication of what to do next (e.g., redirect, forward, rewrite, etc.).

> ATTENTION: Envoy supports routing on HTTP method via [header matching][HeaderMatcher].
*/
pub struct Route {
    /// Name for the route.
    name: String,

    /// Route matching parameters.
    // [(validate.rules).message = {required: true}]
    r#match: RouteMatch,

    action: Action,

    /**
    The Metadata field can be used to provide additional information about the route. It can be used for configuration, stats, and logging.
    The metadata should go under the filter namespace that will need it.
    For instance, if the metadata is intended for the Router filter, the filter name should be specified as `envoy.filters.http.router`.
    */
    metadata: Metadata,

    /// Decorator for the matched route.
    decorator: Decorator,

    /**
    The per_filter_config field can be used to provide route-specific configurations for filters.
    The key should match the [filter config name][crate::extensions::filters::network::http_connection_manager::HTTPFilter.name].
    The canonical filter name (e.g., `envoy.filters.http.buffer` for the HTTP buffer filter) can also be used for the backwards compatibility. If there is no entry referred by the filter config name, the entry referred by the canonical filter name will be provided to the filters as fallback.

    Use of this field is filter specific; see the :ref:`HTTP filter documentation <config_http_filters>` for if and how it is utilized.

    > #comment: An entry's value may be wrapped in a [`FilterConfig`] message to specify additional options.
    */
    typed_per_filter_config: HashMap<String, Any>,

    /**
    Specifies a set of headers that will be added to requests matching this route. Headers specified at this level are applied before headers from the enclosing [crate::config::route::route_components::VirtualHost] and [crate::config::route::route::RouteConfiguration]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    [(validate.rules).repeated = {max_items: 1000}];
    */
    request_headers_to_add: Vec<HeaderValueOption>,
        

    /**
    Specifies a list of HTTP headers that should be removed from each request matching this route.

    [(validate.rules).repeated = {
        items {String {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}}
    }];
    */
    request_headers_to_remove: Vec<String>,

    /**
    Specifies a set of headers that will be added to responses to requests matching this route. Headers specified at this level are applied before headers from the enclosing [crate::config::route::route_components::VirtualHost] and [crate::config::route::route::RouteConfiguration]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    [(validate.rules).repeated = {max_items: 1000}];
    */
    response_headers_to_add: Vec<HeaderValueOption>,
        

    /**
    Specifies a list of HTTP headers that should be removed from each response to requests matching this route.

    [(validate.rules).repeated = {
        items {String {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}}
    }];
    */
    response_headers_to_remove: Vec<String>,

    /// Presence of the object defines whether the connection manager's tracing configuration is overridden by this route specific instance.
    tracing: Tracing,

    /**
    The maximum bytes which will be buffered for retries and shadowing.
    If set, the bytes actually buffered will be the minimum value of this and the listener per_connection_buffer_limit_bytes.
    */
    per_request_buffer_limit_bytes: u32,

    /**
    The human readable prefix to use when emitting statistics for this endpoint.
    The statistics are rooted at `vhost.<virtual host name>.route.<stat_prefix>`.
    This should be set for highly critical endpoints that one wishes to get 'per-route' statistics on.
    If not set, endpoint statistics are not generated.

    The emitted statistics are the same as those documented for :ref:`virtual clusters <config_http_filters_router_vcluster_stats>`.

    > WARNING: We do not recommend setting up a stat prefix for every application endpoint. This is both not easily maintainable and statistics use a non-trivial amount of memory(approximately 1KiB per route).
    */
    stat_prefix: String,
}

pub enum Action {
    // option (validate.required) = true;

    /// Route request to some upstream cluster.
    Route(RouteAction),

    /// Return a redirect.
    Redirect(RedirectAction),

    /// Return an arbitrary HTTP response directly, without proxying.
    DirectResponse(DirectResponseAction),

    /**
    A filter-defined action (e.g., it could dynamically generate the RouteAction).
    
    > #comment: TODO(samflattery): Remove cleanup in route_fuzz_test.cc when
    implemented
    */
    FilterAction(FilterAction),

    /**
    [#not-implemented-hide:]
    An action used when the route will generate a response directly, without forwarding to an upstream host. This will be used in non-proxy xDS clients like the gRPC server. It could also be used in the future in Envoy for a filter that directly generates responses for requests.
    */
    NonForwardingAction(NonForwardingAction),
}

/**
Compared to the [`Cluster`][ClusterSpecifier::Cluster] field that specifies a single upstream cluster as the target of a request, the [`WeightedClusters`][ClusterSpecifier::WeightedClusters] option allows for specification of multiple upstream clusters along with weights that indicate the percentage of traffic to be forwarded to each cluster. The router selects an upstream cluster based on the weights.
*/
pub struct WeightedCluster {
    /// Specifies one or more upstream clusters associated with the route.
    // [(validate.rules).repeated = {min_items: 1}];
    clusters: Vec<ClusterWeight>,

    /**
    Specifies the runtime key prefix that should be used to construct the runtime keys associated with each cluster. When the `runtime_key_prefix` is specified, the router will look for weights associated with each upstream cluster under the key `runtime_key_prefix` + `.` + `cluster[i].name` where `cluster[i]` denotes an entry in the clusters array field. If the runtime key for the cluster does not exist, the value specified in the configuration file will be used as the default weight. See the :ref:`runtime documentation <operations_runtime>` for how key names map to the underlying implementation.
    */
    runtime_key_prefix: String,

    random_value_specifier: RandomValueSpecifier
}

pub enum RandomValueSpecifier {
    /**
    Specifies the header name that is used to look up the random value passed in the request header.
    This is used to ensure consistent cluster picking across multiple proxy levels for weighted traffic.
    If header is not present or invalid, Envoy will fall back to use the internally generated random value.
    This header is expected to be single-valued header as we only want to have one selected value throughout the process for the consistency. And the value is a unsigned number between 0 and UINT64_MAX.

    [(validate.rules).String = {well_known_regex: HTTP_HEADER_NAME strict: false}];
    */
    HeaderName(String)
}

pub struct ClusterWeight {
    /**
    Only one of `name` and `cluster_header` may be specified.

    > #next-major-version: Need to add back the validation rule: (validate.rules).String = {min_len: 1}

    Name of the upstream cluster. The cluster must exist in the :ref:`cluster manager configuration <config_cluster_manager>`.

    [(udpa.annotations.field_migrate).oneof_promotion = "cluster_specifier"];
    */
    name: String,

    /**
    Only one of `name` and `cluster_header` may be specified.

    > #next-major-version: Need to add back the validation rule: (validate.rules).String = {min_len: 1 }

    Envoy will determine the cluster to route to by reading the value of the
    HTTP header named by `cluster_header` from the request headers. If the header is not found or the referenced cluster does not exist, Envoy will return a 404 response.

    > ATTENTION: Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1 `Host` header. Thus, if attempting to match on `Host`, match on `:authority` instead.

    > NOTE: If the header appears multiple times only the first value is used.

    [
      (validate.rules).String = {well_known_regex: HTTP_HEADER_NAME strict: false},
      (udpa.annotations.field_migrate).oneof_promotion = "cluster_specifier"
    ];
    */
    cluster_header: String,

    /**
    The weight of the cluster. This value is relative to the other clusters' weights. When a request matches the route, the choice of an upstream cluster is determined by its weight. The sum of weights across all entries in the clusters array must be greater than 0.
    */
    weight: u32,

    /**
    Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints in the upstream cluster with metadata matching what is set in this field will be considered for load balancing. Note that this will be merged with what's provided in [`RouteAction::metadata_match`][RouteAction::metadata_match], with values here taking precedence. The filter name should be specified as `envoy.lb`.
    */
    metadata_match: Metadata,

    /**
    Specifies a list of headers to be added to requests when this cluster is selected through the enclosing [`RouteAction`].
    Headers specified at this level are applied before headers from the enclosing [`Route`], [`VirtualHost`], and
    [crate::config::route::route::RouteConfiguration]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    [(validate.rules).repeated = {max_items: 1000}];
    */
    request_headers_to_add: Vec<HeaderValueOption>,
        

    /**
    Specifies a list of HTTP headers that should be removed from each request when this cluster is selected through the enclosing [`RouteAction`].

    [(validate.rules).repeated = {
      items {String {well_known_regex: HTTP_HEADER_NAME strict: false}}
    }];
    */
    request_headers_to_remove: Vec<String>,

    /**
    Specifies a list of headers to be added to responses when this cluster is selected through the enclosing [`RouteAction`].
    Headers specified at this level are applied before headers from the enclosing
    [`Route`], [crate::config::route::route_components::VirtualHost], and
    [crate::config::route::route::RouteConfiguration]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    [(validate.rules).repeated = {max_items: 1000}];
    */
    response_headers_to_add: Vec<HeaderValueOption>,

    /**
    Specifies a list of headers to be removed from responses when this cluster is selected through the enclosing [`RouteAction`].

    [(validate.rules).repeated = {
      items {String {well_known_regex: HTTP_HEADER_NAME strict: false}}
    }];
    */
    response_headers_to_remove: Vec<String>,

    /**
    The per_filter_config field can be used to provide weighted cluster-specific configurations for filters.
    The key should match the [filter config name][crate::extensions::filters::network::http_connection_manager::HTTPFilter.name].
    The canonical filter name (e.g., `envoy.filters.http.buffer` for the HTTP buffer filter) can also be used for the backwards compatibility. If there is no entry referred by the filter config name, the entry referred by the canonical filter name will be provided to the filters as fallback.

    Use of this field is filter specific; see the :ref:`HTTP filter documentation <config_http_filters>` for if and how it is utilized.
    
    > #comment: An entry's value may be wrapped in a [`FilterConfig`] message to specify additional options.]
    */
    typed_per_filter_config: HashMap<String, Any>,

    host_rewrite_specifier: ClusterWeightHostRewriteSpecifier
}

pub enum ClusterWeightHostRewriteSpecifier {
    /// Indicates that during forwarding, the host header will be swapped with this value.
    // [(validate.rules).String = {well_known_regex: HTTP_HEADER_VALUE strict: false}];
    HostRewriteLiteral(String)
}

/// Configuration for a cluster specifier plugin.
pub struct ClusterSpecifierPlugin {
    /// The name of the plugin and its opaque configuration.
    // [(validate.rules).message = {required: true}];
    extension: TypedExtensionConfig,

    /**
    If is_optional is not set or is set to false and the plugin defined by this message is not a supported type, the containing resource is NACKed. If is_optional is set to true, the resource would not be NACKed for this reason. In this case, routes referencing this plugin's name would not be treated as an illegal configuration, but would result in a failure if the route is selected.
    */
    is_optional: bool,
}

pub struct RouteMatch {
    path_specifier: PathSpecifier,

    /// Indicates that prefix/path matching should be case sensitive. The default is true. Ignored for safe_regex matching.
    case_sensitive: bool,

    /**
    Indicates that the route should additionally match on a runtime key. Every time the route is considered for a match, it must also fall under the percentage of matches indicated by this field. For some fraction N/D, a random number in the range [0,D) is selected. If the number is <= the value of the numerator N, or if the key is not present, the default value, the router continues to evaluate the remaining match criteria. A runtime_fraction route configuration can be used to roll out route changes in a gradual manner without full code/config deploys. Refer to the :ref:`traffic shifting <config_http_conn_man_route_table_traffic_splitting_shift>` docs for additional documentation.


    > NOTE: Parsing this field is implemented such that the runtime key's data may be represented as a FractionalPercent proto represented as JSON/YAML and may also be represented as an integer with the assumption that the value is an integral percentage out of 100. For instance, a runtime key lookup returning the value "42" would parse as a FractionalPercent whose numerator is 42 and denominator is HUNDRED. This preserves legacy semantics.
    */
    runtime_fraction: RuntimeFractionalPercent,

    /**
    Specifies a set of headers that the route should match on. The router will check the request’s headers against all the specified headers in the route config. A match will happen if all the headers in the route are present in the request with the same values (or based on presence if the value field is not in the config).
    */
    headers: Vec<HeaderMatcher>,

    /**
    Specifies a set of URL query parameters on which the route should match. The router will check the query string from the `path` header against all the specified query parameters. If the number of specified query parameters is nonzero, they all must match the `path` header's query string for a match to occur.

    > NOTE: If query parameters are used to pass request message fields when `grpc_json_transcoder <https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/grpc_json_transcoder_filter>`_ is used, the transcoded message fields maybe different. The query parameters are url encoded, but the message fields are not. For example, if a query parameter is "foo%20bar", the message field will be "foo bar".
    */
    query_parameters: Vec<QueryParameterMatcher>,

    /**
    If specified, only gRPC requests will be matched. The router will check that the content-type header has a application/grpc or one of the various application/grpc+ values.
    */
    grpc: GRPCRouteMatchOptions,

    /**
    If specified, the client tls context will be matched against the defined match options.

    [#next-major-version: unify with RBAC]
    */
    tls_context: TLSContextMatchOptions,

    /*
    Specifies a set of dynamic metadata matchers on which the route should match.
    The router will check the dynamic metadata against all the specified dynamic metadata matchers.
    If the number of specified dynamic metadata matchers is nonzero, they all must match the dynamic metadata for a match to occur.
    */
    dynamic_metadata: Vec<MetadataMatcher>,
}

pub enum PathSpecifier {
    // option (validate.required) = true;

    /// If specified, the route is a prefix rule meaning that the prefix must match the beginning of the `:path` header.
    Prefix(String),

    /// If specified, the route is an exact path rule meaning that the path must exactly match the `:path` header once the query string is removed.
    Path(String),

    /**
    If specified, the route is a regular expression rule meaning that the regex must match the `:path` header once the query string is removed. The entire path (without the query string) must match the regex. The rule will not match if only a subsequence of the `:path` header matches the regex.

    [#next-major-version: In the v3 API we should redo how path specification works such that we utilise StringMatcher, and additionally have consistent options around whether we strip query strings, do a case sensitive match, etc. In the interim it will be too disruptive to deprecate the existing options. We should even consider whether we want to do away with path_specifier entirely and just rely on a set of header matchers which can already match on :path, etc. The issue with that is it is unclear how to generically deal with query string stripping. This needs more thought.]

    [(validate.rules).message = {required: true}];
    */
    SafeRegex(RegexMatcher),

    /**
    If this is used as the matcher, the matcher will only match CONNECT requests.
    Note that this will not match HTTP/2 upgrade-style CONNECT requests
    (WebSocket and the like) as they are normalised in Envoy as HTTP/1.1 style upgrades.
    This is the only way to match CONNECT requests for HTTP/1.1. For HTTP/2, where Extended CONNECT requests may have a path, the path matchers will work if there is a path present.
    Note that CONNECT support is currently considered alpha in Envoy.
    [#comment: TODO(htuch): Replace the above comment with an alpha tag.]
    */
    ConnectMatcher(ConnectMatcher),

    /**
    If specified, the route is a path-separated prefix rule meaning that the
    `:path` header (without the query string) must either exactly match the
    `path_separated_prefix` or have it as a prefix, followed by `/`

    For example, `/api/dev` would match
    `/api/dev`, `/api/dev/`, `/api/dev/v1`, and `/api/dev?param=true`
    but would not match `/api/developer`

    Expect the value to not contain `?` or `#` and not to end in `/`

    [(validate.rules).string = {pattern: "^[^?#]+[^?#/]$"}];
    */
    PathSeparatedPrefix(String),

    /// [#extension-category: envoy.path.match]
    PathMatchPolicy(TypedExtensionConfig),
}

pub struct GRPCRouteMatchOptions {
}

pub struct TLSContextMatchOptions {
    /**
    If specified, the route will match against whether or not a certificate is presented.
    If not specified, certificate presentation status (true or false) will not be considered when route matching.
    */
    presented: bool,

    /**
    If specified, the route will match against whether or not a certificate is validated.
    If not specified, certificate validation status (true or false) will not be considered when route matching.
    */
    validated: bool,
}

/// An extensible message for matching CONNECT requests.
pub struct ConnectMatcher {
}

/**
CORS policy configuration.

> ATTENTION: This message has been deprecated. Please use [CorsPolicy in filter extension][crate::extensions::filters.http.cors::CorsPolicy] as as alternative.
*/
pub struct CorsPolicy {
    /// Specifies string patterns that match allowed origins. An origin is allowed if any of the string matchers match.
    allow_origin_string_match: Vec<StringMatcher>,

    /// Specifies the content for the `access-control-allow-methods` header.
    allow_methods: String,

    /// Specifies the content for the `access-control-allow-headers` header.
    allow_headers: String,

    /// Specifies the content for the `access-control-expose-headers` header.
    expose_headers: String,

    /// Specifies the content for the `access-control-max-age` header.
    max_age: String,

    /// Specifies whether the resource allows credentials.
    allow_credentials: bool,

    enabled_specifier: EabledSpecifier,

    /**
    Specifies the % of requests for which the CORS policies will be evaluated and tracked, but not enforced.

    This field is intended to be used when `filter_enabled` and `enabled` are off. One of those fields have to explicitly disable the filter in order for this setting to take effect.

    If [`runtime_key`][crate::config::RuntimeFractionalPercent.runtime_key] is specified,
    Envoy will lookup the runtime key to get the percentage of requests for which it will evaluate and track the request's `Origin` to determine if it's valid but will not enforce any policies.
    */
    shadow_enabled: RuntimeFractionalPercent,

    /**
    Specify whether allow requests whose target server's IP address is more private than that from which the request initiator was fetched.

    More details refer to <https://developer.chrome.com/blog/private-network-access-preflight>.
    */
    allow_private_network_access: bool,
}

pub enum EabledSpecifier {
    /**
    Specifies the % of requests for which the CORS filter is enabled.

    If neither `enabled`, `filter_enabled`, nor `shadow_enabled` are specified, the CORS filter will be enabled for 100% of the requests.

    If [`runtime_key`][crate::config::core::base::RuntimeFractionalPercent::runtime_key] is specified, Envoy will lookup the runtime key to get the percentage of requests to filter.
    */
    FilterEnabled(RuntimeFractionalPercent),
}

pub enum EnabledSpecifier {
    /**
    Specifies the % of requests for which the CORS filter is enabled.

    If neither `enabled`, `filter_enabled`, nor `shadow_enabled` are specified, the CORS filter will be enabled for 100% of the requests.

    If [`runtime_key`][crate::config::RuntimeFractionalPercent.runtime_key] is specified, Envoy will lookup the runtime key to get the percentage of requests to filter.
    */
    FilterEnabled(RuntimeFractionalPercent),
}

pub struct RouteAction {
    /**
    The HTTP status code to use when configured cluster is not found.
    The default response code is 503 Service Unavailable.
    */
    // [(validate.rules).enum = {defined_only: true}]
    cluster_not_found_response_code: ClusterNotFoundResponseCode,

    /**
    Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints in the upstream cluster with metadata matching what's set in this field will be considered for load balancing. If using [`WeightedClusters`][ClusterSpecifier::WeightedClusters], metadata will be merged, with values provided there taking precedence. The filter name should be specified as `envoy.lb`.
    */
    metadata_match: Metadata,

    /**
    Indicates that during forwarding, the matched prefix (or path) should be swapped with this value. This option allows application URLs to be rooted at a different path from those exposed at the reverse proxy layer. The router filter will place the original path before rewrite into the :ref:`x-envoy-original-path <config_http_filters_router_x-envoy-original-path>` header.

    Only one of :ref:`regex_rewrite <Self::regex_rewrite>` :ref:`path_rewrite_policy <Self::path_rewrite_policy>`, or :ref:`prefix_rewrite <Self::prefix_rewrite>` may be specified.

    > ATTENTION: Pay careful attention to the use of trailing slashes in the  [route's match][Route::match] prefix value.
    > Stripping a prefix from a path requires multiple Routes to handle all cases. For example, rewriting `/prefix` to `/` and `/prefix/etc` to `/etc` cannot be done in a single [`Route`], as shown by the below config entries:

      .. code-block:: yaml

        - match:
            prefix: "/prefix/"
          route:
            prefix_rewrite: "/"
        - match:
            prefix: "/prefix"
          route:
            prefix_rewrite: "/"
    
      Having above entries in the config, requests to `/prefix` will be stripped to `/`, while
      requests to `/prefix/etc` will be stripped to `/etc`.
    */
    // [(validate.rules).String = {well_known_regex: HTTP_HEADER_VALUE strict: false}];
    prefix_rewrite: String,
        

    /**
    Indicates that during forwarding, portions of the path that match the pattern should be rewritten, even allowing the substitution of capture groups from the pattern into the new path as specified by the rewrite substitution string. This is useful to allow application paths to be rewritten in a way that is aware of segments with variable content like identifiers. The router filter will place the original path as it was before the rewrite into the :ref:`x-envoy-original-path <config_http_filters_router_x-envoy-original-path>` header.

    Only one of `regex_rewrite`, [`prefix_rewrite`][Self::prefix_rewrite], or [`path_rewrite_policy`][Self::path_rewrite_policy] may be specified.

    Examples using Google's [RE2](https://github.com/google/re2) engine:

    - The path pattern `^/service/([^/]+)(/.*)$` paired with a substitution string of `\2/instance/\1` would transform `/service/foo/v1/api` into `/v1/api/instance/foo`.

    - The pattern `one` paired with a substitution string of `two` would transform `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/two/zzz`.

    - The pattern `^(.*?)one(.*)$` paired with a substitution string of `\1two\2` would replace only the first occurrence of `one`, transforming path `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/one/zzz`.

    - The pattern `(?i)/xxx/` paired with a substitution string of `/yyy/` would do a case-insensitive match and transform path `/aaa/XxX/bbb` to `/aaa/yyy/bbb`.
    */
    regex_rewrite: RegexMatchAndSubstitute,

    /// [#extension-category: envoy.path.rewrite]
    path_rewrite_policy: TypedExtensionConfig,

    host_rewrite_specifier: HostRewriteSpecifier,

    /**
    If set, then a host rewrite action (one of [`HostRewriteLiteral`][HostRewriteSpecifier::HostRewriteLiteral], [`AutoHostRewrite`][HostRewriteSpecifier::AutoHostRewrite], [`HostRewriteHeader`][HostRewriteSpecifier::HostRewriteHeader], or [`HostRewritePathRegex`][HostRewriteSpecifier::HostRewritePathRegex]) causes the original value of the host header, if any, to be appended to the :ref:`config_http_conn_man_headers_x-forwarded-host` HTTP header.
    */
    append_x_forwarded_host: bool,

    /**
    Specifies the upstream timeout for the route. If not specified, the default is 15s. This spans between the point at which the entire downstream request (i.e. end-of-stream) has been processed and when the upstream response has been completely processed. A value of 0 will disable the route's timeout.

    > NOTE: This timeout includes all retries. See also
      :ref:`config_http_filters_router_x-envoy-upstream-rq-timeout-ms`,
      :ref:`config_http_filters_router_x-envoy-upstream-rq-per-try-timeout-ms`, and the
      :ref:`retry overview <arch_overview_http_routing_retry>`.
    */
    timeout: Duration,

    /**
    Specifies the idle timeout for the route. If not specified, there is no per-route idle timeout, although the connection manager wide [`stream_idle_timeout`][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.stream_idle_timeout] will still apply. A value of 0 will completely disable the route's idle timeout, even if a connection manager stream idle timeout is configured.

    The idle timeout is distinct to [`timeout`][Self::timeout], which provides an upper bound on the upstream response time; `idle_timeout` instead bounds the amount of time the request's stream may be idle.

    After header decoding, the idle timeout will apply on downstream and upstream request events. Each time an encode/decode event for headers or data is processed for the stream, the timer will be reset. If the timeout fires, the stream is terminated with a 408 Request Timeout error code if no upstream response header has been received, otherwise a stream reset occurs.

    If the :ref:`overload action <config_overload_manager_overload_actions>` "envoy.overload_actions.reduce_timeouts" is configured, this timeout is scaled according to the value for [`HTTPDownstreamConnectionIdle`][crate::config::overload::ScaleTimersOverloadActionConfig.TimerType.HTTPDownstreamConnectionIdle].
    */
    idle_timeout: Duration,

    /**
    Specifies how to send request over TLS early data.
    If absent, allows `safe HTTP requests <https://www.rfc-editor.org/rfc/rfc7231#section-4.2.1>`_ to be sent on early data.
    [#extension-category: envoy.route.early_data_policy]
    */
    early_data_policy: TypedExtensionConfig,

    /**
    Indicates that the route has a retry policy. Note that if this is set, it'll take precedence over the virtual host level retry policy entirely (e.g.: policies are not merged, most internal one becomes the enforced policy).
    */
    retry_policy: RetryPolicy,

    /**
    [#not-implemented-hide:]
    Specifies the configuration for retry policy extension. Note that if this is set, it'll take precedence over the virtual host level retry policy entirely (e.g.: policies are not merged, most internal one becomes the enforced policy). [Retry policy][crate::config::route::route_components::VirtualHost::retry_policy] should not be set if this field is used.
    */
    retry_policy_typed_config: Any,

    /**
    Specify a set of route request mirroring policies.
    It takes precedence over the virtual host and route config mirror policy entirely.
    That is, policies are not merged, the most specific non-empty one becomes the mirror policies.
    */
    request_mirror_policies: Vec<RequestMirrorPolicy>,

    /// Optionally specifies the :ref:`routing priority <arch_overview_http_routing_priority>`.
    // [(validate.rules).enum = {defined_only: true}];
    priority: RoutingPriority,

    /// Specifies a set of rate limit configurations that could be applied to the route.
    rate_limits: Vec<RateLimit>,

    /**
    Specifies a list of hash policies to use for ring hash load balancing. Each
    hash policy is evaluated individually and the combined result is used to
    route the request. The method of combination is deterministic such that
    identical lists of hash policies will produce the same hash. Since a hash
    policy examines specific parts of a request, it can fail to produce a hash
    (i.e. if the hashed header is not present). If (and only if) all configured
    hash policies fail to generate a hash, no hash will be produced for
    the route. In this case, the behaviour is the same as if no hash policies
    were specified (i.e. the ring hash load balancer will choose a random
    backend). If a hash policy has the "terminal" attribute set to true, and
    there is already a hash generated, the hash is returned immediately,
    ignoring the rest of the hash policy list.
    */
    hash_policy: Vec<HashPolicy>,

    upgrade_configs: Vec<UpgradeConfig>,

    /**
    If present, Envoy will try to follow an upstream redirect response instead of proxying the response back to the downstream. An upstream redirect response is defined by [`redirect_response_codes`][crate::config::route::InternalRedirectPolicy.redirect_response_codes].
    */
    internal_redirect_policy: InternalRedirectPolicy,

    /**
    Indicates that the route has a hedge policy. Note that if this is set, it'll take precedence over the virtual host level hedge policy entirely (e.g.: policies are not merged, most internal one becomes the enforced policy).
    */
    hedge_policy: HedgePolicy,

    /// Specifies the maximum stream duration for this route.
    max_stream_duration: MaxStreamDuration,
}

pub enum HostRewriteSpecifier {
    /**
    Indicates that during forwarding, the host header will be swapped with this value. Using this option will append the :ref:`config_http_conn_man_headers_x-forwarded-host` header if [`append_x_forwarded_host`][RouteAction::append_x_forwarded_host] is set.
    */
    // [(validate.rules).string = {well_known_regex: HTTP_HEADER_VALUE strict: false}];
    HostRewriteLiteral(String),

    /**
    Indicates that during forwarding, the host header will be swapped with the hostname of the upstream host chosen by the cluster manager. This option is applicable only when the destination cluster for a route is of type `strict_dns` or `logical_dns`. Setting this to true with other cluster types has no effect. Using this option will append the :ref:`config_http_conn_man_headers_x-forwarded-host` header if [`append_x_forwarded_host`][RouteAction::append_x_forwarded_host] is set.
    */
    AutoHostRewrite(bool),

    /**
    Indicates that during forwarding, the host header will be swapped with the content of given downstream or :ref:`custom <config_http_conn_man_headers_custom_request_headers>` header.
    If header value is empty, host header is left intact. Using this option will append the :ref:`config_http_conn_man_headers_x-forwarded-host` header if [`append_x_forwarded_host`][RouteAction::append_x_forwarded_host] is set.

    > ATTENTION: Pay attention to the potential security implications of using this option. Provided header
      must come from trusted source.

    > NOTE: If the header appears multiple times only the first value is used.
    */
    // [(validate.rules).string = {well_known_regex: HTTP_HEADER_NAME strict: false}];
    HostRewriteHeader(String),
        

    /**
    Indicates that during forwarding, the host header will be swapped with
    the result of the regex substitution executed on path value with query and fragment removed.
    This is useful for transitioning variable content between path segment and subdomain.
    Using this option will append the :ref:`config_http_conn_man_headers_x-forwarded-host` header if
    [`append_x_forwarded_host`][RouteAction::append_x_forwarded_host] is set.

    For example with the following config:

    ```yaml
    host_rewrite_path_regex:
      pattern:
        google_re2: {}
        regex: "^/(.+)/.+$"
      substitution: \1
    ```

    Would rewrite the host header to `envoyproxy.io` given the path `/envoyproxy.io/some/path`.
    */
    HostRewritePathRegex(RegexMatchAndSubstitute),
}

/**
The router is capable of shadowing traffic from one cluster to another. The current implementation is "fire and forget," meaning Envoy will not wait for the shadow cluster to respond before returning the response from the primary cluster. All normal statistics are collected for the shadow cluster making this feature useful for testing.

During shadowing, the host/authority header is altered such that `-shadow` is appended. This is useful for logging. For example, `cluster1` becomes `cluster1-shadow`.

> NOTE: Shadowing will not be triggered if the primary cluster does not exist.

> NOTE: Shadowing doesn't support HTTP CONNECT and upgrades.
*/
pub struct RequestMirrorPolicy {
    /**
    Only one of `cluster` and `cluster_header` can be specified.
    
    > #next-major-version: Need to add back the validation rule: (validate.rules).String = {min_len: 1}

    Specifies the cluster that requests will be mirrored to. The cluster must exist in the cluster manager configuration.
    */
    // [(udpa.annotations.field_migrate).oneof_promotion = "cluster_specifier"];
    cluster: String,

    /**
    Only one of `cluster` and `cluster_header` can be specified.
    Envoy will determine the cluster to route to by reading the value of the
    HTTP header named by `cluster_header` from the request headers. Only the first value in header is used, and no shadow request will happen if the value is not found in headers. Envoy will not wait for the shadow cluster to respond before returning the response from the primary cluster.

    > ATTENTION: Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1 `Host` header. Thus, if attempting to match on `Host`, match on `:authority` instead.

    > NOTE: If the header appears multiple times only the first value is used.
    */
    // [
    //  (validate.rules).String = {well_known_regex: HTTP_HEADER_NAME strict: false},
    //  (udpa.annotations.field_migrate).oneof_promotion = "cluster_specifier"
    // ];
    cluster_header: String,

    /**
    If not specified, all requests to the target cluster will be mirrored.

    If specified, this field takes precedence over the `runtime_key` field and requests must also fall under the percentage of matches indicated by this field.

    For some fraction N/D, a random number in the range [0,D) is selected. If the number is <= the value of the numerator N, or if the key is not present, the default value, the request will be mirrored.
    */
    runtime_fraction: RuntimeFractionalPercent,

    /// Determines if the trace span should be sampled. Defaults to `true`.
    trace_sampled: bool,
}

/**
Specifies the route's hashing policy if the upstream cluster uses a hashing :ref:`load balancer
<arch_overview_load_balancing_types>`.
[#next-free-field: 7]
*/
pub struct HashPolicy {
    /**
    The flag that short-circuits the hash computing. This field provides a 'fallback' style of configuration: "if a terminal policy doesn't work, fallback to rest of the policy list", it saves time when the terminal policy works.

    If `true`, and there is already a hash computed, ignore rest of the list of hash polices.
    For example, if the following hash methods are configured:

    ========= ========
    specifier terminal
    ========= ========
    Header A  true
    Header B  false
    Header C  false
    ========= ========

    The generateHash process ends if policy "header A" generates a hash, as it's a terminal policy.
    */
    terminal: bool,
}

pub struct Header {
    /**
    The name of the request header that will be used to obtain the hash key. If the request header is not present, no hash will be produced.

    [(validate.rules).String = {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}];
    */
    header_name: String,

    /// If specified, the request header value will be rewritten and used to produce the hash key.
    regex_rewrite: RegexMatchAndSubstitute,
}

/**
Envoy supports two types of cookie affinity:

1. Passive. Envoy takes a cookie that's present in the cookies header and hashes on its value.

2. Generated. Envoy generates and sets a cookie with an expiration (TTL) on the first request from the client in its response to the client, based on the endpoint the request gets sent to. The client then presents this on the next and all subsequent requests. The hash of this is sufficient to ensure these requests get sent to the same endpoint. The cookie is generated by hashing the source and destination ports and addresses so that multiple independent HTTP2 streams on the same connection will independently receive the same cookie, even if they arrive at the Envoy simultaneously.
*/
pub struct Cookie {
    /**
    The name of the cookie that will be used to obtain the hash key. If the cookie is not present and ttl below is not set, no hash will be produced.
    [(validate.rules).String = {min_len: 1}];
    */
    name: String,

    /*
    If specified, a cookie with the TTL will be generated if the cookie is not present. If the TTL is present and zero, the generated cookie will be a session cookie.
    */
    ttl: Duration,

    /**
    The name of the path for the cookie. If no path is specified here, no path will be set for the cookie.
    */
    path: String,
}

pub struct ConnectionProperties {
    /// Hash on source IP address.
    source_ip: bool,
}

pub struct QueryParameter {
    /*
    The name of the URL query parameter that will be used to obtain the hash key. If the parameter is not present, no hash will be produced. Query parameter names are case-sensitive.

    [(validate.rules).String = {min_len: 1}]
    */
    name: String
}

pub struct FilterState {
    /**
    The name of the Object in the per-request filterState, which is an
    Envoy::Hashable object. If there is no data associated with the key, or the stored object is not Envoy::Hashable, no hash will be produced.

    [(validate.rules).String = {min_len: 1}]
    */
    key: String
}

pub enum PolicySpecifier {
    // option (validate.required) = true;

    /// Header hash policy.
    Header(Header),

    /// Cookie hash policy.
    Cookie(Cookie),

    /// Connection properties hash policy.
    ConnectionProperties(ConnectionProperties),

    /// Query parameter hash policy.
    QueryParameter(QueryParameter),

    /// Filter state hash policy.
    FilterState(FilterState),
}

/**
Allows enabling and disabling upgrades on a per-route basis.
This overrides any enabled/disabled upgrade filter chain specified in the
HTTPConnectionManager [`upgrade_configs`][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.upgrade_configs] but does not affect any custom filter chain specified there.
*/
pub struct UpgradeConfig {
    /**
    The case-insensitive name of this upgrade, e.g. "websocket".
    For each upgrade type present in upgrade_configs, requests with
    Upgrade: `upgrade_type` will be proxied upstream.

    [(validate.rules).String = {min_len: 1 well_known_regex: HTTP_HEADER_VALUE strict: false}];
    */
    upgrade_type: String,
        

    /// Determines if upgrades are available on this route. Defaults to true.
    enabled: bool,

    /**
    Configuration for sending data upstream as a raw data payload. This is used for CONNECT requests, when forwarding CONNECT payload as raw TCP.
    Note that CONNECT support is currently considered alpha in Envoy.
    [#comment: TODO(htuch): Replace the above comment with an alpha tag.]
    */
    connect_config: ConnectConfig,
}

/// Configuration for sending data upstream as a raw data payload. This is used for CONNECT or POST requests, when forwarding request payload as raw TCP.
pub struct ConnectConfig {
    /// If present, the proxy protocol header will be prepended to the CONNECT payload sent upstream.
    proxy_protocol_config: ProxyProtocolConfig,

    /// If set, the route will also allow forwarding POST payload as raw TCP.
    allow_post: bool,
}

pub struct MaxStreamDuration {
    /**
    Specifies the maximum duration allowed for streams on the route. If not specified, the value from the [`max_stream_duration`][crate::config::core::HTTPProtocolOptions.max_stream_duration>` field in :ref:`HTTPConnectionManager.common_http_protocol_options <crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.common_http_protocol_options] is used. If this field is set explicitly to zero, any HTTPConnectionManager max_stream_duration timeout will be disabled for this route.
    */
    max_stream_duration: Duration,

    /**
    If present, and the request contains a `grpc-timeout header <https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md>`_, use that value as the `max_stream_duration`, but limit the applied timeout to the maximum value specified here.
    If set to 0, the `grpc-timeout` header is used without modification.
    */
    grpc_timeout_header_max: Duration,

    /*
    If present, Envoy will adjust the timeout provided by the `grpc-timeout` header by subtracting the provided duration from the header. This is useful for allowing Envoy to set its global timeout to be less than that of the deadline imposed by the calling client, which makes it more likely that Envoy will handle the timeout instead of having the call cancelled by the client. If, after applying the offset, the resulting timeout is zero or negative, the stream will timeout immediately.
    */
    grpc_timeout_header_offset: Duration,
}

pub enum ClusterSpecifier {
    // option (validate.required) = true;

    /**
    Indicates the upstream cluster to which the request should be routed
    to.

    [(validate.rules).String = {min_len: 1}];
    */
    Cluster(String),

    /**
    Envoy will determine the cluster to route to by reading the value of the HTTP header named by `cluster_header` from the request headers. If the header is not found or the referenced cluster does not exist, Envoy will return a 404 response.

    > ATTENTION: Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1 `Host` header. Thus, if attempting to match on `Host`, match on `:authority` instead.

    > NOTE: If the header appears multiple times only the first value is used.

    [(validate.rules).String = {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}];
    */
    ClusterHeader(String),

    /**
    Multiple upstream clusters can be specified for a given route. The request is routed to one of the upstream clusters based on weights assigned to each cluster. See :ref:`traffic splitting <config_http_conn_man_route_table_traffic_splitting_split>` for additional documentation.
    */
    WeightedClusters(WeightedCluster),

    /**
    Name of the cluster specifier plugin to use to determine the cluster for requests on this route.
    The cluster specifier plugin name must be defined in the associated [cluster specifier plugins][crate::config::route::route::RouteConfiguration.cluster_specifier_plugins] in the [`name`][crate::config::TypedExtensionConfig.name] field.
    */
    ClusterSpecifierPlugin(String),

    /// Custom cluster specifier plugin configuration to use to determine the cluster for requests on this route.
    InlineClusterSpecifierPlugin(ClusterSpecifierPlugin),
}

pub enum ClusterNotFoundResponseCode {
    // HTTP status code - 503 Service Unavailable.
    ServiceUnavailable,

    /// HTTP status code - 404 Not Found.
    NotFound,

    /// HTTP status code - 500 Internal Server Error.
    InternalServerError,
}

/// HTTP retry :ref:`architecture overview <arch_overview_http_routing_retry>`.
pub struct RetryPolicy {
    /// Specifies the conditions under which retry takes place. These are the same conditions documented for :ref:`config_http_filters_router_x-envoy-retry-on` and :ref:`config_http_filters_router_x-envoy-retry-grpc-on`.
    retry_on: String,

    /**
    Specifies the allowed number of retries. This parameter is optional and defaults to `1`. These are the same conditions documented for :ref:`config_http_filters_router_x-envoy-max-retries`.

    [(udpa.annotations.field_migrate).rename = "max_retries"];
    */
    num_retries: u32,
        

    /**
    Specifies a non-zero upstream timeout per retry attempt (including the initial attempt). This
    parameter is optional. The same conditions documented for
    :ref:`config_http_filters_router_x-envoy-upstream-rq-per-try-timeout-ms` apply.

    > NOTE: If left unspecified, Envoy will use the global [route timeout][RouteAction::timeout] for the request.
    > Consequently, when using a :ref:`5xx <config_http_filters_router_x-envoy-retry-on>` based retry policy, a request that times out will not be retried as the total timeout budget would have been exhausted.
    */
    per_try_timeout: Duration,

    /**
    Specifies an upstream idle timeout per retry attempt (including the initial attempt). This parameter is optional and if absent there is no per try idle timeout. The semantics of the per try idle timeout are similar to the
    [route idle timeout][RouteAction::timeout] and [stream idle timeout][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.stream_idle_timeout] both enforced by the HTTP connection manager. The difference is that this idle timeout is enforced by the router for each individual attempt and thus after all previous filters have run, as opposed to *before* all previous filters run for the other idle timeouts. This timeout is useful in cases in which total request timeout is bounded by a number of retries and a [`per_try_timeout`], but there is a desire to ensure each try is making incremental progress. Note also that similar to [`per_try_timeout`], this idle timeout does not start until after both the entire request has been received by the router *and* a connection pool connection has been obtained. Unlike [`per_try_timeout`], the idle timer continues once the response starts streaming back to the downstream client.
    This ensures that response data continues to make progress without using one of the HTTP connection manager idle timeouts.

    [`per_try_timeout`]: Self::per_try_timeout
    */
    per_try_idle_timeout: Duration,

    /**
    Specifies an implementation of a RetryPriority which is used to determine the distribution of load across priorities used for retries. Refer to :ref:`retry plugin configuration <arch_overview_http_retry_plugins>` for more details.
    */
    retry_priority: RetryPriority,

    /**
    Specifies a collection of RetryHostPredicates that will be consulted when selecting a host for retries. If any of the predicates reject the host, host selection will be reattempted.
    Refer to :ref:`retry plugin configuration <arch_overview_http_retry_plugins>` for more details.
    */
    retry_host_predicate: Vec<RetryHostPredicate>,

    /*
    Retry options predicates that will be applied prior to retrying a request. These predicates allow customizing request behaviour between retries.
    
    > #comment: add [#extension-category: envoy.retry_options_predicates] when there are built-in extensions
    */
    retry_options_predicates: Vec<TypedExtensionConfig>,

    /**
    The maximum number of times host selection will be reattempted before giving up, at which point the host that was last selected will be routed to. If unspecified, this will default to retrying once.
    */
    host_selection_retry_max_attempts: i64,

    /// HTTP status codes that should trigger a retry in addition to those specified by retry_on.
    retriable_status_codes: Vec<u32>,

    /**
    Specifies parameters that control exponential retry back off. This parameter is optional, in which case the default base interval is 25 milliseconds or, if set, the current value of the `upstream.base_retry_backoff_ms` runtime parameter. The default maximum interval is 10 times the base interval. The documentation for :ref:`config_http_filters_router_x-envoy-max-retries` describes Envoy's back-off algorithm.
    */
    retry_back_off: RetryBackOff,

    /**
    Specifies parameters that control a retry back-off strategy that is used when the request is rate limited by the upstream server. The server may return a response header like `Retry-After` or `X-RateLimit-Reset` to provide feedback to the client on how long to wait before retrying. If configured, this back-off strategy will be used instead of the default exponential back off strategy (configured using `retry_back_off`) whenever a response includes the matching headers.
    */
    rate_limited_retry_back_off: RateLimitedRetryBackOff,

    /**
    HTTP response headers that trigger a retry if present in the response. A retry will be triggered if any of the header matches match the upstream response headers.
    The field is only consulted if 'retriable-headers' retry policy is active.
    */
    retriable_headers: Vec<HeaderMatcher>,

    /// HTTP headers which must be present in the request for retries to be attempted.
    retriable_request_headers: Vec<HeaderMatcher>,
}

pub enum ResetHeaderFormat {
    Seconds,
    UnixTimestamp,
}

pub struct RetryPriority {
    // [!is_empty()]
    name: String,

    // [#extension-category: envoy.retry_priorities]
    config_type: ConfigType
}

pub enum ConfigType {
    TypedConfig(Any)
}

pub struct RetryHostPredicate {
    // [!is_empty()]
    name: String,

    // [#extension-category: envoy.retry_host_predicates]
    config_type: ConfigType
}

pub struct RetryBackOff {
    /**
    Specifies the base interval between retries. This parameter is required and must be greater than zero. Values less than 1 ms are rounded up to 1 ms.
    See :ref:`config_http_filters_router_x-envoy-max-retries` for a discussion of Envoy's back-off algorithm.

    [(validate.rules).duration = {
      required: true
      gt {}
    }];
    */
    base_interval: Duration,

    /**
    Specifies the maximum interval between retries. This parameter is optional, but must be greater than or equal to the `base_interval` if set. The default is 10 times the `base_interval`. Se :ref:`config_http_filters_router_x-envoy-max-retries` for a discussion of Envoy's back-off algorithm.

    [(validate.rules).duration = {gt {}}];
    */
    max_interval: Duration
}

pub struct ResetHeader {
    /**
    The name of the reset header.

    > NOTE: If the header appears multiple times only the first value is used.

    [(validate.rules).string = {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}];
    */
    name: String,
        

    /// The format of the reset header.
    // [(validate.rules).enum = {defined_only: true}];
    format: ResetHeaderFormat
}

/**
A retry back-off strategy that applies when the upstream server rate limits the request.

Given this configuration:

```yaml
rate_limited_retry_back_off:
  reset_headers:
    - name: Retry-After
      format: SECONDS
    - name: X-RateLimit-Reset
      format: UNIX_TIMESTAMP
  max_interval: "300s"
```

The following algorithm will apply:

1. If the response contains the header `Retry-After` its value must be on the form `120` (an integer that represents the number of seconds to wait before retrying). If so, this value is used as the back-off interval.
2. Otherwise, if the response contains the header `X-RateLimit-Reset` its value must be on the form `1595320702` (an integer that represents the point in time at which to retry, as a Unix timestamp in seconds). If so, the current time is subtracted from this value and the result is used as the back-off interval.
3. Otherwise, Envoy will use the default [`exponential back-off][crate::config::route::route_components::RetryPolicy.retry_back_off] strategy.

No matter which format is used, if the resulting back-off interval exceeds `max_interval` it is discarded and the next header in `reset_headers` is tried. If a request timeout is configured for the route it will further limit how long the request will be allowed to run.

To prevent many clients retrying at the same point in time jitter is added to the back-off interval, so the resulting interval is decided by taking: `random(interval, interval * 1.5)`.

> ATTENTION: Configuring `rate_limited_retry_back_off` will not by itself cause a request to be retried. You will still need to configure the right retry policy to match the responses from the upstream server.
*/
pub struct RateLimitedRetryBackOff {
    /**
    Specifies the reset headers (like `Retry-After` or `X-RateLimit-Reset`) to match against the response. Headers are tried in order, and matched case insensitive. The first header to be parsed successfully is used. If no headers match the default exponential back-off is used instead.

    [(validate.rules).repeated = {min_items: 1}];
    */
    reset_headers: Vec<ResetHeader>,

    /**
    Specifies the maximum back off interval that Envoy will allow. If a reset header contains an interval longer than this then it will be discarded and the next header will be tried. Defaults to 300 seconds.

    [(validate.rules).duration = {gt {}}];
    */
    max_interval: Duration
}

/// HTTP request hedging :ref:`architecture overview <arch_overview_http_routing_hedging>`.
pub struct HedgePolicy {
    /**
    Specifies the number of initial requests that should be sent upstream. Must be at least `1`. Defaults to `1`.
    [#not-implemented-hide:]
    [(validate.rules).u32 = {gte: 1}];
    */
    initial_requests: u32,

    /**
    Specifies a probability that an additional upstream request should be sent on top of what is specified by initial_requests. Defaults to `0`.
    [#not-implemented-hide:]
    */
    additional_request_chance: FractionalPercent,

    /**
    Indicates that a hedged request should be sent when the per-try timeout is hit.
    This means that a retry will be issued without resetting the original request, leaving multiple upstream requests in flight.
    The first request to complete successfully will be the one returned to the caller.

    - At any time, a successful response (i.e. not triggering any of the retry-on conditions) would be returned to the client.
    - Before per-try timeout, an error response (per retry-on conditions) would be retried immediately or returned ot the client
      if there are no more retries left.
    - After per-try timeout, an error response would be discarded, as a retry in the form of a hedged request is already in progress.

    Note: For this to have effect, you must have a [`RetryPolicy`][crate::config::route::route_components::RetryPolicy] that retries at least
    one error code and specifies a maximum number of retries.

    Defaults to `false`.
    */
    hedge_on_per_try_timeout: bool,
}

pub struct RedirectAction {
    /**
    When the scheme redirection take place, the following rules apply:
    1. If the source URI scheme is `http` and the port is explicitly set to `:80`, the port will be removed after the redirection;
    2. If the source URI scheme is `https` and the port is explicitly set to `:443`, the port will be removed after the redirection.
    */
    scheme_rewrite_specifier: SchemeRewriteSpecifier,

    /// The host portion of the URL will be swapped with this value.
    host_redirect: String,
    // [(validate.rules).string = {well_known_regex: HTTP_HEADER_VALUE strict: false}]

    /// The port value of the URL will be swapped with this value.
    port_redirect: u32,

    path_rewrite_specifier: PathRewriteSpecifier,
        
    /// The HTTP status code to use in the redirect response. The default response code is MOVED_PERMANENTLY (301).
    // [(validate.rules).enum = {defined_only: true}];
    response_code: RedirectResponseCode,

    /// Indicates that during redirection, the query portion of the URL will be removed. Default value is `false`.
    strip_query: bool,
}

pub enum PathRewriteSpecifier {
    /**
    The path portion of the URL will be swapped with this value.
    Please note that query string in path_redirect will override the request's query string and will not be stripped.

    For example, let's say we have the following routes:

    - match: { path: "/old-path-1" }
      redirect: { path_redirect: "/new-path-1" }
    - match: { path: "/old-path-2" }
      redirect: { path_redirect: "/new-path-2", strip-query: "true" }
    - match: { path: "/old-path-3" }
      redirect: { path_redirect: "/new-path-3?foo=1", strip_query: "true" }

    1. if request uri is "/old-path-1?bar=1", users will be redirected to "/new-path-1?bar=1"
    2. if request uri is "/old-path-2?bar=1", users will be redirected to "/new-path-2"
    3. if request uri is "/old-path-3?bar=1", users will be redirected to "/new-path-3?foo=1"

    [(validate.rules).string = {well_known_regex: HTTP_HEADER_VALUE strict: false}]
    */
    PathRedirect(String),

    /**
    Indicates that during redirection, the matched prefix (or path) should be swapped with this value. This option allows redirect URLs be dynamically created based on the request.

    > ATTENTION: Pay attention to the use of trailing slashes as mentioned in [`RouteAction::prefix_rewrite`].
    */
    // [(validate.rules).string = {well_known_regex: HTTP_HEADER_VALUE strict: false}];
    PrefixRewrite(String),
        
    /**
    Indicates that during redirect, portions of the path that match the pattern should be rewritten, even allowing the substitution of capture groups from the pattern into the new path as specified by the rewrite substitution string. This is useful to allow application paths to be rewritten in a way that is aware of segments with variable content like identifiers.

    Examples using Google's `RE2 <https://github.com/google/re2>`_ engine:

    - The path pattern `^/service/([^/]+)(/.*)$` paired with a substitution
      string of `\2/instance/\1` would transform `/service/foo/v1/api`
      into `/v1/api/instance/foo`.

    - The pattern `one` paired with a substitution string of `two` would
      transform `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/two/zzz`.

    - The pattern `^(.*?)one(.*)$` paired with a substitution string of
      `\1two\2` would replace only the first occurrence of `one`,
      transforming path `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/one/zzz`.

    - The pattern `(?i)/xxx/` paired with a substitution string of `/yyy/`
      would do a case-insensitive match and transform path `/aaa/XxX/bbb` to
      `/aaa/yyy/bbb`.
    */
    RegexRewrite(RegexMatchAndSubstitute),
}

pub enum SchemeRewriteSpecifier {
    /// The scheme portion of the URL will be swapped with "https".
    HTTPSRedirect(bool),

    /// The scheme portion of the URL will be swapped with this value.
    SchemeRedirect(String),
}

pub enum RedirectResponseCode {
    /// Moved Permanently HTTP Status Code - 301.
    MovedPermanently,

    /// Found HTTP Status Code - 302.
    Found,

    /// See Other HTTP Status Code - 303.
    SeeOther,

    /// Temporary Redirect HTTP Status Code - 307.
    TemporaryRedirect,

    /// Permanent Redirect HTTP Status Code - 308.
    PermanentRedirect,
  }

pub struct DirectResponseAction {
    /// Specifies the HTTP response status to be returned.
    // [(validate.rules).u32 = {lt: 600 gte: 200}];
    status: u16,

    /**
    Specifies the content of the response body. If this setting is omitted, no body is included in the generated response.

    > NOTE: Headers can be specified using `response_headers_to_add` in the enclosing
      [`Route`], [crate::config::route::route::RouteConfiguration] or
      [crate::config::route::route_components::VirtualHost].
    */
    body: DataSource,
}

// [#not-implemented-hide:]
pub struct NonForwardingAction {
}

pub struct Decorator {
    /**
    The operation name associated with the request matched to this route. If tracing is enabled, this information will be used as the span name reported for this request.

    > NOTE: For ingress (inbound) requests, or egress (outbound) responses, this value may be overridden by the :ref:`x-envoy-decorator-operation <config_http_filters_router_x-envoy-decorator-operation>` header.

    [(validate.rules).string = {min_len: 1}];
    */
    operation: String,

    /// Whether the decorated details should be propagated to the other party. The default is `true`.
    propagate: bool,
}

pub struct Tracing {
    /**
    Target percentage of requests managed by this HTTP connection manager that will be force traced if the :ref:`x-client-trace-id <config_http_conn_man_headers_x-client-trace-id>` header is set. This field is a direct analog for the runtime variable 'tracing.client_sampling' in the :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`. Default: `100%`.
    */
    client_sampling: FractionalPercent,

    /**
    Target percentage of requests managed by this HTTP connection manager that will be randomly selected for trace generation, if not requested by the client or not forced. This field is a direct analog for the runtime variable 'tracing.random_sampling' in the :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`. Default: `100%`.
    */
    random_sampling: FractionalPercent,

    /**
    Target percentage of requests managed by this HTTP connection manager that will be traced after all other sampling checks have been applied (client-directed, force tracing, random sampling). This field functions as an upper limit on the total configured sampling rate. For instance, setting client_sampling to 100% but overall_sampling to 1% will result in only 1% of client requests with the appropriate headers to be force traced. This field is a direct analog for the runtime variable 'tracing.global_enabled' in the :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`. Default: `100%`.
    */
    overall_sampling: FractionalPercent,

    /**
    A list of custom tags with unique tag name to create tags for the active span.
    It will take effect after merging with the [corresponding configuration][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.Tracing.custom_tags] configured in the HTTP connection manager. If two tags with the same name are configured each in the HTTP connection manager and the route level, the one configured here takes priority.
    */
    custom_tags: Vec<CustomTag>,
}

/**
A virtual cluster is a way of specifying a regex matching rule against certain important endpoints such that statistics are generated explicitly for the matched requests. The reason this is useful is that when doing prefix/path matching Envoy does not always know what the application considers to be an endpoint. Thus, it’s impossible for Envoy to generically emit per endpoint statistics. However, often systems have highly critical endpoints that they wish to get “perfect” statistics on. Virtual cluster statistics are perfect in the sense that they are emitted on the downstream side such that they include network level failures.

Documentation for :ref:`virtual cluster statistics <config_http_filters_router_vcluster_stats>`.

> NOTE: Virtual clusters are a useful tool, but we do not recommend setting up a virtual cluster for every application endpoint. This is both not easily maintainable and as well the matching and statistics output are not free.
*/
pub struct VirtualCluster {
    /// Specifies a list of header matchers to use for matching requests. Each specified header must match. The pseudo-headers `:path` and `:method` can be used to match the request path and method, respectively.
    headers: Vec<HeaderMatcher>,

    /// Specifies the name of the virtual cluster. The virtual cluster name as well as the virtual host name are used when emitting statistics. The statistics are emitted by the router filter and are documented :ref:`here <config_http_filters_router_stats>`.
    // [!is_empty()]
    name: String
}

/**
Global rate limiting :ref:`architecture overview <arch_overview_global_rate_limit>`.
Also applies to Local rate limiting :ref:`using descriptors <config_http_filters_local_rate_limit_descriptors>`.
*/
pub struct RateLimit {
    /**
    Refers to the stage set in the filter. The rate limit configuration only
    applies to filters with the same stage number. The default stage number is
    0.

    > note: The filter supports a range of 0 - 10 inclusively for stage numbers.

    [(validate.rules).u32 = {lte: 10}];
    */
    stage: u8,

    /// The key to be set in runtime to disable this rate limit configuration.
    disable_key: String,

    /**
    A list of actions that are to be applied for this rate limit configuration.
    Order matters as the actions are processed sequentially and the descriptor is composed by appending descriptor entries in that sequence. If an action cannot append a descriptor entry, no descriptor is generated for the configuration. See :ref:`composing actions <config_http_filters_rate_limit_composing_actions>` for additional documentation.

    [(validate.rules).repeated = {min_items: 1}];
    */
    actions: Vec<RateLimitAction>,

    /**
    An optional limit override to be appended to the descriptor produced by this rate limit configuration. If the override value is invalid or cannot be resolved from metadata, no override is provided. See :ref:`rate limit override <config_http_filters_rate_limit_rate_limit_override>` for more information.
    */
    limit: Override,
}

pub struct Override {
    override_specifier: OverrideSpecifier
}

pub enum OverrideSpecifier {
    // option (validate.required) = true;

    /// Limit override from dynamic metadata.
    DynamicMetadata(DynamicMetadata),
}

/// Fetches the override from the dynamic metadata.
pub struct DynamicMetadata {
    /**
    Metadata struct that defines the key and path to retrieve the struct value.
    The value must be a struct containing an integer `requests_per_unit` property and a 'unit' property with a value parseable to [RateLimitUnit enum][crate::types::rate_limit_unit::RateLimitUnit]
    */
    // [(validate.rules).message = {required: true}];
    metadata_key: MetadataKey
}

pub enum RateLimitAction {
    // option (validate.required) = true;

    /// Rate limit on source cluster.
    SourceCluster(SourceCluster),

    /// Rate limit on destination cluster.
    DestinationCluster(DestinationCluster),

    /// Rate limit on request headers.
    RequestHeaders(RequestHeaders),

    /// Rate limit on remote address.
    RemoteAddress(RemoteAddress),

    /// Rate limit on a generic key.
    GenericKey(GenericKey),

    /// Rate limit on the existence of request headers.
    HeaderValueMatch(HeaderValueMatch),

    /// Rate limit on metadata.
    Metadata(MetaData),

    /**
    Rate limit descriptor extension. See the rate limit descriptor extensions documentation.

    :ref:`HTTP matching input functions <arch_overview_matching_api>` are permitted as descriptor extensions. The input functions are only looked up if there is no rate limit descriptor extension matching the type URL.

    [#extension-category: envoy.rate_limit_descriptors]
    */
    Extension(TypedExtensionConfig),

    /// Rate limit on masked remote address.
    MaskedRemoteAddress(MaskedRemoteAddress),
}

/**
The following descriptor entry is appended to the descriptor:

```cpp
("source_cluster", "<local service cluster>")
```

\<local service cluster\> is derived from the :option:`--service-cluster` option.
*/
pub struct SourceCluster {
}

/**
The following descriptor entry is appended to the descriptor:

```cpp
("destination_cluster", "<routed target cluster>")
```

Once a request matches against a route table rule, a routed cluster is determined by one of the following [route table configuration][crate::config::route::route::RouteConfiguration] settings:

- [`Cluster`][ClusterSpecifier::Cluster] indicates the upstream cluster to route to.
- [`WeightedClusters`][ClusterSpecifier::WeightedClusters] chooses a cluster randomly from a set of clusters with attributed weight.
- [`ClusterHeader`][ClusterSpecifier::ClusterHeader] indicates which header in the request contains the target cluster.
*/
pub struct DestinationCluster {
}

/**
The following descriptor entry is appended when a header contains a key that matches the
`header_name`:

```cpp
("<descriptor_key>", "<header_value_queried_from_header>")
```
*/
pub struct RequestHeaders {
    /**
    The header name to be queried from the request headers. The header’s value is used to populate the value of the descriptor entry for the `descriptor_key`.

    [(validate.rules).string = {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}];
    */
    header_name: String,
        
    /// The key to use in the descriptor entry.
    // [!is_empty()]
    descriptor_key: String,

    /**
    If set to true, Envoy skips the descriptor while calling rate limiting service when header is not present in the request. By default it skips calling the rate limiting service if this header is not present in the request.
    */
    skip_if_absent: bool,
}

/**
The following descriptor entry is appended to the descriptor and is populated using the trusted address from :ref:`x-forwarded-for <config_http_conn_man_headers_x-forwarded-for>`:

```cpp
("remote_address", "<trusted address from x-forwarded-for>")
```
*/
pub struct RemoteAddress {
}

/*
The following descriptor entry is appended to the descriptor and is populated using the masked address from :ref:`x-forwarded-for <config_http_conn_man_headers_x-forwarded-for>`:

```cpp
("masked_remote_address", "<masked address from x-forwarded-for>")
```
*/
pub struct MaskedRemoteAddress {
    /**
    Length of prefix mask len for IPv4 (e.g. 0, 32).
    Defaults to 32 when unset.
    For example, trusted address from x-forwarded-for is `192.168.1.1`, the descriptor entry is ("masked_remote_address", "192.168.1.1/32"); if mask len is 24, the descriptor entry is ("masked_remote_address", "192.168.1.0/24").

    [(validate.rules).u8 = {lte: 32}];
    */
    v4_prefix_mask_len: u8,

    /**
    Length of prefix mask len for IPv6 (e.g. 0, 128).
    Defaults to 128 when unset.
    For example, trusted address from x-forwarded-for is `2001:abcd:ef01:2345:6789:abcd:ef01:234`, the descriptor entry is ("masked_remote_address", "2001:abcd:ef01:2345:6789:abcd:ef01:234/128"); if mask len is 64, the descriptor entry is ("masked_remote_address", "2001:abcd:ef01:2345::/64").

    [(validate.rules).u8 = {lte: 128}]
    */
    v6_prefix_mask_len: u8,
}

/**
The following descriptor entry is appended to the descriptor:

```cpp
("generic_key", "<descriptor_value>")
```
*/
pub struct GenericKey {
    /// The value to use in the descriptor entry.
    // [!is_empty()]
    descriptor_value: String,

    /// An optional key to use in the descriptor entry. If not set it defaults to 'generic_key' as the descriptor key.
    descriptor_key: String,
}

/**
The following descriptor entry is appended to the descriptor:

```cpp
("header_match", "<descriptor_value>")
```
*/
pub struct HeaderValueMatch {
    /// The key to use in the descriptor entry. Defaults to `header_match`.
    descriptor_key: String,

    /// The value to use in the descriptor entry.
    // [!is_empty()]
    descriptor_value: String,

    /**
    If set to true, the action will append a descriptor entry when the request matches the headers. If set to false, the action will append a descriptor entry when the request does not match the headers. The default value is `true`.
    */
    expect_match: bool,

    /**
    Specifies a set of headers that the rate limit action should match on. The action will check the request’s headers against all the specified headers in the config. A match will happen if all the headers in the config are present in the request with the same values (or based on presence if the value field is not in the config).

    [(validate.rules).repeated = {min_items: 1}];
    */
    headers: Vec<HeaderMatcher>
}

/**
The following descriptor entry is appended when the :ref:`dynamic metadata <well_known_dynamic_metadata>` contains a key value:

```cpp
("<descriptor_key>", "<value_queried_from_dynamic_metadata>")
```

> ATTENTION: This action has been deprecated in favor of the [`metadata`][crate::config::route::RateLimit.Action.MetaData] action
*/
pub struct DynamicMetaData {
    /// The key to use in the descriptor entry.
    // [!is_empty()]
    descriptor_key: String,

    /// Metadata struct that defines the key and path to retrieve the string value. A match will only happen if the value in the dynamic metadata is of type string.
    // [(validate.rules).message = {required: true}];
    metadata_key: MetadataKey,

    /// An optional value to use if `metadata_key` is empty. If not set and no value is present under the metadata_key then no descriptor is generated.
    default_value: String,
}

/**
The following descriptor entry is appended when the metadata contains a key value:

```cpp
("<descriptor_key>", "<value_queried_from_metadata>")
```
*/
pub struct MetaData {
    /// The key to use in the descriptor entry.
    // [!is_empty()]
    descriptor_key: String,

    /// Metadata struct that defines the key and path to retrieve the string value. A match will only happen if the value in the metadata is of type string.
    // [(validate.rules).message = {required: true}];
    metadata_key: MetadataKey,

    /// An optional value to use if `metadata_key` is empty. If not set and no value is present under the metadata_key then no descriptor is generated.
    default_value: String,

    /// Source of metadata.
    // [(validate.rules).enum = {defined_only: true}];
    source: Source
}

pub enum Source {
    /// Query :ref:`dynamic metadata <well_known_dynamic_metadata>`
    Dynamic,

    /// Query [route entry metadata][Route::.metadata]
    RouteEntry,
}

/**
> ATTENTION: Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1 `Host` header. Thus, if attempting to match on `Host`, match on `:authority` instead.

> ATTENTION: To route on HTTP method, use the special HTTP/2 `:method` header. This works for both HTTP/1 and HTTP/2 as Envoy normalises headers. E.g.,

```json
{
    "name": ":method",
    "exact_match": "POST"
}
```

> ATTENTION: In the absence of any header match specifier, match will default to [`PresentMatch`][HeaderMatchSpecifier::PresentMatch]. i.e, a request that has the [`name`][HeaderMatcher::name] header will match, regardless of the header's value.

> #next-major-version: HeaderMatcher should be refactored to use `StringMatcher`.
*/
pub struct HeaderMatcher {
    /// Specifies the name of the header in the request.
    // [(validate.rules).string = {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}];
    name: String,

    /// Specifies how the header match will be performed to route the request.
    header_match_specifier: HeaderMatchSpecifier,

    /**
    If specified, the match result will be inverted before checking. Defaults to false.

    Examples:

    - The regex `\d{3}` does not match the value `1234`, so it will match when inverted.
    - The range [-10,0) will match the value -1, so it will not match when inverted.
    */
    invert_match: bool,

    /**
    If specified, for any header match rule, if the header match rule specified header does not exist, this header value will be treated as empty. Defaults to false.

    Examples:

    - The header match rule specified header "header2" to range match of [0, 10], [`invert_match`][Self::invert_match] is set to `true` and `treat_missing_header_as_empty` is set to `false`; The "header2" header is not present and the header matcher rule for "header2" will be ignored so it will not match.
    - The header match rule specified header "header3" to a string regex match `^$` which means an empty string, and `treat_missing_header_as_empty` is set to `true`; The "header3" header is not present. The match rule will treat the "header3" header as an empty header so it will match.
    - The header match rule specified header "header4" to a string regex match `^$` which means an empty string, and :ref:`treat_missing_header_as_empty <Self::treat_missing_header_as_empty>` is set to false; The "header4" header is not present. The match rule for "header4" will be ignored so it will not match.
    */
    treat_missing_header_as_empty: bool,
}

pub enum HeaderMatchSpecifier {
    /**
    If specified, header match will be performed based on range.
    The rule will match if the request header value is within this range.
    The entire request header value must represent an integer in base 10 notation: consisting of an optional plus or minus sign followed by a sequence of digits. The rule will not match if the header value does not represent an integer. Match will fail for empty values, floating point numbers or if only a subsequence of the header value is an integer.

    Examples:

    - For range [-10,0), route will match for header value -1, but not for 0, `somestring`, 10.9,
      `-1somestring`
    */
    RangeMatch(I64Range),

    /// If specified as true, header match will be performed based on whether the header is in the request. If specified as false, header match will be performed based on whether the header is absent.
    PresentMatch(bool),

    /// If specified, header match will be performed based on the string match of the header value.
    StringMatch(StringMatcher),
}

/// Query parameter matching treats the query string of a request's :path header as an ampersand-separated list of keys and/or key=value elements.
pub struct QueryParameterMatcher {
    /// Specifies the name of a key that must be present in the requested `path`'s query string.
    // [(validate.rules).string = {min_len: 1 max_bytes: 1024}];
    name: String,

    query_parameter_match_specifier: QueryParameterMatchSpecifier
}

pub enum QueryParameterMatchSpecifier {
    /// Specifies whether a query parameter value should match against a string.
    // [(validate.rules).message = {required: true}];
    StringMatch(StringMatcher),

    /// Specifies whether a query parameter should be present.
    PresentMatch(bool),
}

/// HTTP Internal Redirect :ref:`architecture overview <arch_overview_internal_redirects>`.
pub struct InternalRedirectPolicy {
    /**
    An internal redirect is not handled, unless the number of previous internal redirects that a downstream request has encountered is lower than this value.
    In the case where a downstream request is bounced among multiple routes by internal redirect, the first route that hits this threshold, or does not set :ref:`internal_redirect_policy <RouteAction::internal_redirect_policy>` will pass the redirect back to downstream.

    If not specified, at most one redirect will be followed.
    */
    max_internal_redirects: u32,

    /**
    Defines what upstream response codes are allowed to trigger internal redirect. If unspecified, only 302 will be treated as internal redirect.
    Only 301, 302, 303, 307 and 308 are valid values. Any other codes will be ignored.

    [(validate.rules).repeated = {max_items: 5}];
    */
    redirect_response_codes: Vec<u32>,

    /**
    Specifies a list of predicates that are queried when an upstream response is deemed to trigger an internal redirect by all other criteria. Any predicate in the list can reject the redirect, causing the response to be proxied to downstream.
    [#extension-category: envoy.internal_redirect_predicates]
    */
    predicates: Vec<TypedExtensionConfig>,

    /// Allow internal redirect to follow a target URI with a different scheme than the value of x-forwarded-proto. The default is `false`.
    allow_cross_scheme_redirect: bool,
}

/**
A simple wrapper for an HTTP filter config. This is intended to be used as a wrapper for the map value in [`VirtualHost::typed_per_filter_config`] [VirtualHost::typed_per_filter_config], [`Route::typed_per_filter_config`][Route::typed_per_filter_config], or [`ClusterWeight::typed_per_filter_config`][ClusterWeight::typed_per_filter_config] to add additional flags to the filter.
*/
pub struct FilterConfig {
    /// The filter config.
    config: Any,

    /// If `true`, the filter is optional, meaning that if the client does not support the specified filter, it may ignore the map entry rather than rejecting the config.
    is_optional: bool,
}
