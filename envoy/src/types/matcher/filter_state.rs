/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/matcher/v3/filter_state.proto>
*/

use crate::types::matcher::string::StringMatcher;

/// FilterStateMatcher provides a general interface for matching the filter state objects.
pub struct  FilterStateMatcher {
    /// The filter state key to retrieve the object.
    // [!is_empty()]
    key: String,

    matcher: Matcher
}

pub enum Matcher {
    // option (validate.required) = true;

    /// Matches the filter state object as a string value.
    StringMatch(StringMatcher)
}
