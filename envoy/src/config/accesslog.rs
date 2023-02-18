/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/accesslog/v3/accesslog.proto>
*/

type Any = String;
type Struct = String;

use crate::{
    config::{
        core::base::RuntimeU32,
        route::route_components::HeaderMatcher
    },
    types::{
        matcher::metadata::MetadataMatcher,
        percent::FractionalPercent,
    }
};

pub struct AccessLog {
    /// The name of the access log extension configuration.
    name: String,

    /// Filter which is used to determine if the access log needs to be written.
    filter: AccessLogFilter,

    /// Custom configuration that must be set according to the access logger extension being instantiated.
    // [#extension-category: envoy.access_loggers]
    config_type: ConfigType
}

pub enum ConfigType {
    TypedConfig(Any)
}

pub struct AccessLogFilter {
    filter_specifier: FilterSpecifier
}

pub enum FilterSpecifier {
    // option (validate.required) = true;

    /// Status code filter.
    StatusCodeFilter(StatusCodeFilter),

    // Duration filter.
    DurationFilter(DurationFilter),

    /// Not health check filter.
    NotHealthCheckFilter(NotHealthCheckFilter),

    /// Traceable filter.
    TraceableFilter(TraceableFilter),

    /// Runtime filter.
    RuntimeFilter(RuntimeFilter),

    /// And filter.
    AndFilter(AndFilter),

    /// Or filter.
    OrFilter(OrFilter),

    /// Header filter.
    HeaderFilter(HeaderFilter),

    /// Response flag filter.
    ResponseFlagFilter(ResponseFlagFilter),

    /// gRPC status filter.
    GRPCStatusFilter(GRPCStatusFilter),

    /// Extension filter.
    // [#extension-category: envoy.access_loggers.extension_filters]
    ExtensionFilter(ExtensionFilter),

    /// Metadata Filter
    MetadataFilter(MetadataFilter),
}

/// Filter on an integer comparison.
pub struct ComparisonFilter {
    /// Comparison operator.
    // [(validate.rules).enum = {defined_only: true}];
    op: Op,

    /// Value to compare against.
    // [(validate.rules).message = {required: true}];
    value: RuntimeU32
}

pub enum Op {
    /// =
    EQ,

    /// >=
    GE,

    /// <=
    LE,
}

// Filters on HTTP response/status code.
pub struct StatusCodeFilter {
    /// Comparison.
    // [(validate.rules).message = {required: true}];
    comparison: ComparisonFilter
}

/// Filters on total request duration in milliseconds.
pub struct DurationFilter {
    /// Comparison.
    // [(validate.rules).message = {required: true}];
    comparison: ComparisonFilter
}

/// Filters for requests that are not health check requests. A health check request is marked by the health check filter.
pub struct NotHealthCheckFilter {
}

/// Filters for requests that are traceable. See the tracing overview for more information on how a request becomes traceable.
pub struct TraceableFilter {
}

/// Filters for random sampling of requests.
pub struct RuntimeFilter {
    /**
    Runtime key to get an optional overridden numerator for use in the `percent_sampled` field. If found in runtime, this value will replace the default numerator.

    [!runtime_key.is_empty()]
    */
    runtime_key: String,

    /// The default sampling percentage. If not specified, defaults to 0% with denominator of 100.
    percent_sampled: FractionalPercent,

    /**
    By default, sampling pivots on the header :ref:`x-request-id <config_http_conn_man_headers_x-request-id>` being present. If :ref:`x-request-id<config_http_conn_man_headers_x-request-id>` is present, the filter will consistently sample across multiple hosts based on the runtime key value and the value extracted from :ref:`x-request-id <config_http_conn_man_headers_x-request-id>`. If it is missing, or `use_independent_randomness` is set to true, the filter will randomly sample based on the runtime key value alone. `use_independent_randomness` can be used for logging kill switches within complex nested [`AndFilter`] and [`OrFilter`] blocks that are easier to reason about from a probability perspective (i.e., setting to true will cause the filter to behave like an independent random variable when composed within logical operator filters).
    */
    use_independent_randomness: bool,
}

/**
Performs a logical 'and' operation on the result of each filter in filters.
Filters are evaluated sequentially and if one of them returns `false`, the filter returns false immediately.
*/
pub struct AndFilter {
    /// [filters.len() >= 2]
    filters: Vec<AccessLogFilter>
}

/**
Performs a logical 'or' operation on the result of each individual filter.
Filters are evaluated sequentially and if one of them returns true, the filter returns true immediately.
*/
pub struct OrFilter {
    /// [filters.len() >= 2]
    filters: Vec<AccessLogFilter>
}

/// Filters requests based on the presence or value of a request header.
pub struct HeaderFilter {
    /// Only requests with a header which matches the specified HeaderMatcher will pass the filter check.
    // [(validate.rules).message = {required: true}];
    header: HeaderMatcher
}

/**
Filters requests that received responses with an Envoy response flag set.
A list of the response flags can be found in the access log formatter :ref:`documentation <config_access_log_format_response_flags>`.
*/
pub struct ResponseFlagFilter {
    /// Only responses with the any of the flags listed in this field will be logged. This field is optional. If it is not specified, then any response flag will pass the filter check.
    flags: Vec<Flag>,
}

pub enum Flag {
    LH,
    UH,
    UT,
    LR,
    UR,
    UF,
    UC,
    UO,
    NR,
    DI,
    FI,
    RL,
    UAEX,
    RLSE,
    DC,
    URX,
    SI,
    IH,
    DPE,
    UMSDR,
    RFCF,
    NFCF,
    DT,
    UPE,
    NC,
    OM,
}

/// Filters gRPC requests based on their response status. If a gRPC status is not provided, the filter will infer the status from the HTTP status code.
pub struct GRPCStatusFilter {
    /// Logs only responses that have any one of the gRPC statuses in this field.
    // [(validate.rules).repeated = {items {enum {defined_only: true}}}];
    statuses: Vec<Status>,

    /**
    If included and set to true, the filter will instead block all responses with a gRPC status or inferred gRPC status enumerated in statuses, and allow all other responses.
    */
    exclude: bool,
}

pub enum Status {
    Ok,
    Cancelled,
    Unknown,
    InvalidArgument,
    DeadlineExceeded,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    ResourceExhausted,
    FailedPrecondition,
    Aborted,
    OutOfRange,
    Unimplemented,
    Internal,
    Unavailable,
    DataLoss,
    Unauthenticated,
}

/*
Filters based on matching dynamic metadata.
If the matcher path and key correspond to an existing key in dynamic metadata, the request is logged only if the matcher value is equal to the metadata value. If the matcher path and key *do not* correspond to anexisting key in dynamic metadata, the request is logged only if match_if_key_not_found is "true" or unset.
*/
pub struct MetadataFilter {
    /// Matcher to check metadata for specified value. For example, to match on the `access_log_hint metadata`, set the filter to "envoy.common" and the path to `access_log_hint`, and the value to `true`.
    matcher: MetadataMatcher,

    /// Default result if the key does not exist in dynamic metadata: if unset or `true`, then log; if false, then don't log.
    match_if_key_not_found: bool,
}

/// Extension filter is statically registered at runtime.
pub struct ExtensionFilter {
    /// The name of the filter implementation to instantiate. The name must match a statically registered filter.
    name :String,

    /// Custom configuration that depends on the filter being instantiated.
    config_type: ConfigType
}

