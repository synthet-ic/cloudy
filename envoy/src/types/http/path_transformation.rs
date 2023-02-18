/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/http/v3/path_transformation.proto>
*/

/**
PathTransformation defines an API to apply a sequence of operations that can be used to alter text before it is used for matching or routing. Multiple actions can be applied in the same Transformation, forming a sequential pipeline. The transformations will be performed in the order that they appear.

This API is a work in progress.
*/
pub struct PathTransformation {
  /// A list of operations to apply. Transformations will be performed in the order that they appear.
  operations: Vec<Operation>,
}

/// A type of operation to alter text.
pub enum Operation {
  // option (validate.required) = true;

  /// Enable path normalization per RFC 3986.
  NormalisePathRFC3986(NormalisePathRFC3986),

  /// Enable merging adjacent slashes.
  MergeSlashes(MergeSlashes)
}

/**
Should text be normalized according to RFC 3986? This typically is used for path headers before any processing of requests by HTTP filters or routing. This applies percent-encoded normalisation and path segment normalisation. Fails on characters disallowed in URLs (e.g. NULLs). See [Normalisation and Comparison](https://www.rfc-editor.org/rfc/rfc3986#section-6) for details of normalisation. Note that this options does not perform [case normalisation](https://www.rfc-editor.org/rfc/rfc3986#section-6.2.2.1).
*/
pub struct NormalisePathRFC3986 {
}

/**
Determines if adjacent slashes are merged into one. A common use case is for a request path header. Using this option in [`PathNormalisationOptions`][crate::extensions::filters::network::http_connection_manager::PathNormalisationOptions] will allow incoming requests with path `//dir///file` to match against route with `prefix` match set to `/dir`. When using for header transformations, note that slash merging is not part of [HTTP spec](https://www.rfc-editor.org/rfc/rfc3986) and is provided for convenience.
*/
pub struct MergeSlashes {
}
