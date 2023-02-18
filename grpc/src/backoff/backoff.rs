/*!
<https://github.com/grpc/grpc-go/blob/master/backoff/backoff.go>
*/

use std::{
    default::Default,
    time::Duration
};

/// Config defines the configuration options for backoff.
pub struct Config {
    /// base_delay is the amount of time to backoff after the first failure.
    pub base_delay: Duration,
    /// multiplier is the factor with which to multiply backoffs after a
    /// failed retry. Should ideally be greater than 1.
    pub multiplier: f64,
    /// jitter is the factor with which backoffs are randomized.
    pub jitter: f64,
    /// max_delay is the upper bound of backoff delay.
    pub max_delay: Duration
}

/**
DefaultConfig is a backoff configuration with the default values specfied
at <https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md>.

This should be useful for callers who want to configure backoff with non-default values only for a subset of the options.
*/
impl Default for Config {
    fn default() -> Self {
        Self {
            base_delay: 1.0 * time.Second,
            multiplier: 1.6,
            jitter: 0.2,
            max_delay: 120 * time.Second,
        }
    }
}
