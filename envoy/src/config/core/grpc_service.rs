/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/grpc_service.proto>
*/

type Struct = String;
type Empty = String;

use std::{
    collections::HashMap,
    time::Duration,
};

use crate::config::{core::base::{DataSource, HeaderValue}, accesslog::ConfigType};

/**
gRPC service configuration. This is used by [`APIConfigSource`][crate::config::core::config_source::APIConfigSource] and filter configurations.
*/
pub struct GRPCService {
    target_specifier: TargetSpecifier,

    /// The timeout for the gRPC request. This is the timeout for a specific request.
    timeout: Duration,

    /**
    Additional metadata to include in streams initiated to the GrpcService. This can be used for scenarios in which additional ad hoc authorization headers (e.g. `x-foo-bar: baz-key`) are to be injected. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    */
    initial_metadata: Vec<HeaderValue>
}

pub enum TargetSpecifier {
    // option (validate.required) = true;

    /**
    Envoy's in-built gRPC client.
    See the :ref:`gRPC services overview <arch_overview_grpc_services>` documentation for discussion on gRPC client selection.
    */
    EnvoyGRPC(EnvoyGRPC),

    /**
    `Google C++ gRPC client <https://github.com/grpc/grpc>`_
    See the :ref:`gRPC services overview <arch_overview_grpc_services>` documentation for discussion on gRPC client selection.
    */
    GoogleGRPC(GoogleGRPC)
}

pub struct EnvoyGRPC {
    /**
    The name of the upstream gRPC cluster. SSL credentials will be supplied
    in the [`Cluster`][crate::config::cluster::cluster::Cluster] [`transport_socket`][crate::config::cluster::cluster::Cluster.transport_socket].

    [(validate.rules).String = {min_len: 1}];
    */
    cluster_name: String,

    /**
    The `:authority` header in the grpc request. If this field is not set, the authority header value will be `cluster_name`.
    Note that this authority does not override the SNI. The SNI is provided by the transport socket of the cluster.

    [(validate.rules).String =
        {min_len: 0 max_bytes: 16384 well_known_regex: HTTP_HEADER_VALUE strict: false}];
    */
    authority: String
        
}

pub struct GoogleGRPC {
    /**
    The target URI when using the [Google C++ gRPC client](https://github.com/grpc/grpc). SSL credentials will be supplied in [`channel_credentials`][Self::channel_credentials].

    [(validate.rules).String = {min_len: 1}];
    */
    target_uri: String,

    channel_credentials: ChannelCredentials,

    /**
    A set of call credentials that can be composed with [channel credentials](https://grpc.io/docs/guides/auth.html#credential-types>).
    */
    call_credentials: Vec<CallCredentials>,

    /**
    The human readable prefix to use when emitting statistics for the gRPC
    service.

    .. csv-table::
       :header: Name, Type, Description
       :widths: 1, 1, 2

       streams_total, Counter, Total number of streams opened
       streams_closed_\<gRPC status code\>, Counter, Total streams closed with \<gRPC status code\>

    [(validate.rules).String = {min_len: 1}];
    */
    stat_prefix: String,

    /**
    The name of the Google gRPC credentials factory to use. This must have been registered with Envoy. If this is empty, a default credentials factory will be used that sets up channel credentials based on other configuration parameters.
    */
    credentials_factory_name: String,

    /**
    Additional configuration for site-specific customizations of the Google gRPC library.
    */
    config: Struct,

    /**
    How many bytes each stream can buffer internally.
    If not set an implementation defined default is applied (1MiB).
    */
    per_stream_buffer_limit_bytes: u32,

    /// Custom channels args.
    channel_args: ChannelArgs
}

/// See <https://grpc.io/grpc/cpp/structgrpc_1_1_ssl_credentials_options.html>.
pub struct SSLCredentials {
    /// PEM encoded server root certificates.
    root_certs: DataSource,

    /// PEM encoded client private key.
    /// [(udpa.annotations.sensitive) = true];
    private_key: DataSource,

    /// PEM encoded client certificate chain.
    cert_chain: DataSource,
}

/**
Local channel credentials. Only UDS is supported for now.
See <https://github.com/grpc/grpc/pull/15909>.
*/
pub struct GoogleLocalCredentials {
}

/**
See <https://grpc.io/docs/guides/auth.html#credential-types> to understand Channel and Call credential types.
*/
pub enum ChannelCredentials {
    // option (validate.required) = true;

