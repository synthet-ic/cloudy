/*!
- <https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/bootstrap/v3/bootstrap.proto>
- <https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/bootstrap/v3/bootstrap.proto>
*/

type Any = String;
type Struct = String;

use std::{
    collections::HashMap,
    time::Duration,
};

use crate::{
    config::{
        accesslog::AccessLog,
        cluster::cluster::Cluster,
        core::{
            address::{Address, BindConfig},
            base::Node,
            config_source::{APIConfigSource, ConfigSource},
            event_service_config::EventServiceConfig,
            extension::TypedExtensionConfig,
            // resolver,
            socket_option::SocketOption,
        },
        listener::listener::Listener,
        metrics::stats::{StatsConfig, StatsSink},
        overload::OverloadManager,
        // trace
    },
    extensions::transport_sockets::tls::secret::Secret,
    types::percent::Percent,
};

/// Bootstrap :ref:`configuration overview <config_overview_bootstrap>`.
pub struct Bootstrap {
    /**
    Node identity to present to the management server and for instance identification purposes (e.g. in generated headers).
    */
    node: Node,

    /**
    A list of [`Node`][crate::config::core::base::Node] field names that will be included in the context parameters of the effective xdstp:// URL that is sent in a discovery request when resource locators are used for LDS/CDS. Any non-string field will have its JSON encoding set as the context parameter value, with the exception of metadata, which will be flattened (see example below). The supported field names are:
    - "cluster"
    - "id"
    - "locality.region"
    - "locality.sub_zone"
    - "locality.zone"
    - "metadata"
    - "user_agent_build_version.metadata"
    - "user_agent_build_version.version"
    - "user_agent_name"
    - "user_agent_version"

    The node context parameters act as a base layer dictionary for the context parameters (i.e. more specific resource specific context parameters will override). Field names will be prefixed with “udpa.node.” when included in context parameters.

    For example, if node_context_params is `["user_agent_name", "metadata"]`, the implied context parameters might be::

      node.user_agent_name: "envoy"
      node.metadata.foo: "{\"bar\": \"baz\"}"
      node.metadata.some: "42"
      node.metadata.thing: "\"thing\""

    */
    node_context_params: Vec<String>,

    /// Statically specified resources.
    static_resources: StaticResources,

    /// xDS configuration sources.
    dynamic_resources: DynamicResources,

    /**
    Configuration for the cluster manager which owns all upstream clusters within the server.
    */
    cluster_manager: ClusterManager,

    /**
    Health discovery service config option. [`core::APIConfigSource`][crate::config::core::config_source::APIConfigSource])
    */
    hds_config: APIConfigSource,

    /// Optional file system path to search for startup flag files.
    flags_path: String,

    /// Optional set of stats sinks.
    stats_sinks: Vec<StatsSink>,

    /// Configuration for internal processing of stats.
    stats_config: StatsConfig,

    /**
    Optional duration between flushes to configured stats sinks. For performance reasons Envoy latches counters and only flushes counters and gauges at a periodic interval. If not specified the default is 5000ms (5 seconds). Only one of `stats_flush_interval` or `stats_flush_on_admin` can be set.
    Duration must be at least 1ms and at most 5 min.
    */
    // [
    //     (validate.rules).duration = {
    //     lt {seconds: 300}
    //     gte {nanos: 1000000}
    //     },
    //     (udpa.annotations.field_migrate).oneof_promotion = "stats_flush"
    // ];
    stats_flush_interval: Duration,

    stats_flush: StatsFlush,

    /**
    Optional watchdogs configuration.
    This is used for specifying different watchdogs for the different subsystems.
    [#extension-category: envoy.guarddog_actions]
    */
    watchdogs: Watchdogs,

    /**
    Configuration for the runtime configuration provider. If not specified, a 'null' provider will be used which will result in all defaults being used.
    */
    layered_runtime: LayeredRuntime,

    /// Configuration for the local administration HTTP server.
    admin: Admin,

    /**
    Optional overload manager configuration.
    */
    // [
    //     (udpa.annotations.security).configure_for_untrusted_downstream = true,
    //     (udpa.annotations.security).configure_for_untrusted_upstream = true
    // ];
    overload_manager: OverloadManager,

