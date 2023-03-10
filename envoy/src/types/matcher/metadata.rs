/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/matcher/metadata.proto>
*/

use crate::types::matcher::value::ValueMatcher;

/**
MetadataMatcher provides a general interface to check if a given value is matched in [`Metadata`][crate::core.Metadata]. It uses `filter` and `path` to retrieve the value from the Metadata and then check if it's matched to the specified value.

For example, for the following Metadata:

```yaml
filter_metadata:
  envoy.filters.http.rbac:
    fields:
      a:
        struct_value:
          fields:
            b:
              struct_value:
                fields:
                  c:
                    string_value: pro
            t:
              list_value:
                values:
                - string_value: m
                - string_value: n
```

The following MetadataMatcher is matched as the path [a, b, c] will retrieve a string value "pro" from the Metadata which is matched to the specified prefix match.

```yaml
filter: envoy.filters.http.rbac
path:
- key: a
- key: b
- key: c
value:
  string_match:
    prefix: pr
```

The following MetadataMatcher is matched as the code will match one of the string values in the
list at the path [a, t].

```yaml
filter: envoy.filters.http.rbac
path:
- key: a
- key: t
value:
  list_match:
    one_of:
      string_match:
        exact: m
```

An example use of MetadataMatcher is specifying additional metadata in envoy.filters.http.rbac to enforce access control based on dynamic metadata in a request. See [Permission][crate::config::rbac.v2.Permission>` and :ref:`Principal <crate::config::rbac.v2.Principal].
[#next-major-version: MetadataMatcher should use StructMatcher]
*/
pub struct MetadataMatcher {
    /// The filter name to retrieve the Struct from the Metadata.
    // [!is_empty()]
    filter: String,

    /// The path to retrieve the Value from the Struct.
    // [(validate.rules).repeated = {min_items: 1}];
    path: Vec<PathSegment>,

    /// The MetadataMatcher is matched if the value retrieved by path is matched to this value.
    // [(validate.rules).message = {required: true}];
    value: ValueMatcher
}

/**
Specifies the segment in a path to retrieve value from Metadata.
Note: Currently it's not supported to retrieve a value from a list in Metadata. This means that if the segment key refers to a list, it has to be the last segment in a path.
*/
pub struct PathSegment {
    // option (validate.required) = true;

    /// If specified, use the key to retrieve the value in a Struct.
    // [!is_empty()]
    key: String
}
