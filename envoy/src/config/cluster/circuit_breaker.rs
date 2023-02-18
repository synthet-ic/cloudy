/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/cluster/v3/circuit_breaker.proto>
*/

use crate::{
    config::core::base::RoutingPriority,
    types::percent::Percent
};

/**
:ref:`Circuit breaking <arch_overview_circuit_break>` settings can be
specified individually for each defined priority.
*/
pub struct CircuitBreakers {
    /**
    If multiple `thresholds` are defined with the same [`RoutingPriority`], the first one in the list is used. If no Thresholds is defined for a given [`RoutingPriority`], the default values are used.
    */
    thresholds: Vec<Thresholds>,

    /**
    Optional per-host limits which apply to each individual host in a cluster.

    > NOTE: currently only the [`max_connections`][Thresholds::max_connections] field is supported for per-host limits.

    If multiple per-host [`thresholds`][Self::thresholds] are defined with the same [`RoutingPriority`], the first one in the list is used. If no per-host Thresholds are defined for a given [`RoutingPriority`], the cluster will not have per-host limits.
    */
    per_host_thresholds: Vec<Thresholds>,
}

/// A Thresholds defines CircuitBreaker settings for a [`RoutingPriority`].
pub struct Thresholds {
    /// The [`RoutingPriority`] the specified CircuitBreaker settings apply to.
    //[(validate.rules).enum = {defined_only: true}];
    priority: RoutingPriority,

    /// The maximum number of connections that Envoy will make to the upstream cluster. If not specified, the default is 1024.
    max_connections: u32,

    /**
    The maximum number of pending requests that Envoy will allow to the upstream cluster. If not specified, the default is 1024.
    This limit is applied as a connection limit for non-HTTP traffic.
    */
    max_pending_requests: u32,

    /**
    The maximum number of parallel requests that Envoy will make to the upstream cluster. If not specified, the default is 1024.
    This limit does not apply to non-HTTP traffic.
    */
    max_requests: u32,

    /// The maximum number of parallel retries that Envoy will allow to the upstream cluster. If not specified, the default is 3.
    max_retries: u32,

    /**
    Specifies a limit on concurrent retries in relation to the number of active requests. This parameter is optional.

    > NOTE: If this field is set, the retry budget will override any configured retry circuit breaker.
    */
    retry_budget: RetryBudget,

    /**
    If track_remaining is true, then stats will be published that expose the number of resources remaining until the circuit breakers open. If not specified, the default is false.

    > NOTE: If a retry budget is used in lieu of the max_retries circuit breaker, the remaining retry resources remaining will not be tracked.
    */
    track_remaining: bool,

    /**
    The maximum number of connection pools per cluster that Envoy will concurrently support at once. If not specified, the default is unlimited. Set this for clusters which create a large number of connection pools. See :ref:`Circuit Breaking <arch_overview_circuit_break_cluster_maximum_connection_pools>` for more details.
    */
    max_connection_pools: u32,
}

pub struct RetryBudget {
    /**
    Specifies the limit on concurrent retries as a percentage of the sum of active requests and active pending requests. For example, if there are 100 active requests and the `budget_percent` is set to 25, there may be 25 active retries.

    This parameter is optional. Defaults to 20%.
    */
    budget_percent: Option<Percent>,

    /**
    Specifies the minimum retry concurrency allowed for the retry budget. The limit on the number of active retries may never go below this number.

    This parameter is optional. Defaults to 3.
    */
    min_retry_concurrency: Option<u32>,
}
