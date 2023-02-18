/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/percent.proto>
*/

use std::time::Duration;

/// Configures a token bucket, typically used for rate limiting.
pub struct TokenBucket {
    /// The maximum tokens that the bucket can hold. This is also the number of tokens that the bucket initially contains.
    // [max_tokens > 0]
    max_tokens: u32,

    /// The number of tokens added to the bucket during each fill interval. If not specified, defaults to a single token.
    // [tokens_per_fill > 0
    tokens_per_fill: u32,

    /**
    The fill interval that tokens are added to the bucket. During each fill interval `tokens_per_fill` are added to the bucket. The bucket will never contain more than `max_tokens` tokens.
    [(validate.rules).duration = {
      required: true
      gt {}
    }]
    */
    fill_interval: Duration
}
