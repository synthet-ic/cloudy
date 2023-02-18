/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/v3/ratelimit_unit.proto>
*/

/// Identifies the unit of of time for rate limit.
pub enum RateLimitUnit {
    /// The time unit is not known.
    Unknown,

    /// The time unit representing a second.
    Second,

    /// The time unit representing a minute.
    Minute,

    /// The time unit representing an hour.
    Hour,

    /// The time unit representing a day.
    Day,

    /// The time unit representing a month.
    Month,

    /// The time unit representing a year.
    Year
}