    /**
    Enable :ref:`stats for event dispatcher <operations_performance>`, defaults to false.
    Note that this records a value for each iteration of the event loop on every thread. This should normally be minimal overhead, but when using
    [statsd][crate::config::metrics::stats::StatsdSink], it will send each observed value over the wire individually because the statsd protocol doesn't have any way to represent a histogram summary. Be aware that this can be a very large volume of data.
    */
    enable_dispatcher_stats: bool,

    /**
    Optional string which will be used in lieu of x-envoy in prefixing headers.

    For example, if this string is present and set to X-Foo, then x-envoy-retry-on will be transformed into x-foo-retry-on etc.

    Note this applies to the headers Envoy will generate, the headers Envoy will sanitize, and the headers Envoy will trust for core code and core extensions only. Be VERY careful making changes to this string, especially in multi-layer Envoy deployments or deployments using extensions which are not upstream.
    */
    header_prefix: String,

    /**
    Optional proxy version which will be used to set the value of :ref:`server.version statistic <server_statistics>` if specified. Envoy will not process this value, it will be sent as is to [stats sinks][crate::config::metrics::stats::StatsSink].
    */
    stats_server_version_override: u64,

    /**
    DNS resolver type configuration extension. This extension can be used to configure c-ares, apple, or any other DNS resolver types and the related parameters. For example, an object of [`CaresDNSResolverConfig`][crate::extensions::network::dns_resolver::cares::cares_dns_resolver::CaresDNSResolverConfig] can be packed into this `typed_dns_resolver_config`.
    When `typed_dns_resolver_config` is missing, the default behaviour is in place.
    */
    // [#extension-category: envoy.network.dns_resolver]
    typed_dns_resolver_config: TypedExtensionConfig,

    /**
    Specifies optional bootstrap extensions to be instantiated at startup time.
    Each item contains extension specific configuration.
    */
    // [#extension-category: envoy.bootstrap]
    bootstrap_extensions: Vec<TypedExtensionConfig>,

    /**
    Specifies optional extensions instantiated at startup time and invoked during crash time on the request that caused the crash.
    */
    fatal_actions: Vec<FatalAction>,

    /**
    Configuration sources that will participate in xdstp:// URL authority resolution. The algorithm is as follows:
    1. The authority field is taken from the xdstp:// URL, call this `resource_authority`.
    2. `resource_authority` is compared against the authorities in any peer `ConfigSource`. The peer `ConfigSource` is the configuration source message which would have been used unconditionally for resolution with opaque resource names. If there is a match with an authority, the peer `ConfigSource` message is used.
    3. `resource_authority` is compared sequentially with the authorities in each configuration source in `config_sources`. The first `ConfigSource` to match wins.
    4. As a fallback, if no configuration source matches, then `default_config_source` is used.
    5. If `default_config_source` is not specified, resolution fails.
    */
    config_sources: Vec<ConfigSource>,

    /**
    Default configuration source for xdstp:// URLs if all other resolution fails.
    */
    default_config_source: ConfigSource,

    /**
    Optional overriding of default socket interface. The value must be the name of one of the socket interface factories initialised through a bootstrap extension.
    */
    default_socket_interface: String,

    /**
    Global map of CertificateProvider instances. These instances are referred to by name in the [`CertificateProviderInstance::instance_name`][crate::extensions::transport_sockets::tls::tls::CertificateProviderInstance::instance_name] field.
    */
    certificate_provider_instances: HashMap<String, TypedExtensionConfig>,

    /**
    Specifies a set of headers that need to be registered as inline header. This configuration allows users to customize the inline headers on-demand at Envoy startup without modifying Envoy's source code.

    Note that the 'set-cookie' header cannot be registered as inline header.
    */
    inline_headers: Vec<CustomInlineHeader>,

    /**
    Optional path to a file with performance tracing data created by "Perfetto" SDK in binary
    ProtoBuf format. The default value is "envoy.pftrace".
    */
    perf_tracing_file_path: String,

    /**
    Optional overriding of default regex engine.
    If the value is not specified, Google RE2 will be used by default.
    */
    // [#extension-category: envoy.regex_engines]
    default_regex_engine: TypedExtensionConfig,

    /**
    Optional XdsResourcesDelegate configuration, which allows plugging custom logic into both fetch and load events during xDS processing.
    If a value is not specified, no XdsResourcesDelegate will be used.
    TODO(abeyad): Add public-facing documentation.
    */
    xds_delegate_extension: TypedExtensionConfig,
}

