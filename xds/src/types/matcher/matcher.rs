/*!
<https://github.com/cncf/xds/blob/main/xds/type/matcher/v3/matcher.proto>
*/

use std::collections::HashMap;

use crate::{
    core::extension::TypedExtensionConfig,
    types::matcher::StringMatcher
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
    Optional OnMatch to use if no matcher above matched (e.g., if there are no matchers specified above, or if none of the matches specified above succeeded).
    If no matcher above matched and this field is not populated, the match will be considered unsuccessful.
    */
    on_no_match: Box<OnMatch>
}


pub enum MatcherType {
    /// A linear list of matchers to evaluate.
    MatcherList(MatcherList),

    /// A match tree to evaluate.
    MatcherTree(MatcherTree)
}

pub struct MatcherTree {
    /// Protocol-specific specification of input field to match on.
    /// [(validate.rules).message = {required: true}];
    input: TypedExtensionConfig,

    /**
    Exact or prefix match maps in which to look up the input value.
    If the lookup succeeds, the match is considered successful, and the corresponding OnMatch is used.
    */
    tree_type: TreeType
}

/**
Exact or prefix match maps in which to look up the input value.
If the lookup succeeds, the match is considered successful, and the corresponding OnMatch is used.
*/
pub enum TreeType {
    // option (validate.required) = true;

    ExactMatchMap(MatchMap),

    /// Longest matching prefix wins.
    PrefixMatchMap(MatchMap),

    /// Extension for custom matching logic.
    CustomMatch(TypedExtensionConfig)
}

/// A map of configured matchers. Used to allow using a map within a oneof.
pub struct MatchMap {
    /// [(validate.rules).map = {min_pairs: 1}];
    map: HashMap<String, OnMatch>
}

/// What to do if a match is successful.
pub enum OnMatch {
    // option (validate.required) = true;

    /**
    Nested matcher to evaluate.
    If the nested matcher does not match and does not specify
    on_no_match, then this matcher is considered not to have
    matched, even if a predicate at this level or above returned
    true.
    */
    Matcher(Box<Matcher>),

    /// Protocol-specific action to take.
    Action(TypedExtensionConfig)
}

// A linear list of field matchers.
// The field matchers are evaluated in order, and the first match
// wins.
pub struct MatcherList {
    /// A list of matchers. First match wins.
    /// [(validate.rules).repeated = {min_items: 1}];
    matchers: Vec<FieldMatcher>
}

/// Predicate to determine if a match is successful.
pub struct Predicate {
    match_type: Box<MatchType>
}

pub enum MatchType {
    // option (validate.required) = true;

    /// A single predicate to evaluate.
    SinglePredicate(SinglePredicate),

    /// A list of predicates to be OR-ed together.
    OrMatcher(PredicateList),

    /// A list of predicates to be AND-ed together.
    AndMatcher(PredicateList),

    /// The invert of a predicate
    NotMatcher(Box<Predicate>)
}

/// Predicate for a single input field.
pub struct SinglePredicate {
    /// Protocol-specific specification of input field to match on.
    /// [#extension-category: envoy.matching.common_inputs]
    /// [(validate.rules).message = {required: true}];
    input: TypedExtensionConfig,

    matcher: _Matcher
}

pub enum _Matcher {
    // option (validate.required) = true;

    /// Built-in string matcher.
    ValueMatch(StringMatcher),

    /// Extension for custom matching logic.
    /// [#extension-category: envoy.matching.input_matchers]
    CustomMatch(TypedExtensionConfig)
}

/// A list of two or more matchers. Used to allow using a list within a oneof.
pub struct PredicateList {
    /// [(validate.rules).repeated = {min_items: 2}];
    predicate: Vec<Predicate>
}

/// An individual matcher.
pub struct FieldMatcher {
    /// Determines if the match succeeds.
    /// [(validate.rules).message = {required: true}];
    predicate: Predicate,

    /// What to do if the match succeeds.
    /// [(validate.rules).message = {required: true}];
    on_match: OnMatch
}
