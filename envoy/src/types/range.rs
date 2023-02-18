/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/range.proto>
*/


/// Specifies the i64 start and end of the range using half-open interval semantics [start, end).
pub struct I64Range {
    // start of the range (inclusive)
    start: i64,

    // end of the range (exclusive)
    end: i64,
}

/// Specifies the i32 start and end of the range using half-open interval semantics [start, end).
pub struct I32Range {
    // start of the range (inclusive)
    start: i32,

    // end of the range (exclusive)
    end: i32,
}

/// Specifies the f64 start and end of the range using half-open interval semantics [start, end).
pub struct F64Range {
    // start of the range (inclusive)
    start: f64,

    // end of the range (exclusive)
    end: f64,
}