pub enum StatsFlush {
    /**
    Flush stats to sinks only when queried for on the admin interface. If set,
    a flush timer is not created. Only one of `stats_flush_on_admin` or
    `stats_flush_interval` can be set.
    [(validate.rules).bool = {const: true}];
    */
    StatsFlushOnAdmin(bool)
}

pub struct StaticResources {
    /**
    Static [Listeners][Listener]. These listeners are available regardless of LDS configuration.
    */
    listeners: Vec<Listener>,

    /**
    If a network based configuration source is specified for [`cds_config`][crate::config::bootstrap::DynamicResources::cds_config], it's necessary to have some initial cluster definitions available to allow Envoy to know how to speak to the management server. These cluster definitions may not use :ref:`EDS <arch_overview_dynamic_config_eds>` (i.e. they should be static IP or DNS-based).
    */
    clusters: Vec<Cluster>,

    /**
    These static secrets can be used by [`SDSSecretConfig`][crate::extensions::transport_sockets::tls::secret::SDSSecretConfig]
    */
    secrets: Vec<Secret>,
}

pub struct DynamicResources {
    /**
    All [Listeners][Listener] are provided by a single :ref:`LDS <arch_overview_dynamic_config_lds>` configuration source.
    */
    lds_config: ConfigSource,

    /// xdstp:// resource locator for listener collection.
    lds_resources_locator: String,

    /**
    All post-bootstrap [`Cluster`] definitions are provided by a single :ref:`CDS <arch_overview_dynamic_config_cds>` configuration source.
    */
    cds_config: ConfigSource,

    /// xdstp:// resource locator for cluster collection.
    cds_resources_locator: String,

    /**
    A single :ref:`ADS <config_overview_ads>` source may be optionally specified. This must have [`api_type`][APIConfigSource::api_type] [`GRPC`][crate::config::core::config_source::APIType::GRPC]. Only [`ConfigSource`]s that have the [`ADS`][crate::config::core::config_source::ConfigSourceSpecifier::ADS] field set will be streamed on the ADS channel.
    */
    ads_config: APIConfigSource,
}

/**
Administration interface :ref:`operations documentation <operations_admin_interface>`.
*/
pub struct Admin {
    /**
    Configuration for :ref:`access logs <arch_overview_access_logs>` emitted by the administration server.
    */
    access_log: Vec<AccessLog>,

    /**
    The CPU profiler output path for the administration server. If no profile path is specified, the default is `/var/log/envoy/envoy.prof`.
    */
    profile_path: String,

    /**
    The TCP address that the administration server will listen on.
    If not specified, Envoy will not start an administration server.
    */
    address: Address,

    /**
    Additional socket options that may not be present in Envoy source code or precompiled binaries.
    */
    socket_options: Vec<SocketOption>,

    /**
    Indicates whether :ref:`global_downstream_max_connections <config_overload_manager_limiting_connections>` should apply to the admin interface or not.
    */
    ignore_global_conn_limit: bool,
}

/// Cluster manager :ref:`architecture overview <arch_overview_cluster_manager>`.
pub struct ClusterManager {
    /**
    Name of the local cluster (i.e., the cluster that owns the Envoy running this configuration). In order to enable :ref:`zone aware routing <arch_overview_load_balancing_zone_aware_routing>` this option must be set.
    If `local_cluster_name` is defined then [clusters][crate::config::cluster::cluster::Cluster] must be defined in the [Bootstrap
    static cluster resources][crate::config::bootstrap::Bootstrap.StaticResources.clusters>`. This is unrelated to
    the :option:`--service-cluster` option which does not `affect zone aware
    routing <https://github.com/envoyproxy/envoy/issues/774>`_.
    */
    local_cluster_name: String,

    /// Optional global configuration for outlier detection.
    outlier_detection: OutlierDetection,

    /**
    Optional configuration used to bind newly established upstream connections.
    This may be overridden on a per-cluster basis by upstream_bind_config in the cds_config.
    */
    upstream_bind_config: BindConfig,

    /**
    A management server endpoint to stream load stats to via `StreamLoadStats`. This must have [`api_type`][APIConfigSource::api_type] [`GRPC`][crate::config::core::config_source::APIType.GRPC].
    */
    load_stats_config: APIConfigSource,
}

