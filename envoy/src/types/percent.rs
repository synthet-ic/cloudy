/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/percent.proto>
*/

/// Identifies a percentage, in the range [0.0, 100.0].
pub struct Percent {
    /// [0.0 <= value <= 100.0]
    value: f64
}

/**
A fractional percentage is used in cases in which for performance reasons performing floating point to integer conversions during randomness calculations is undesirable. The message includes both a numerator and denominator that together determine the final fractional value.

- **Example**: 1/100 = 1%.
- **Example**: 3/10000 = 0.03%.
*/
pub struct FractionalPercent {
    /// Specifies the numerator. Defaults to 0.
    numerator: u32,

    /// Specifies the denominator. If the denominator specified is less than the numerator, the final fractional percentage is capped at 1 (100%).
    // [(validate.rules).enum = {defined_only: true}]
    denominator: DenominatorType
}

/// Fraction percentages support several fixed denominator values.
pub enum DenominatorType {
    /**
    100.

    **Example**: 1/100 = 1%.
    */
    Hundred,

    /**
    10,000.

    **Example**: 1/10000 = 0.01%.
    */
    TenThousand,

    /**
    1,000,000.

    **Example**: 1/1000000 = 0.0001%.
    */
    Million
}
