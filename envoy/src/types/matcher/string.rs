/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/matcher/string.proto>
*/

use crate::types::matcher::regex::RegexMatcher;

/// Specifies the way to match a string.
pub struct StringMatcher {
    match_pattern: MatchPattern,

    /**
    If `true`, indicates the exact/prefix/suffix matching should be case insensitive. This has no effect for the safe_regex match.
    For example, the matcher *data* will match both input string *Data* and *data* if set to true.
    */
    ignore_case: bool
}

pub enum MatchPattern {
    // option (validate.required) = true;

    /**
    The input string must match exactly the string specified here.

    Examples:

    - *abc* only matches the value *abc*.
    */
    Exact(String),

    /**
    The input string must have the prefix specified here.
    Note: empty prefix is not allowed, please use regex instead.

    Examples:

    - *abc* matches the value *abc.xyz*

    [(validate.rules).string = {min_len: 1}];
    */
    Prefix(String),

    /**
    The input string must have the suffix specified here.
    Note: empty prefix is not allowed, please use regex instead.

    Examples:

    - *abc* matches the value *xyz.abc*

    [(validate.rules).string = {min_len: 1}];
    */
    Suffix(String),

    /// The input string must match the regular expression specified here.
    // [(validate.rules).message = {required: true}];
    SafeRegex(RegexMatcher)
}

/// Specifies a list of ways to match a string.
pub struct ListStringMatcher {
    ///  [(validate.rules).repeated = {min_items: 1}];
    patterns: Vec<StringMatcher>
}