pub struct OutlierDetection {
    /// Specifies the path to the outlier event log.
    event_log_path: String,

    /**
    The gRPC service for the outlier detection event service.
    If empty, outlier detection events won't be sent to a remote endpoint.
    */
    event_service: EventServiceConfig,
}

/**
Allows you to specify different watchdog configs for different subsystems.
This allows finer tuned policies for the watchdog. If a subsystem is omitted the default values for that system will be used.
*/
pub struct Watchdogs {
    /// Watchdog for the main thread.
    main_thread_watchdog: Watchdog,

    /// Watchdog for the worker threads.
    worker_watchdog: Watchdog,
}

/**
Envoy process watchdog configuration. When configured, this monitors for nonresponsive threads and kills the process after the configured thresholds.
See the :ref:`watchdog documentation <operations_performance_watchdog>` for more information.
*/
pub struct Watchdog {
    /**
    Register actions that will fire on given WatchDog events.
    See `WatchDogAction` for priority of events.
    */
    actions: Vec<WatchdogAction>,

    /**
    The duration after which Envoy counts a nonresponsive thread in the `watchdog_miss` statistic. If not specified the default is 200ms.
    */
    miss_timeout: Duration,

    /**
    The duration after which Envoy counts a nonresponsive thread in the `watchdog_mega_miss` statistic. If not specified the default is 1000ms.
    */
    megamiss_timeout: Duration,

    /**
    If a watched thread has been nonresponsive for this duration, assume a programming error and kill the entire Envoy process. Set to 0 to disable kill behaviour. If not specified the default is 0 (disabled).
    */
    kill_timeout: Duration,

    /**
    Defines the maximum jitter used to adjust the `kill_timeout` if `kill_timeout` is enabled. Enabling this feature would help to reduce risk of synchronised watchdog kill events across proxies due to external triggers. Set to 0 to disable. If not specified the default is 0 (disabled).
    */
    // [(validate.rules).duration = {gte {}}];
    max_kill_timeout_jitter: Duration,

    /**
    If `max(2, ceil(registered_threads * Fraction(*multikill_threshold*)))` threads have been nonresponsive for at least this duration kill the entire
    Envoy process. Set to 0 to disable this behaviour. If not specified the default is 0 (disabled).
    */
    multikill_timeout: Duration,

    /**
    Sets the threshold for `multikill_timeout` in terms of the percentage of nonresponsive threads required for the `multikill_timeout`.
    If not specified the default is 0.
    */
    multikill_threshold: Percent,
}

pub struct WatchdogAction {
    /// Extension specific configuration for the action.
    config: TypedExtensionConfig,

    // [(validate.rules).enum = {defined_only: true}];
    event:WatchdogEvent
}

/**
The events are fired in this order: KILL, MULTIKILL, MEGAMISS, MISS.
Within an event type, actions execute in the order they are configured.
For KILL/MULTIKILL there is a default PANIC that will run after the registered actions and kills the process if it wasn't already killed.
It might be useful to specify several debug actions, and possibly an alternate FATAL action.
*/
pub enum WatchdogEvent {
    Unknown,
    Kill,
    Multikill,
    Megamiss,
    Miss,
}

/**
Fatal actions to run while crashing. Actions can be safe (meaning they are async-signal safe) or unsafe. We run all safe actions before we run unsafe actions.
If using an unsafe action that could get stuck or deadlock, it important to have an out of band system to terminate the process.

The interface for the extension is `Envoy::Server::Configuration::FatalAction`. `FatalAction` extensions live in the `envoy.extensions.fatal_actions` API namespace.
*/
pub struct FatalAction {
    /**
    Extension specific configuration for the action. It's expected to conform to the `Envoy::Server::Configuration::FatalAction` interface.
    */
    config: TypedExtensionConfig,
}

/// Runtime :ref:`configuration overview <config_runtime>` (deprecated).
pub struct Runtime {
    /**
    The implementation assumes that the file system tree is accessed via a symbolic link. An atomic link swap is used when a new tree should be switched to. This parameter specifies the path to the symbolic link. Envoy will watch the location for changes and reload the file system tree when they happen. If this parameter is not set, there will be no disc based runtime.
    */
    symlink_root: String,

