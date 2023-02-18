/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/matcher/number.proto>
*/

use crate::types::range::F64Range;

/// Specifies the way to match a double value.
pub enum F64Matcher {
    // option (validate.required) = true;
    
    /**
    If specified, the input double value must be in the range specified here.
    Note: The range is using half-open interval semantics [start, end).
    */
    Range(F64Range),

    /// If specified, the input double value must be equal to the value specified here.
    Exact(f64),
}
