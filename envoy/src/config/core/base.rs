/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/base.proto>
*/

type Any = String;
type Struct = String;

use std::collections::HashMap;

use xds::core::context_params::ContextParams;

use crate::{
    config::core::http_uri::HTTPURI,
    types::{
        percent::{Percent, FractionalPercent},
        semantic_version::SemanticVersion
    }
};
use super::backoff::BackoffStrategy;

/**
Envoy supports :ref:`upstream priority routing <arch_overview_http_routing_priority>` both at the route and the virtual cluster level. The current priority implementation uses different connection pool and circuit breaking settings for each priority level. This means that even for HTTP/2 requests, two physical connections will be used to an upstream host. In the future Envoy will likely support true HTTP/2 priority over a single upstream connection.
*/
pub enum RoutingPriority {
    Default,
    High,
}
  
/// HTTP request method.
pub enum RequestMethod {
    Unspecified,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
  
/// Identifies the direction of the traffic relative to the local Envoy.
pub enum TrafficDirection {
    /// Default option is unspecified.
    Unspecified,
  
    /// The transport is used for incoming traffic.
    Inbound,
  
    /// The transport is used for outgoing traffic.
    Outbound
}

/// Identifies location of where either Envoy runs or where upstream hosts run.
pub struct Locality {
    /// Region this [`zone`][Self::zone] belongs to.
    region: String,

    /// Defines the local service zone where Envoy is running. Though optional, it should be set if discovery service routing is used and the discovery service exposes [zone data][crate::config::endpoint::endpoint_components::LocalityLBEndpoints::locality], either in this message or via :option:`--service-zone`. The meaning of zone is context dependent, e.g. [Availability Zone (AZ)](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html) on AWS, [Zone](https://cloud.google.com/compute/docs/regions-zones/) on GCP, etc.
    zone: String,

    /// When used for locality of upstream hosts, this field further splits zone into smaller chunks of sub-zones so they can be load balanced independently.
    sub_zone: String,
}

/// Identifies a specific Envoy instance. The node identifier is presented to the management server, which may use this identifier to distinguish per Envoy configuration for serving.
pub struct Node {
    /// An opaque node identifier for the Envoy node. This also provides the local service node name. It should be set if any of the following features are used: :ref:`statsd <arch_overview_statistics>`, :ref:`CDS <config_cluster_manager_cds>`, and :ref:`HTTP tracing <arch_overview_tracing>`, either in this message or via :option:`--service-node`.
    id: String,
  
    /// Defines the local service cluster name where Envoy is running. Though optional, it should be set if any of the following features are used: :ref:`statsd <arch_overview_statistics>`, [health check cluster verification][crate::config::core::health_check::HTTPHealthCheck::service_name_matcher], [runtime override directory][crate::config::bootstrap::Runtime], [user agent addition][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.add_user_agent], :ref:`HTTP global rate limiting <config_http_filters_rate_limit>`, :ref:`CDS <config_cluster_manager_cds>`, and :ref:`HTTP tracing <arch_overview_tracing>`, either in this message or via :option:`--service-cluster`.
    cluster: String,
  
    /// Opaque metadata extending the node identifier. Envoy will pass this directly to the management server.
    metadata: Struct,
  
    /**
    Map from xDS resource type URL to dynamic context parameters. These may vary at runtime (unlike other fields in this message). For example, the xDS client may have a shard identifier that changes during the lifetime of the xDS client. In Envoy, this would be achieved by updating the dynamic context on the Server::Instance's LocalInfo context provider. The shard ID dynamic parameter then appears in this field during future discovery requests.
    */
    dynamic_parameters: HashMap<String, ContextParams>,
  
    /// Locality specifying where the Envoy instance is running.
    locality: Locality,
  
    /// Free-form string that identifies the entity requesting config. E.g. "envoy" or "grpc"
    user_agent_name: String,
  
    user_agent_version_type: UserAgentVersionType,
  
    /// List of extensions and their versions supported by the node.
    extensions: Vec<Extension>,
  
    /**
    Client feature support list. These are well known features described in the Envoy API repository for a given major version of an API. Client features use reverse DNS naming scheme, for example `com.acme.feature`.
    See :ref:`the list of features <client_features>` that xDS client may support.
    */
    client_features: Vec<String>,
}

pub enum UserAgentVersionType {
    /// Free-form string that identifies the version of the entity requesting config. E.g. `"1.12.2"`, `"abcd1234"`, or `"SpecialEnvoyBuild"`.
    UserAgentVersion(String),

