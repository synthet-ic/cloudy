/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/admin/v3/metrics.proto>
*/

/// Proto representation of an Envoy Counter or Gauge value.
pub struct SimpleMetric {
    /// Type of the metric represented.
    r#type: Type,

    /// Current metric value.
    value: u64,

    /// Name of the metric.
    name: String
}

pub enum Type {
    Counter,
    Gauge
}
