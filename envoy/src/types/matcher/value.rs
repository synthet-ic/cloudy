/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/matcher/value.proto>
*/

use crate::types::matcher::{
    number::F64Matcher,
    string::StringMatcher
};

/**
Specifies the way to match a ProtobufWkt::Value. Primitive values and ListValue are supported.
StructValue is not supported and is always not matched.
*/
pub enum ValueMatcher {  
    // option (validate.required) = true;
    
    /// If specified, a match occurs if and only if the target value is a NullValue.
    NullMatch(NullMatch),

    /**
    If specified, a match occurs if and only if the target value is a double value and is
    matched to this field.
    */
    F64Matcher(F64Matcher),

    /**
    If specified, a match occurs if and only if the target value is a string value and is matched to this field.
    */
    StringMatcher(StringMatcher),

    /**
    If specified, a match occurs if and only if the target value is a bool value and is equal to this field.
    */
    BoolMatch(bool),

    /**
    If specified, value match will be performed based on whether the path is referring to a valid primitive value in the metadata. If the path is referring to a non-primitive value, the result is always not matched.
    */
    PresentMatch(bool),

    /**
    If specified, a match occurs if and only if the target value is a list value and is matched to this field.
    */
    ListMatcher(Box<ListMatcher>),
}

/// NullMatch is an empty message to specify a null value.
pub struct NullMatch {
}
  
/// Specifies the way to match a list value.
pub enum ListMatcher {
    // option (validate.required) = true;
  
    /// If specified, at least one of the values in the list must match the value specified.
    OneOf(ValueMatcher)
}
