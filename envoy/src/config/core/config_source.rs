/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/config_source.proto>
*/

type Any = String;

use std::time::Duration;

use xds::core::authority::Authority;

use crate::config::core::{
    base::WatchedDirectory,
    extension::TypedExtensionConfig,
    grpc_service::GRPCService
};

// xDS API and non-xDS services version. This is used to describe both resource and transport
// protocol versions (in distinct configuration fields).
pub enum APIVersion {
    // Use xDS v3 API.
    V3
}

// API configuration source. This identifies the API type and cluster that Envoy will use to fetch an xDS API.
pub struct APIConfigSource {
    /// API type (gRPC, REST, delta gRPC)
    // [(validate.rules).enum = {defined_only: true}];
    api_type: APIType,

    /**
    API version for xDS transport protocol. This describes the xDS gRPC/REST endpoint and version of \[Delta\]DiscoveryRequest/Response used on the wire.
    */
    // [(validate.rules).enum = {defined_only: true}];
    transport_api_versiona: APIVersion,

    /**
    Cluster names should be used only with REST. If > 1 cluster is defined, clusters will be cycled through if any kind of failure occurs.

    > NOTE: The cluster with name `cluster_name` must be statically defined and its type must not be `EDS`.
    */
    cluster_names: Vec<String>,

    /**
    Multiple gRPC services be provided for GRPC. If > 1 cluster is defined, services will be cycled through if any kind of failure occurs.
    */
    grpc_services: Vec<GRPCService>,

    /// For REST APIs, the delay between successive polls.
    refresh_delay: Duration,

    /**
    For REST APIs, the request timeout. If not set, a default value of 1s will be used.
    */
    // [(validate.rules).duration = {gt {}}];
    request_timeout: Duration,

    /**
    For GRPC APIs, the rate limit settings. If present, discovery requests made by Envoy will be rate limited.
    */
    rate_limit_settings: RateLimitSettings,

    /// Skip the node identifier in subsequent discovery requests for streaming gRPC config types.
    set_node_on_first_message_only: bool,

    /**
    A list of config validators that will be executed when a new update is received from the ApiConfigSource. Note that each validator handles a specific xDS service type, and only the validators corresponding to the type url (in `DiscoveryResponse` or `DeltaDiscoveryResponse`) will be invoked.
    If the validator returns false or throws an exception, the config will be rejected by the client, and a NACK will be sent.
    */
    // [#extension-category: envoy.config.validators]
    config_validators: Vec<TypedExtensionConfig>,
}

/// APIs may be fetched via either REST or gRPC.
pub enum APIType {
    /**
    REST-JSON v2 API. The [canonical JSON encoding](https://developers.google.com/protocol-buffers/docs/proto3#json) for the v2 protos is used.
    */
    REST,

    /// SotW gRPC service.
    GRPC,

    /**
    Using the delta xDS gRPC service, i.e. DeltaDiscovery{Request,Response} rather than Discovery{Request,Response}. Rather than sending Envoy the entire state with every update, the xDS server only sends what has changed since the last update.
    */
    DeltaGRPC,

    /**
    SotW xDS gRPC with ADS. All resources which resolve to this configuration source will be multiplexed on a single connection to an ADS endpoint.
    */
    AggregatedGRPC,

    /**
    Delta xDS gRPC with ADS. All resources which resolve to this configuration source will be multiplexed on a single connection to an ADS endpoint.
    */
    AggregatedDeltaGRPC,
}

/*
Aggregated Discovery Service (ADS) options. This is currently empty, but when set in [`ConfigSource`] can be used to specify that ADS is to be used.
*/
pub struct AggregatedConfigSource {
}

/**
Self-referencing config source options. This is currently empty, but when set in [`ConfigSource`] can be used to specify that other data can be obtained from the same server.
*/
pub struct SelfConfigSource {
    /**
    API version for xDS transport protocol. This describes the xDS gRPC/REST endpoint and version of \[Delta\]DiscoveryRequest/Response used on the wire.
    */
    // [(validate.rules).enum = {defined_only: true}];
    transport_api_version: APIVersion
}

/// Rate Limit settings to be applied for discovery requests made by Envoy.
pub struct RateLimitSettings {
    /**
    Maximum number of tokens to be used for rate limiting discovery request calls. If not set, a default value of 100 will be used.
    */
    max_tokens: u32,

    /**
    Rate at which tokens will be filled per second. If not set, a default fill rate of 10 tokens per second will be used.

    [(validate.rules).double = {gt: 0.0}];
    */
    fill_rate: f64
}

