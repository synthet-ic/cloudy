/*!
- <https://github.com/envoyproxy/envoy/blob/main/api/envoy/admin/v3/clusters.proto>
- <https://www.envoyproxy.io/docs/envoy/latest/api-v3/admin/v3/clusters.proto>
*/

use crate::{
    admin::metrics::SimpleMetric,
    config::{
        cluster::circuit_breaker::CircuitBreakers,
        core::{
            address::Address,
            base::Locality,
            health_check::HealthStatus
        }
    },
    types::percent::Percent,
};

/// Details an individual cluster's current status.
pub struct ClusterStatus {
    /// Name of the cluster.
    name: String,

    /// Denotes whether this cluster was added via API or configured statically.
    added_via_api: bool,

    /**
    The success rate threshold used in the last interval.
    If [`OutlierDetection::split_external_local_origin_errors`][crate::config::cluster::outlier_detection::OutlierDetection::split_external_local_origin_errors] is `false`, all errors: externally and locally generated were used to calculate the threshold.
    If [`OutlierDetection::split_external_local_origin_errors`][crate::config::cluster::outlier_detection::OutlierDetection::split_external_local_origin_errors] is `true`, only externally generated errors were used to calculate the threshold.
    The threshold is used to eject hosts based on their success rate. See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for details.

    Note: this field may be omitted in any of the three following cases:

    1. There were not enough hosts with enough request volume to proceed with success rate based outlier ejection.
    2. The threshold is computed to be < 0 because a negative value implies that there was no threshold for that interval.
    3. Outlier detection is not enabled for this cluster.
    */
    success_rate_ejection_threshold: Percent,

    /// Mapping from host address to the host's current status.
    host_statuses: Vec<HostStatus>,

    /**
    The success rate threshold used in the last interval when only locally originated failures were taken into account and externally originated errors were treated as success.
    This field should be interpreted only when [`OutlierDetection::split_external_local_origin_errors`][crate::config::cluster::outlier_detection::OutlierDetection::split_external_local_origin_errors] is `true`. The threshold is used to eject hosts based on their success rate.
    See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for details.

    > Note: this field may be omitted in any of the three following cases:

    1. There were not enough hosts with enough request volume to proceed with success rate based outlier ejection.
    2. The threshold is computed to be < 0 because a negative value implies that there was no threshold for that interval.
    3. Outlier detection is not enabled for this cluster.
    */
    local_origin_success_rate_ejection_threshold: Percent,

    /// :ref:`Circuit breaking <arch_overview_circuit_break>` settings of the cluster.
    circuit_breakers: CircuitBreakers,

    /// Observability name of the cluster.
    observability_name: String,

    /// The [EDS service name][crate::config::cluster::cluster::EDSClusterConfig::service_name] if the cluster is an EDS cluster.
    eds_service_name: String
}

/// Current state of a particular host.
pub struct HostStatus {
    /// Address of this host.
    address: Address,

    /// List of stats specific to this host.
    stats: Vec<SimpleMetric>,

    /// The host's current health status.
    health_status: HostHealthStatus,

    /**
    Request success rate for this host over the last calculated interval.
    If [`OutlierDetection::split_external_local_origin_errors`][crate::config::cluster::outlier_detection::OutlierDetection::split_external_local_origin_errors] is `false`, all errors: externally and locally generated were used in success rate calculation. If [`outlier_detection::split_external_local_origin_errors`][crate::config::cluster::outlier_detection::OutlierDetection::split_external_local_origin_errors] is `true`, only externally generated errors were used in success rate calculation.
    See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for details.

    > Note: the message will not be present if host did not have enough request volume to calculate success rate or the cluster did not have enough hosts to run through success rate outlier ejection.
    */
    success_rate: Percent,

    /// The host's weight. If not configured, the value defaults to 1.
    weight: u32,

    /// The hostname of the host, if applicable.
    hostname: String,

    /// The host's priority. If not configured, the value defaults to 0 (highest priority).
    priority: u32,

    /**
    Request success rate for this host over the last calculated interval when only locally originated errors are taken into account and externally originated errors were treated as success.
    This field should be interpreted only when [`OutlierDetection::split_external_local_origin_errors`][crate::config::cluster::outlier_detection::OutlierDetection::split_external_local_origin_errors] is `true`.
    See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for
    details.

    Note: the message will not be present if host did not have enough request volume to calculate success rate or the cluster did not have enough hosts to run through success rate outlier ejection.
    */
    local_origin_success_rate: Percent,

    /// locality of the host.
    locality: Locality,
}

/// Health status for a host.
pub struct HostHealthStatus {
    /// The host is currently failing active health checks.
    failed_active_health_check: bool,

    /// The host is currently considered an outlier and has been ejected.
    failed_outlier_check: bool,

    /// The host is currently being marked as degraded through active health checking.
    failed_active_degraded_check: bool,

    /// The host has been removed from service discovery, but is being stabilised due to active health checking.
    pending_dynamic_removal: bool,

    /// The host has not yet been health checked.
    pending_active_hc: bool,

    /// The host should be excluded from panic, spillover, etc. calculations because it was explicitly taken out of rotation via protocol signal and is not meant to be routed to.
    excluded_via_immediate_hc_fail: bool,

    /// The host failed active HC due to timeout.
    active_hc_timeout: bool,

    /// Health status as reported by EDS. Note: only HEALTHY and UNHEALTHY are currently supported here.
    // [#comment:TODO(mrice32): pipe through remaining EDS health status possibilities.]
    eds_health_status: HealthStatus,
}
