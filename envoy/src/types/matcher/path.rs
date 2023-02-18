/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/matcher/path.proto>
*/

use crate::types::matcher::string::StringMatcher;

/// Specifies the way to match a path on HTTP request.
pub enum PathMatcher {
    // option (validate.required) = true;

    /**
    The `path` must match the URL path portion of the :path header. The query and fragment string (if present) are removed in the URL path portion.
    For example, the path *\/data* will match the *:path* header *\/data#fragment?param=value*.

    [(validate.rules).message = {required: true}];
    */
    Path(StringMatcher)
}