    /**
    Specifies the subdirectory to load within the root directory. This is useful if multiple systems share the same delivery mechanism. Envoy configuration elements can be contained in a dedicated subdirectory.
    */
    subdirectory: String,

    /**
    Specifies an optional subdirectory to load within the root directory. If specified and the directory exists, configuration values within this directory will override those found in the primary subdirectory. This is useful when Envoy is deployed across many different types of servers.
    Sometimes it is useful to have a per service cluster directory for runtime configuration. See below for exactly how the override directory is used.
    */
    override_subdirectory: String,

    /**
    Static base runtime. This will be :ref:`overridden <config_runtime_layering>` by other runtime layers, e.g. disc or admin. This follows the :ref:`runtime protobuf JSON representation encoding <config_runtime_proto_json>`.
    */
    base: Struct,
}

pub struct RuntimeLayer {
    /**
    Descriptive name for the runtime layer. This is only used for the runtime
    :http:get:`/runtime` output.
    [(validate.rules).String = {min_len: 1}];
    */
    name: String,

    layer_specifier: LayerSpecifier
}

pub enum LayerSpecifier {
    // option (validate.required) = true;

    /**
    :ref:`Static runtime <config_runtime_bootstrap>` layer.
    This follows the :ref:`runtime protobuf JSON representation encoding
    <config_runtime_proto_json>`. Unlike static xDS resources, this static
    layer is overridable by later layers in the runtime virtual filesystem.
    */
    StaticLayer(Struct),

    DiscLayer(DiscLayer),

    AdminLayer(AdminLayer),

    RTDSLayer(RTDSLayer),
}

/// :ref:`Disc runtime <config_runtime_local_disk>` layer.
pub struct DiscLayer {
    /**
    The implementation assumes that the file system tree is accessed via a symbolic link. An atomic link swap is used when a new tree should be switched to. This parameter specifies the path to the symbolic link.
    Envoy will watch the location for changes and reload the file system tree when they happen. See documentation on runtime :ref:`atomicity <config_runtime_atomicity>` for further details on how reloads are treated.
    */
    symlink_root: String,

    /**
    Specifies the subdirectory to load within the root directory. This is useful if multiple systems share the same delivery mechanism. Envoy configuration elements can be contained in a dedicated subdirectory.
    */
    subdirectory: String,

    /**
    :ref:`Append <config_runtime_local_disk_service_cluster_subdirs>` the
    service cluster to the path under symlink root.
    */
    append_service_cluster: bool,
}

/// :ref:`Admin console runtime <config_runtime_admin>` layer.
pub struct AdminLayer {
}

/// :ref:`Runtime Discovery Service (RTDS) <config_runtime_rtds>` layer.
pub struct RTDSLayer {
    /// Resource to subscribe to at `rtds_config` for the RTDS layer.
    name: String,

    /// RTDS configuration source.
    rtds_config: ConfigSource,
}

/// Runtime :ref:`configuration overview <config_runtime>`.
pub struct LayeredRuntime {
    /**
    The :ref:`layers <config_runtime_layering>` of the runtime. This is ordered such that later layers in the list overlay earlier entries.
    */
    layers: Vec<RuntimeLayer>,
}

/**
Used to specify the header that needs to be registered as an inline header.

If request or response contain multiple headers with the same name and the header name is registered as an inline header. Then multiple headers will be folded into one, and multiple header values will be concatenated by a suitable delimiter.
The delimiter is generally a comma.

For example, if 'foo' is registered as an inline header, and the headers contains the following two headers:

```text
foo: bar
foo: eep
```

Then they will eventually be folded into:

```text
foo: bar, eep
```

Inline headers provide O(1) search performance, but each inline header imposes an additional memory overhead on all instances of the corresponding type of
HeaderMap or TrailerMap.
*/
pub struct CustomInlineHeader {
    /// The name of the header that is expected to be set as the inline header.
    // [(validate.rules).String = {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}];
    inline_header_name: String,

    /// The type of the header that is expected to be set as the inline header.
    // [(validate.rules).enum = {defined_only: true}];
    inline_header_type: InlineHeaderType
}

pub enum InlineHeaderType {
    RequestHeader,
    RequestTrailer,
    ResponseHeader,
    ResponseTrailer,
}