    SSLCredentials(SSLCredentials),

    /// <https://grpc.io/grpc/cpp/namespacegrpc.html#a6beb3ac70ff94bd2ebbd89b8f21d1f61>
    GoogleDefault(Empty),

    LocalCredentials(GoogleLocalCredentials),
}

// [#next-free-field: 8]
pub enum CallCredentials {
    // option (validate.required) = true;

    /*
    Access token credentials.
    <https://grpc.io/grpc/cpp/namespacegrpc.html#ad3a80da696ffdaea943f0f858d7a360d>.
    */
    AccessToken(String),

    /**
    Google Compute Engine credentials.
    <https://grpc.io/grpc/cpp/namespacegrpc.html#a6beb3ac70ff94bd2ebbd89b8f21d1f61>.
    */
    GoogleComputeEngine(Empty),

    /**
    Google refresh token credentials.
    <https://grpc.io/grpc/cpp/namespacegrpc.html#a96901c997b91bc6513b08491e0dca37c>.
    */
    GoogleRefreshToken(String),

    /**
    Service Account JWT Access credentials.
    <https://grpc.io/grpc/cpp/namespacegrpc.html#a92a9f959d6102461f66ee973d8e9d3aa>.
    */
    ServiceAccountJWTAccess(ServiceAccountJWTAccessCredentials),

    /**
    Google IAM credentials.
    <https://grpc.io/grpc/cpp/namespacegrpc.html#a9fc1fc101b41e680d47028166e76f9d0>.
    */
    GoogleIAM(GoogleIAMCredentials),

    /**
    Custom authenticator credentials.
    <https://grpc.io/grpc/cpp/namespacegrpc.html#a823c6a4b19ffc71fb33e90154ee2ad07>.
    <https://grpc.io/docs/guides/auth.html#extending-grpc-to-support-other-authentication-mechanisms>.
    */
    FromPlugin(MetadataCredentialsFromPlugin),

    /**
    Custom security token service which implements OAuth 2.0 token exchange.
    <https://tools.ietf.org/html/draft-ietf-oauth-token-exchange-16>
    See <https://github.com/grpc/grpc/pull/19587>.
    */
    STSService(STSService),
}

/**
Security token service configuration that allows Google gRPC to fetch security token from an OAuth 2.0 authorization server.
See <https://tools.ietf.org/html/draft-ietf-oauth-token-exchange-16> and <https://github.com/grpc/grpc/pull/19587>.
*/
pub struct STSService {
    /**
    URI of the token exchange service that handles token exchange requests.
    [#comment:TODO(asraa): Add URI validation when implemented. Tracked by
    <https://github.com/bufbuild/protoc-gen-validate/issues/303>]
    */
    token_exchange_service_uri: String,

    /// Location of the target service or resource where the client intends to use the requested security token.
    resource: String,

    /// Logical name of the target service where the client intends to use the requested security token.
    audience: String,

    /// The desired scope of the requested security token in the context of the service or resource where the token will be used.
    scope: String,

    /// Type of the requested security token.
    requested_token_type: String,

    /// The path of subject token, a security token that represents the identity of the party on behalf of whom the request is being made.
    // [(validate.rules).String = {min_len: 1}];
    subject_token_path: String,

    /// Type of the subject token.
    // [(validate.rules).String = {min_len: 1}];
    subject_token_type: String,

    // The path of actor token, a security token that represents the identity of the acting party. The acting party is authorised to use the requested security token and act on behalf of the subject.
    actor_token_path: String,

    /// Type of the actor token.
    actor_token_type: String,
}

pub struct ServiceAccountJWTAccessCredentials {
    json_key: String,

    token_lifetime_seconds: u64,
}

pub struct GoogleIAMCredentials {
    authorisation_token: String,

    authority_selector: String,
}

pub struct MetadataCredentialsFromPlugin {
    name: String,

    // [#extension-category: envoy.grpc_credentials]
    config_type: ConfigType
}

/// Channel arguments.
pub struct ChannelArgs {
    /// See grpc_types.h GRPC_ARG #defines for keys that work here.
    args: HashMap<String, Value>,
}

/**
Pointer values are not supported, since they don't make any sense when delivered via the API.
*/
pub enum Value {
    // option (validate.required) = true;

    StringValue(String),

    IntValue(i64),
}