/// Local filesystem path configuration source.
pub struct PathConfigSource {
    /**
    Path on the filesystem to source and watch for configuration updates.
    When sourcing configuration for a [secret][crate::extensions::transport_sockets::tls::secret::Secret], the certificate and key files are also watched for updates.

    > NOTE: The path to the source must exist at config load time.

    > NOTE: If `watched_directory` is *not* configured, Envoy will watch the file path for *moves*.
    > This is because in general only moves are atomic. The same method of swapping files as is demonstrated in the :ref:`runtime documentation <config_runtime_symbolic_link_swap>` can be used here also. If `watched_directory` is configured, no watch will be placed directly on this path. Instead, the configured `watched_directory` will be used to trigger reloads of this path. This is required in certain deployment scenarios. See below for more information.

    [(validate.rules).string = {min_len: 1}];
    */
    path: String,

    /**
    If configured, this directory will be watched for *moves*. When an entry in this directory is moved to, the `path` will be reloaded. This is required in certain deployment scenarios.

    Specifically, if trying to load an xDS resource using a
    [Kubernetes ConfigMap](https://kubernetes.io/docs/concepts/configuration/configmap/), the following configuration might be used:
    1. Store xds.yaml inside a ConfigMap.
    2. Mount the ConfigMap to `/config_map/xds`
    3. Configure path `/config_map/xds/xds.yaml`
    4. Configure watched directory `/config_map/xds`

    The above configuration will ensure that Envoy watches the owning directory for moves which is required due to how Kubernetes manages ConfigMap symbolic links during atomic updates.
    */
    watched_directory: WatchedDirectory,
}

/**
Configuration for `listeners <config_listeners>`, `clusters
<config_cluster_manager>`, [routes][crate::config::route::route::RouteConfiguration], `endpoints <arch_overview_service_discovery>` etc. may either be sourced from the filesystem or from an xDS API source. Filesystem configs are watched with
inotify for updates.
*/
pub struct ConfigSource {
    /**
    Authorities that this config source may be used for. An authority specified in a xdstp:// URL is resolved to a `ConfigSource` prior to configuration fetch. This field provides the association between authority name and configuration source.
    */
    authorities: Vec<Authority>,

    config_source_specifier: ConfigSourceSpecifier,

    /**
    When this timeout is specified, Envoy will wait no longer than the specified time for first config response on this xDS subscription during the :ref:`initialisation process <arch_overview_initialisation>`. After reaching the timeout, Envoy will move to the next initialisation phase, even if the first config is not delivered yet. The timer is activated when the xDS API subscription starts, and is disarmed on first config update or on error. 0 means no timeout - Envoy will wait indefinitely for the first xDS config (unless another timeout applies). The default is 15s.
    */
    initial_fetch_timeout: Duration,

    /**
    API version for xDS resources. This implies the type URLs that the client will request for resources and the resource type that the client will in turn expect to be delivered.

    [(validate.rules).enum = {defined_only: true}];
    */
    resource_api_version: APIVersion
}

pub enum ConfigSourceSpecifier {
    // option (validate.required) = true;

    /// Local filesystem path configuration source.
    PathConfigSource(PathConfigSource),

    /// API configuration source.
    APIConfigSource(APIConfigSource),

    /**
    When set, ADS will be used to fetch resources. The ADS API configuration source in the bootstrap configuration is used.
    */
    ADS(AggregatedConfigSource),

    /**
    When set, the client will access the resources from the same server it got the `ConfigSource` from, although not necessarily from the same stream. This is similar to the [`ADS`][ConfigSourceSpecifier::ADS] field, except that the client may use a different stream to the same server. As a result, this field can be used for things like LRS that cannot be sent on an ADS stream. It can also be used to link from (e.g.)
    LDS to RDS on the same server without requiring the management server to know its name or required credentials.
    [#next-major-version: In xDS v3, consider replacing the ads field with this one, since this field can implicitly mean to use the same stream in the case where the `ConfigSource` is provided via ADS and the specified data can also be obtained via ADS.]
    */
    SelfConfigSource(SelfConfigSource),
}

/**
Configuration source specifier for a late-bound extension configuration. The parent resource is warmed until all the initial extension configurations are received, unless the flag to apply the default configuration is set.
Subsequent extension updates are atomic on a per-worker basis. Once an extension configuration is applied to a request or a connection, it remains constant for the duration of processing. If the initial delivery of the extension configuration fails, due to a timeout for example, the optional default configuration is applied. Without a default configuration, the extension is disabled, until an extension configuration is received. The behaviour of a disabled extension depends on the context. For example, a filter chain with a disabled extension filter rejects all incoming streams.
*/
pub struct ExtensionConfigSource {
    // [(validate.rules).any = {required: true}];
    config_source: ConfigSource,

    /**
    Optional default configuration to use as the initial configuration if there is a failure to receive the initial extension configuration or if `apply_default_config_without_warming` flag is set.
    */
    default_config: Any,

    /**
    Use the default config as the initial configuration without warming and waiting for the first discovery response. Requires the default configuration to be supplied.
    */
    apply_default_config_without_warming: bool,

    /**
    A set of permitted extension type URLs. Extension configuration updates are rejected if they do not match any type URL in the set.

    [(validate.rules).repeated = {min_items: 1}];
    */
    type_urls: Vec<String>
}
