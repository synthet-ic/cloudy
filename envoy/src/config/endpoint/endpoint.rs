/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/endpoint/v3/endpoint.proto>
*/

use std::{
    collections::HashMap,
    time::Duration,
};

use crate::{
    config::endpoint::endpoint_components::{Endpoint, LocalityLBEndpoints},
    types::percent::FractionalPercent
};

// [#protodoc-title: Endpoint configuration]
// Endpoint discovery :ref:`architecture overview <arch_overview_service_discovery_types_eds>`

/**
Each route from RDS will map to a single cluster or traffic split across clusters using weights expressed in the RDS WeightedCluster.

With EDS, each cluster is treated independently from a LB perspective, with LB taking place between the Localities within a cluster and at a finer granularity between the hosts within a locality. The percentage of traffic for each endpoint is determined by both its load_balancing_weight, and the load_balancing_weight of its locality. First, a locality will be selected, then an endpoint within that locality will be chose based on its weight.
*/
pub struct ClusterLoadAssignment {
    /**
    Name of the cluster. This will be the [`service_name`][crate::config::cluster::cluster::EDSClusterConfig::service_name] value if specified in the cluster [`EDSClusterConfig`][crate::config::cluster::cluster::EDSClusterConfig].
    */
    // [!cluster_name.is_empty()]
    cluster_name: String,

    /// List of endpoints to load balance to.
    endpoints: Vec<LocalityLBEndpoints>,

    /// Map of named endpoints that can be referenced in LocalityLBEndpoints.
    named_endpoints: HashMap<String, Endpoint>,

    /// Load balancing policy settings.
    policy: Policy
}

/// Load balancing policy settings.
pub struct Policy {
    /**
    Action to trim the overall incoming traffic to protect the upstream hosts. This action allows protection in case the hosts are unable to recover from an outage, or unable to autoscale or unable to handle incoming traffic volume for any reason.

    At the client each category is applied one after the other to generate the 'actual' drop percentage on all outgoing traffic. For example:

    ```json
    { "drop_overloads": [
         { "category": "throttle", "drop_percentage": 60 }
         { "category": "lb", "drop_percentage": 50 }
     ]}
    ```

    The actual drop percentages applied to the traffic at the clients will be
       "throttle"_drop = 60%
       "lb"_drop = 20%  // 50% of the remaining 'actual' load, which is 40%.
       actual_outgoing_load = 20% // remaining after applying all categories.
    */
    drop_overloads: Vec<DropOverload>,

    /**
    Priority levels and localities are considered overprovisioned with this factor (in percentage). This means that we don't consider a priority level or locality unhealthy until the fraction of healthy hosts multiplied by the overprovisioning factor drops below 100.
    With the default value 140(1.4), Envoy doesn't consider a priority level or a locality unhealthy until their percentage of healthy hosts drops below 72%. For example:

    ```json
    { "overprovisioning_factor": 100 }
    ```
    
    Read more at :ref:`priority levels <arch_overview_load_balancing_priority_levels>` and :ref:`localities <arch_overview_load_balancing_locality_weighted_lb>`.
    */
    // [overprovisioning_factor > 0]
    overprovisioning_factor: u32,

    /**
    The max time until which the endpoints from this assignment can be used.
    If no new assignments are received before this time expires the endpoints are considered stale and should be marked unhealthy.
    Defaults to `0` which means endpoints never go stale.
    */
    // [(validate.rules).duration = {gt {}}]
    endpoint_stale_after: Duration
}

pub struct DropOverload {
    /// Identifier for the policy specifying the drop.
    /// [!category.is_empty()]
    category: String,

    /// Percentage of traffic that should be dropped for the category.
    drop_percentage: FractionalPercent
}
