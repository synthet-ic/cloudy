/*!
<https://github.com/cncf/xds/blob/main/xds/type/matcher/v3/string.proto>
*/

use super::regex::RegexMatcher;

/// Specifies the way to match a string.
pub struct StringMatcher {
    match_pattern: MatchPattern,

    /**
    If `true`, indicates the exact/prefix/suffix matching should be case insensitive. This has no effect for the safe_regex match.
    For example, the matcher *data* will match both input string *Data* and *data* if set to `true`.
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

    /*
    The input string must match the regular expression specified here.

    [(validate.rules).message = {required: true}];
    */
    SafeRegex(RegexMatcher),

    /*
    The input string must have the substring specified here.
    Note: empty contains match is not allowed, please use regex instead.

    Examples:

    - *abc* matches the value *xyz.abc.def*

    [(validate.rules).string = {min_len: 1}];
    */
    Contains(String)
  }