    /// Structured version of the entity requesting config.
    UserAgentBuildVersion(BuildVersion)
}

/**
Metadata provides additional inputs to filters based on matched listeners, filter chains, routes and endpoints. It is structured as a map, usually from filter name (in reverse DNS format) to metadata specific to the filter. Metadata key-values for a filter are merged as connection and request handling occurs, with later values for the same key overriding earlier values.

An example use of metadata is providing additional values to http_connection_manager in the envoy.http_connection_manager.access_log namespace.

Another example use of metadata is to per service config info in cluster metadata, which may get consumed by multiple filters.

For load balancing, Metadata provides a means to subset cluster endpoints.
Endpoints have a Metadata object associated and routes contain a Metadata object to match against. There are some well defined metadata used today for this purpose:

- `{"envoy.lb": {"canary": <bool> }}` This indicates the canary status of an endpoint and is also used during header processing (x-envoy-upstream-canary) and for stats purposes.
[#next-major-version: move to type/metadata/v2]
*/
pub struct Metadata {
    /**
    Key is the reverse DNS filter name, e.g. com.acme.widget. The `envoy.*` namespace is reserved for Envoy's built-in filters.
    If both `filter_metadata` and [`typed_filter_metadata`][Self::typed_filter_metadata] fields are present in the metadata with same keys, only `typed_filter_metadata` field will be parsed.
    */
    filter_metadata: HashMap<String, Struct>,
  
    /**
    Key is the reverse DNS filter name, e.g. com.acme.widget. The `envoy.*` namespace is reserved for Envoy's built-in filters.
    The value is encoded as google.protobuf.Any.
    If both [`filter_metadata`][Self::filter_metadata] and `typed_filter_metadata` fields are present in the metadata with same keys, only `typed_filter_metadata` field will be parsed.
    */
    typed_filter_metadata: HashMap<String, Any>
}


/// BuildVersion combines SemVer version of extension with free-form build information (i.e. 'alpha', 'private-build') as a set of strings.
pub struct BuildVersion {
    /// SemVer version of extension.
    version: SemanticVersion,
  
    /**
    Free-form build information.
    Envoy defines several well known keys in the source/common/version/version.h file
    */
    metadata: Struct,
}
  
/// Version and identification for an Envoy extension.
pub struct Extension {
    /// This is the name of the Envoy filter as specified in the Envoy configuration, e.g. envoy.filters.http.router, com.acme.widget.
    name: String,
  
    /**
    Category of the extension.
    Extension category names use reverse DNS notation. For instance "envoy.filters.listener" for Envoy's built-in listener filters or "com.acme.filters.http" for HTTP filters from acme.com vendor.
    [#comment:TODO(yanavlasov): Link to the doc with existing envoy category names.]
    */
    category: String,
  
    /**
    The version is a property of the extension and maintained independently of other extensions and the Envoy API.
    This field is not set when extension did not provide version information.
    */
    version: BuildVersion,
  
    /// Indicates that the extension is present but was disabled via dynamic configuration.
    disabled: bool,
  
    /// Type URLs of extension configuration protos.
    type_urls: Vec<String>,
}

/// Runtime derived u32 with a default when not specified.
pub struct RuntimeU32 {
    /// Default value if runtime value is not available.
    default_value: u32,
  
    /// Runtime key to get value for comparison. This value is used if defined.
    // [!runtime_key.is_empty()]
    runtime_key: String
}
  
/// Runtime derived percentage with a default when not specified.
pub struct RuntimePercent {
    /// Default value if runtime value is not available.
    default_value: Percent,
  
    /// Runtime key to get value for comparison. This value is used if defined.
    // [!runtime_key.is_empty()]
    runtime_key: String
}
  
/// Runtime derived double with a default when not specified.
pub struct RuntimeF64 {
    /// Default value if runtime value is not available.
    default_value: f64,
  
    /// Runtime key to get value for comparison. This value is used if defined.
    // [!runtime_key.is_empty()]
    runtime_key: String
}
  
/// Runtime derived bool with a default when not specified.
pub struct RuntimeFeatureFlag {
    /// Default value if runtime value is not available.
    // [(validate.rules).message = {required: true}];
    default_value: bool,
  
    /**
    Runtime key to get value for comparison. This value is used if defined. The boolean value must be represented via its [canonical JSON encoding](https://developers.google.com/protocol-buffers/docs/proto3#json).
    */
    // [!runtime_key.is_empty()]
    runtime_key: String
}
  
/// Query parameter name/value pair.
pub struct QueryParameter {
    /// The key of the query parameter. Case sensitive.
    // [!runtime_key.is_empty()]
    runtime_key: String,
  
    /// The value of the query parameter.
    value: String
}
  
/// Header name/value pair.
pub struct HeaderValue {
    /**
    Header name.

    [(validate.rules).string =
        {min_len: 1 max_bytes: 16384 well_known_regex: HTTP_HEADER_NAME strict: false}];
    */
    key: String,
        
  
    /**
    Header value.

    The same :ref:`format specifier <config_access_log_format>` as used for
    :ref:`HTTP access logging <config_access_log>` applies here, however
    unknown header values are replaced with the empty string instead of `-`.

    [
      (validate.rules).string = {max_bytes: 16384 well_known_regex: HTTP_HEADER_VALUE strict: false}
    ];
    */
    value: String
}
  
/// Header name/value pair plus option to control append behaviour.
pub struct HeaderValueOption {  
    /// Header name/value pair that this option applies to.
    // [(validate.rules).message = {required: true}];
    header: HeaderValue,
  
