/*!
<https://github.com/grpc/grpc-go/blob/master/backoff.go>
*/

pub mod backoff;

use std::time::Duration;

use backoff::Config;

/**
ConnectParams defines the parameters for connecting and retrying. Users are encouraged to use this instead of the BackoffConfig type defined above. See
here for more details: <https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md>.

# Experimental

Notice: This type is EXPERIMENTAL and may be changed or removed in a later release.
*/
pub struct ConnectParams {
    /// backoff specifies the configuration options for connection backoff.
    pub backoff: Config,
    /// Minimum amount of time we are willing to give a connection to complete.
    pub min_connect_timeout: Duration
}
