/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/tracing/v3/custom_tag.proto>
*/

use crate::types::metadata::{MetadataKey, MetadataKind};


/// Describes custom tags for the active span.
pub struct CustomTag {
    /// Used to populate the tag name.
    // [(validate.rules).String = {min_len: 1}];
    tag: String,

    /// Used to specify what kind of custom tag.
    r#type: Type
}

pub enum Type {
    /// option (validate.required) = true;

    /// A literal custom tag.
    Literal(Literal),

    /// An environment custom tag.
    Environment(Environment),

    /// A request header custom tag.
    RequestHeader(Header),

    /// A custom tag to obtain tag value from the metadata.
    Metadata(Metadata),
}

/// Literal type custom tag with static value for the tag value.
pub struct Literal {
    /// Static literal value to populate the tag value.
    // [(validate.rules).String = {min_len: 1}];
    value: String
}

/// Environment type custom tag with environment name and default value.
pub struct Environment {
    /// Environment variable name to obtain the value to populate the tag value.
    // [(validate.rules).String = {min_len: 1}];
    name: String,

    /**
    When the environment variable is not found, the tag value will be populated with this default value if specified, otherwise no tag will be populated.
    */
    default_value: String,
}

/// Header type custom tag with header name and default value.
pub struct Header {
    /// Header name to obtain the value to populate the tag value.
    // [(validate.rules).String = {min_len: 1 well_known_regex: HTTP_HEADER_NAME strict: false}];
    name: String,
        

    /*
    When the header does not exist, the tag value will be populated with this default value if specified, otherwise no tag will be populated.
    */
    default_value: String,
}

/**
Metadata type custom tag using [`MetadataKey`][crate::types::metadata::MetadataKey>` to retrieve the protobuf value
from [`Metadata`][crate::config::core::Metadata], and populate the tag value with
`the canonical JSON <https://developers.google.com/protocol-buffers/docs/proto3#json>`_
representation of it.
*/
pub struct Metadata {
    /// Specify what kind of metadata to obtain tag value from.
    kind: MetadataKind,

    /// Metadata key to define the path to retrieve the tag value.
    metadata_key: MetadataKey,

    /**
    When no valid metadata is found,
    the tag value would be populated with this default value if specified,
    otherwise no tag would be populated.
    */
    default_value: String
}
