/*!
<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/logs/v1/logs.proto>
*/

use crate::{
    common::{AnyValue, InstrumentationScope, KeyValue},
    resource::Resource
};

/**
`LogsData` represents the logs data that can be stored in a persistent storage,
OR can be embedded by other protocols that transfer OTLP logs data but do not implement the OTLP protocol.

The main difference between this message and collector protocol is that in this message there will not be any 'control' or 'metadata' specific to
OTLP protocol.

When new fields are added into this message, the OTLP request MUST be updated as well.
*/
pub struct LogsData {
    /**
    An array of ResourceLogs.
    For data coming from a single resource this array will typically contain one element. Intermediary nodes that receive data from multiple origins typically batch the data before forwarding further and in that case this array will contain multiple elements.
    */
    resource_logs: Vec<ResourceLogs>,
}
  
/// A collection of ScopeLogs from a Resource.
pub struct ResourceLogs {
    /**
    The resource for the logs in this message.
    If this field is not set then resource info is unknown.
    */
    resource: Option<Resource>,
  
    /// A list of `ScopeLogs` that originate from a resource.
    scope_logs: Vec<ScopeLogs>,
  
    /// This `schema_url` applies to the data in the `resource` field. It does not apply to the data in the `scope_logs` field which have their own `schema_url` field.
    schema_url: String,
  }
  
/// A collection of Logs produced by a Scope.
pub struct ScopeLogs {
    /**
    The instrumentation scope information for the logs in this message.
    Semantically when `InstrumentationScope` isn't set, it is equivalent with an empty instrumentation scope name (unknown).
    */
    scope: InstrumentationScope,
  
    /// A list of log records.
    log_records: Vec<LogRecord>,
  
    /// This `schema_url` applies to all logs in the "logs" field.
    schema_url: String,
}
  
/// Possible values for LogRecord.SeverityNumber.
pub enum SeverityNumber {
    /// UNSPECIFIED is the default SeverityNumber, it MUST NOT be used.
    Unspecified,
    Trace ,
    Trace2,
    Trace3,
    Trace4,
    Debug ,
    Debug2,
    Debug3,
    Debug4,
    Info  ,
    Info2 ,
    Info3 ,
    Info4 ,
    Warn  ,
    Warn2 ,
    Warn3 ,
    Warn4 ,
    Error ,
    Error2,
    Error3,
    Error4,
    Fatal ,
    Fatal2,
    Fatal3,
    Fatal4,
}

/// Masks for LogRecord.flags field.
pub enum LogRecordFlags {
    Unspecified,
    TraceFlagsMask  // = 0x000000FF;
}
  
/// A log record according to OpenTelemetry Log Data Model: <https://github.com/open-telemetry/oteps/blob/main/text/logs/0097-log-data-model.md>
pub struct LogRecord {
    /**
    `time_unix_nano` is the time when the event occurred.
    Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    Value of `0` indicates unknown or missing timestamp.
    */
    time_unix_nano: u64,
  
    /**
    Time when the event was observed by the collection system.
    For events that originate in OpenTelemetry (e.g. using OpenTelemetry Logging SDK) this timestamp is typically set at the generation time and is equal to Timestamp.
    For events originating externally and collected by OpenTelemetry (e.g. using Collector) this is the time when OpenTelemetry's code observed the event measured by the clock of the OpenTelemetry code. This field MUST be set once the event is observed by OpenTelemetry.

    For converting OpenTelemetry log data to formats that support only one timestamp or when receiving OpenTelemetry log data by recipients that support only one timestamp internally the following logic is recommended:
      - Use `time_unix_nano` if it is present, otherwise use `observed_time_unix_nano`.

    Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    Value of 0 indicates unknown or missing timestamp.
    */
    observed_time_unix_nano: u64,
  
    severity_number: Option<SeverityNumber>,
  
    /// The severity text (also known as log level). The original string representation as it is known at the source. [Optional.
    severity_text: Option<String>,
  
    /// A value containing the body of the log record. Can be for example a human-readable string message (including multi-line) describing the event in a free form or it can be a structured data composed of arrays and maps of other values. Optional.
    body: Option<AnyValue>,
  
    /**
    Additional attributes that describe the specific event occurrence. Optional.
    Attribute keys MUST be unique (it is not allowed to have more than one attribute with the same key).
    */
    attributes: Option<Vec<KeyValue>>,
    dropped_attributes_count: u32,
  
    /**
    Flags, a bit field. 8 least significant bits are the trace flags as defined in W3C Trace Context specification. 24 most significant bits are reserved and must be set to `0`. Readers must not assume that 24 most significant bits will be zero and must correctly mask the bits when reading 8-bit trace flag (use flags & TRACE_FLAGS_MASK). Optional.
    */
    flags: Option<u32>,
  
    /**
    A unique identifier for a trace. All logs from the same trace share the same `trace_id`. The ID is a 16-byte array. An ID with all zeroes is considered invalid. Can be set for logs that are part of request processing and have an assigned trace id. Optional.
    */
    trace_id: Option<Vec<u8>>,
  
    /**
    A unique identifier for a span within a trace, assigned when the span is created. The ID is an 8-byte array. An ID with all zeroes is considered invalid. Can be set for logs that are part of a particular processing span.
    If `span_id` is present trace_id SHOULD be also present. Optional.
    */
    span_id: Option<Vec<u8>>,
}
