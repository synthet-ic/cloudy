/*!
<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/trace/v1/trace.proto>
*/

use std::time::SystemTime;

use w3c::trace_context::TraceContext;

use crate::{
    common::{InstrumentationScope, KeyValue},
    resource::Resource
};

/**
`TracesData` represents the traces data that can be stored in a persistent storage, OR can be embedded by other protocols that transfer OTLP traces data but do not implement the OTLP protocol.

The main difference between this message and collector protocol is that in this message there will not be any 'control' or 'metadata' specific to OTLP protocol.

When new fields are added into this message, the OTLP request MUST be updated as well.
*/
pub struct TracesData {
    /**
    An array of `ResourceSpans`.
    For data coming from a single resource this array will typically contain one element. Intermediary nodes that receive data from multiple origins typically batch the data before forwarding further and in that case this array will contain multiple elements.
    */
    resource_spans: Vec<ResourceSpans>,
}
  
/// A collection of ScopeSpans from a Resource.
pub struct ResourceSpans {
    /**
    The resource for the spans in this message.
    If this field is not set then no resource info is known.
    */
    resource: Resource,
  
    /// A list of `ScopeSpans` that originate from a resource.
    scope_spans: Vec<ScopeSpans>,
  
    /**
    This schema_url applies to the data in the `resource` field. It does not apply to the data in the `scope_spans` field which have their own `schema_url` field.
    */
    schema_url: String,
}
  
/// A collection of Spans produced by an InstrumentationScope.
pub struct ScopeSpans {
    /**
    The instrumentation scope information for the spans in this message. Semantically when `InstrumentationScope` isn't set, it is equivalent with an empty instrumentation scope name (unknown).
    */
    scope: InstrumentationScope,
  
    /// A list of `Spans` that originate from an instrumentation scope.
    spans: Vec<Span>,
  
    /// This `schema_url` applies to all spans and span events in the `spans` field.
    schema_url: String,
}
  
/**
<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/api.md#span>

A `Span` represents a single operation performed by a single component of the system.
*/
pub struct Span {
    /// The `span_id` of this span's parent span. If this is a root span, then this field must be empty. The ID is an 8-byte array.
    parent_span_id: Option<Vec<u8>>,
  
    /**
    A description of the span's operation.

    For example, the name can be a qualified method name or a file name and a line number where the operation is called. A best practice is to use the same display name at the same call point in an application.
    This makes it easier to correlate spans in different traces.

    This field is semantically required to be set to non-empty string.
    Empty value is equivalent to an unknown span name.

    This field is required.
    */
    name: String,
  
    /// Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `Client` (caller) and `Server` (callee) to identify queueing latency associated with the span.
    kind: Option<SpanKind>,
  
    /**
    `start_time` is the start time of the span. On the client side, this is the time kept by the local machine where the span execution starts. On the server side, this is the time when the server's application handler starts running.
    Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.

    This field is semantically required and it is expected that `end_time >= start_time`.
    */
    start_time: SystemTime,
  
    /**
    `end_time` is the end time of the span. On the client side, this is the time kept by the local machine where the span execution ends. On the server side, this is the time when the server application handler stops running.
    Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.

    This field is semantically required and it is expected that `end_time >= start_time`.
    */
    end_time: SystemTime,
  
    /**
    `attributes` is a collection of key/value pairs. Note, global attributes like server name can be set using the resource API. Examples of attributes:

    ```
    "/http/user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36"
    "/http/server_latency": 300
    "abc.com/myattribute": true
    "abc.com/score": 10.239
    ```

    The OpenTelemetry API specification further restricts the allowed value types: <https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/common/README.md#attribute>
    Attribute keys MUST be unique (it is not allowed to have more than one attribute with the same key).
    */
    attributes: Vec<KeyValue>,
  
    /**
    `dropped_attributes_count` is the number of attributes that were discarded. Attributes can be discarded because their keys are too long or because there are too many attributes. If this value is `0`, then no attributes were dropped.
    */
    dropped_attributes_count: u32,
  
    /// `events` is a collection of Event items.
    events: Vec<Event>,
  
    /// `dropped_events_count` is the number of dropped events. If the value is `0`, then no events were dropped.
    dropped_events_count: u32,
  
    /// `links` is a collection of `Link`s, which are references from this span to a span in the same or different trace.
    links: Vec<Link>,
  
    /// `dropped_links_count` is the number of dropped links after the maximum size was enforced. If this value is `0`, then no links were dropped.
    dropped_links_count: u32,
  
    /// An optional final status for this span.
    status: Option<Status>,
}

/**
<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/api.md#spankind>

`SpanKind` is the type of span. Can be used to specify additional relationships between spans in addition to a parent/child relationship.
*/
#[derive(Default)]
pub enum SpanKind {
    /**
    Indicates that the span represents an internal operation within an application, as opposed to an operation happening at the boundaries. Default value.
    */
    #[default]
    Internal,

    /// Indicates that the span covers server-side handling of an RPC or other remote network request.
    Server,

    /// Indicates that the span describes a request to some remote service.
    Client,

    /**
    Indicates that the span describes a producer sending a message to a broker.
    Unlike `Client` and `Server`, there is often no direct critical path latency relationship between producer and consumer spans. A `Producer` span ends when the message was accepted by the broker while the logical processing of the message might span a much longer time.
    */
    Producer,

    /**
    Indicates that the span describes consumer receiving a message from a broker.
    Like the `Producer` kind, there is often no direct critical path latency relationship between producer and consumer spans.
    */
    Consumer,
}

/// Event is a time-stamped annotation of the span, consisting of user-supplied text description and key-value pairs.
pub struct Event {
    /// `time` is the time the event occurred.
    time: SystemTime,

    /**
    `name `of the event.
    This field is semantically required to be set to non-empty string.
    */
    name: String,

    /**
    `attributes` is a collection of attribute key/value pairs on the event.
    Attribute keys MUST be unique (it is not allowed to have more than one attribute with the same key).
    */
    attributes: Vec<KeyValue>,

    /// `dropped_attributes_count` is the number of dropped attributes. If the value is `0`, then no attributes were dropped.
    dropped_attributes_count: u32,
}

/**
A pointer from the current span to another span in the same trace or in a different trace. For example, this can be used in batching operations, where a single batch handler processes multiple requests from different traces or when the handler receives a request from a different project.
*/
pub struct Link {
    /// A unique identifier of a trace that this linked span is part of. The ID is a 16-byte array.
    trace_id: Vec<u8>,

    /// A unique identifier for the linked span. The ID is an 8-byte array.
    span_id: Vec<u8>,

    /// The `trace_state` associated with the link.
    trace_state: String,

    /**
    `attributes` is a collection of attribute key/value pairs on the link.
    Attribute keys MUST be unique (it is not allowed to have more than one attribute with the same key).
    */
    attributes: Vec<KeyValue>,

    /// `dropped_attributes_count` is the number of dropped attributes. If the value is `0`, then no attributes were dropped.
    dropped_attributes_count: u32,
}

/**
<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/api.md#set-status>.

The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs.
*/
pub enum Status {
    /// The `Span` has been validated by an Application developer or Operator to have completed successfully.
    Ok,
    /// The `Span` contains an error.
    Error(String),
}