    /**
    Describes the action taken to append/overwrite the given value for an existing header or to only add this header if it's absent.
    Value defaults to [`AppendIfExistsOrAdd`][HeaderAppendAction::AppendIfExistsOrAdd].

    [(validate.rules).enum = {defined_only: true}];
    */
    append_action: HeaderAppendAction,
  
    /**
    Is the header value allowed to be empty? If false (default), custom headers with empty values are dropped, otherwise they are added.
    */
    keep_empty_value: bool
}

/// Describes the supported actions types for header append action.
pub enum HeaderAppendAction {
    /// This action will append the specified value to the existing values if the header already exists. If the header doesn't exist then this will add the header with specified key and value.
    AppendIfExistsOrAdd,

    /// This action will add the header if it doesn't already exist. If the header already exists then this will be a no-op.
    AddIfAbsent,

    /// This action will overwrite the specified value by discarding any existing values if the header already exists. If the header doesn't exist then this will add the header with specified key and value.
    OverwriteIfExistsOrAdd
}
  
/// Wrapper for a set of headers.
pub struct HeaderMap {
    headers: Vec<HeaderValue>
}
  
/// A directory that is watched for changes, e.g. by inotify on Linux. Move/rename events inside this directory trigger the watch.
pub struct WatchedDirectory {
    /// Directory path to watch.
    // [!path.is_empty()]
    path: String
}
  
/// Data source consisting of a file, an inline value, or an environment variable.
pub enum DataSource {
    // option (validate.required) = true;

    /// Local filesystem data source.
    // [!is_empty()]
    Filename(String),

    /// Bytes inlined in the configuration.
    InlineBytes(Vec<u8>),

    /// String inlined in the configuration.
    InlineString(String),

    /// Environment variable data source.
    // [!is_empty()]
    EnvironmentVariable(String)
}

/// The message specifies the retry policy of remote data source when fetching fails.
pub struct RetryPolicy {
    /**
    Specifies parameters that control [retry backoff strategy][crate::config::core::backoff::BackoffStrategy].
    This parameter is optional, in which case the default base interval is 1000 milliseconds. The default maximum interval is 10 times the base interval.
    */
    retry_back_off: BackoffStrategy,
  
    /**
    Specifies the allowed number of retries. This parameter is optional and defaults to `1`.
    */
    // [(udpa.annotations.field_migrate).rename = "max_retries"];
    num_retries: u32
        
}
  
/// The message specifies how to fetch data from remote and how to verify it.
pub struct RemoteDataSource {
    /// The HTTP URI to fetch the remote data.
    // [(validate.rules).message = {required: true}];
    http_uri: HTTPURI,
  
    /// SHA256 string for verifying data.
    // [!is_empty()]
    sha256: String,
  
    /// Retry policy for fetching remote data.
    retry_policy: RetryPolicy
}
  
/// Async data source which support async data fetch.
pub enum AsyncDataSource {
    // option (validate.required) = true;
  
    /// Local async data source.
    Local(DataSource),

    /// Remote async data source.
    Remote(RemoteDataSource),
}
  
/// Configuration for transport socket in :ref:`listeners <config_listeners>` and [clusters][crate::config::cluster::cluster::Cluster]. If the configuration is empty, a default transport socket implementation and configuration will be chosen based on the platform and existence of tls_context.
pub struct TransportSocket {
    /// The name of the transport socket to instantiate. The name must match a supported transport socket implementation.
    // [(validate.rules).string = {min_len: 1}];
    name: String,
  
    /// Implementation specific configuration which depends on the implementation being instantiated.
    /// See the supported transport socket implementations for further documentation.
    config_type: ConfigType
}

pub enum ConfigType {
    TypedConfig(Any)
}
  
/**
Runtime derived FractionalPercent with defaults for when the numerator or denominator is not specified via a runtime key.

> NOTE: Parsing of the runtime key's data is implemented such that it may be represented as a [`FractionalPercent`] proto represented as JSON/YAML and may also be represented as an integer with the assumption that the value is an integral percentage out of 100. For instance, a runtime key lookup returning the value "42" would parse as a `FractionalPercent` whose numerator is 42 and denominator is HUNDRED.
*/
pub struct RuntimeFractionalPercent {
    /// Default value if the runtime value's for the numerator/denominator keys are not available.
    // [(validate.rules).message = {required: true}];
    default_value: FractionalPercent,
  
    /// Runtime key for a YAML representation of a FractionalPercent.
    runtime_key: String
}
  
/// Identifies a specific ControlPlane instance that Envoy is connected to.
pub struct ControlPlane {
    /// An opaque control plane identifier that uniquely identifies an instance of control plane. This can be used to identify which control plane instance, the Envoy is connected to.
    identifier: String
 }
  
