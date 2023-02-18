/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/backoff.proto>
*/

use std::time::Duration;

/// Configuration defining a jittered exponential back off strategy.
pub struct BackoffStrategy {
    /// The base interval to be used for the next back off computation. It should be greater than zero and less than or equal to [`max_interval`][Self::max_interval].
    // [>= Duration::from_nanos(1000000)]
    base_interval: Duration,

    /// Specifies the maximum interval between retries. This parameter is optional, but must be greater than or equal to the [`base_interval`][Self::base_interval] if set. The default is `10` times the [`base_interval`][Self::base_interval].
    // [gt {}]
    max_interval: Option<Duration>
}
