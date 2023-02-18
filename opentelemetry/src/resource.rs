/*!
<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/resource/v1/resource.proto>
*/

use crate::common::KeyValue;

/// Resource information.
pub struct Resource {
    /**
    Set of attributes that describe the resource.
    Attribute keys MUST be unique (it is not allowed to have more than one attribute with the same key).
    */
    attributes: Vec<KeyValue>,

    /// `dropped_attributes_count` is the number of dropped attributes. If the value is `0`, then no attributes were dropped.
    dropped_attributes_count: u32,
}
