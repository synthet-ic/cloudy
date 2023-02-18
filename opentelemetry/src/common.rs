/*!
<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/common/v1/common.proto>
*/

/// `AnyValue` is used to represent any type of attribute value. `AnyValue` may contain a primitive value such as a string or integer or it may contain an arbitrary nested object containing arrays, key-value lists and primitives.
pub struct AnyValue {
    /// The value is one of the listed fields. It is valid for all values to be unspecified in which case this AnyValue is considered to be 'empty'.
    value: Value
}

pub enum Value {
    StringValue(String),
    BoolValue(bool),
    IntValue(i64),
    F64Value(f64),
    ArrayValue(ArrayValue),
    KVListValue(KeyValueList),
    BytesValue(Vec<u8>),
}

/// `ArrayValue` is a list of AnyValue messages. We need `ArrayValue` as a message since oneof in AnyValue does not allow repeated fields.
pub struct ArrayValue {
    /// Array of values. The array may be empty (contain 0 elements).
    values: Vec<AnyValue>,
}

/**
`KeyValueList` is a list of `KeyValue` messages. We need `KeyValueList` as a message since `oneof` in AnyValue does not allow repeated fields. Everywhere else where we need a list of `KeyValue` messages (e.g. in Span) we use `repeated KeyValue` directly to avoid unnecessary extra wrapping (which slows down the protocol). The 2 approaches are semantically equivalent.
*/
pub struct KeyValueList {
    /**
    A collection of key/value pairs of key-value pairs. The list may be empty (may contain 0 elements).
    The keys MUST be unique (it is not allowed to have more than one value with the same key).
    */
    values: Vec<KeyValue>,
}

/// `KeyValue` is a key-value pair that is used to store Span attributes, Link attributes, etc.
pub struct KeyValue {
    key: String,
    value: AnyValue,
}

/**
<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/glossary.md#instrumentation-scope>

`InstrumentationScope` is a message representing the instrumentation scope information such as the fully qualified name and version.
*/
pub struct InstrumentationScope {
    /// An empty instrumentation scope name means the name is unknown.
    name: String,
    version: Option<String>,
    attributes: Vec<KeyValue>,
    dropped_attributes_count: u32,
}
