/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/common/matcher/v3/matcher.proto>
*/

use std::collections::HashMap;

use crate::{
    config::{
        core::extension::TypedExtensionConfig,
        route::route_components::HeaderMatcher,
    },
    types::matcher::string::StringMatcher
};

/**
A matcher, which may traverse a matching tree in order to result in a match action.
During matching, the tree will be traversed until a match is found, or if no match is found the action specified by the most specific on_no_match will be evaluated.
As an on_no_match might result in another matching tree being evaluated, this process might repeat several times until the final OnMatch (or no match) is decided.
*/
pub struct Matcher {
    // option (xds.annotations::message_status).work_in_progress = true;
  
    matcher_type: MatcherType,
  
    /**
    Optional OnMatch to use if the matcher failed.
    If specified, the OnMatch is used, and the matcher is considered to have matched.
    If not specified, the matcher is considered not to have matched.
    */
    on_no_match: Box<OnMatch>,
}

pub enum MatcherType {
    // option (validate.required) = true;

    /// A linear list of matchers to evaluate.
    MatcherList(MatcherList),

    /// A match tree to evaluate.
    MatcherTree(MatcherTree),
}

/// What to do if a match is successful.
pub enum OnMatch {
    // option (validate.required) = true;

    /**
    Nested matcher to evaluate.
    If the nested matcher does not match and does not specify `on_no_match`, then this matcher is considered not to have matched, even if a predicate at this level or above returned `true`.
    */
    Matcher(Matcher),

    /// Protocol-specific action to take.
    Action(TypedExtensionConfig),
}

/**
 A linear list of field matchers.
The field matchers are evaluated in order, and the first match wins.
*/
pub struct MatcherList {
    /// A list of matchers. First match wins.
    // [(validate.rules).repeated = {min_items: 1}];
    matchers: Vec<FieldMatcher>, 
}

/// Predicate to determine if a match is successful.
pub enum Predicate {
    // option (validate.required) = true;

    /// A single predicate to evaluate.
    SinglePredicate(SinglePredicate),

    /// A list of predicates to be OR-ed together.
    OrMatcher(PredicateList),

    /// A list of predicates to be AND-ed together.
    AndMatcher(PredicateList),

    /// The invert of a predicate
    NotMatcher(Box<Predicate>),
}

/// Predicate for a single input field.
pub struct SinglePredicate {
    /// Protocol-specific specification of input field to match on.
    /// [#extension-category: envoy.matching.common_inputs]
    // [(validate.rules).message = {required: true}];
    input: TypedExtensionConfig,

    matcher: SinglePredicateMatcher
}

pub enum SinglePredicateMatcher {
    // option (validate.required) = true;

    /// Built-in string matcher.
    ValueMatch(StringMatcher),

    /**
    Extension for custom matching logic.
    [#extension-category: envoy.matching.input_matchers]
    */
    CustomMatch(TypedExtensionConfig),
}

/// A list of two or more matchers. Used to allow using a list within a oneof.
pub struct PredicateList {
    // [(validate.rules).repeated = {min_items: 2}];
    predicate: Vec<Predicate>
}

/// An individual matcher.
pub struct FieldMatcher {
    /// Determines if the match succeeds.
    // [(validate.rules).message = {required: true}];
    predicate: Predicate,

    /// What to do if the match succeeds.
    // [(validate.rules).message = {required: true}];
    on_match: OnMatch,
}

pub struct MatcherTree {
    /// Protocol-specific specification of input field to match on.
    // [(validate.rules).message = {required: true}];
    input: TypedExtensionConfig,

    /**
    Exact or prefix match maps in which to look up the input value.
    If the lookup succeeds, the match is considered successful, and the corresponding OnMatch is used.
    */
    tree_type: TreeType
}

/// A map of configured matchers. Used to allow using a map within a oneof.
// [(validate.rules).map = {min_pairs: 1}];
pub struct MatchMap {
    map: HashMap<String, OnMatch>,
}

pub enum TreeType {
    // option (validate.required) = true;

    ExactMatchMap(MatchMap),

    // Longest matching prefix wins.
    PrefixMatchMap(MatchMap),

    // Extension for custom matching logic.
    CustomMatch(TypedExtensionConfig),
}

/**
Match configuration. This is a recursive structure which allows complex nested match configurations to be built using various logical operators.
*/
pub enum MatchPredicate {  
    // option (validate.required) = true;
    
    /**
    A set that describes a logical OR. If any member of the set matches, the match configuration matches.
    */
    OrMatch(MatchSet),

    /**
    A set that describes a logical AND. If all members of the set match, the match configuration matches.
    */
    AndMatch(MatchSet),

    /**
    A negation match. The match configuration will match if the negated match condition matches.
    */
    NotMatch(Box<MatchPredicate>),

    /// The match configuration will always match.
    // [(validate.rules).bool = {const: true}];
    AnyMatch(bool),

    /// HTTP request headers match configuration.
    HTTPRequestHeadersMatch(HTTPHeadersMatch),

    /// HTTP request trailers match configuration.
    HTTPRequestTrailersMatch(HTTPHeadersMatch),

    /// HTTP response headers match configuration.
    HTTPResponseHeadersMatch(HTTPHeadersMatch),

    /// HTTP response trailers match configuration.
    HTTPResponseTrailersMatch(HTTPHeadersMatch),

    /// HTTP request generic body match configuration.
    HTTPRequestGenericBodyMatch(HTTPGenericBodyMatch),

    /// HTTP response generic body match configuration.
    HTTPResponseGenericBodyMatch(HTTPGenericBodyMatch),
}

/// A set of match configurations used for logical operations.
pub struct MatchSet {
    /// The list of rules that make up the set.
    // [(validate.rules).repeated = {min_items: 2}];
    rules: Vec<MatchPredicate>
}

// HTTP headers match configuration.
pub struct HTTPHeadersMatch {
    // HTTP headers to match.
    headers: Vec<HeaderMatcher>,
}
  
/**
HTTP generic body match configuration.
List of text strings and hex strings to be located in HTTP body.
All specified strings must be found in the HTTP body for positive match.
The search may be limited to specified number of bytes from the body start.

> attention: Searching for patterns in HTTP body is potentially cpu intensive. For each specified pattern, http body is scanned byte by byte to find a match.
> If multiple patterns are specified, the process is repeated for each pattern. If location of a pattern is known, ``bytes_limit`` should be specified to scan only part of the http body.
*/
pub struct HTTPGenericBodyMatch {  
    /// Limits search to specified number of bytes - default zero (no limit - match entire captured buffer).
    bytes_limit: u32,
  
    /// List of patterns to match.
    // [(validate.rules).repeated = {min_items: 1}];
    patterns: Vec<GenericTextMatch>,
}

pub enum GenericTextMatch {
    // option (validate.required) = true;

    /// Text string to be located in HTTP body.
    // [!is_empty()]
    StringMatch(String),

    /// Sequence of bytes to be located in HTTP body.
    // [(validate.rules).bytes = {min_len: 1}];
    BinaryMatch(Vec<u8>),
}
