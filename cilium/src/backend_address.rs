pub struct BackendAddress {
    /// Layer 3 address
    /// Required: true
    ip: Option<String>,

    /// Optional name of the node on which this backend runs
    node_name: String,

    /// Layer 4 port number
    port: u16,

    /// Indicator if this backend is preferred in the context of clustermesh service affinity. The value is set based
    /// on related annotation of global service. Applicable for active state only.
    preferred: bool,

    /// State of the backend for load-balancing service traffic
    /// Enum: [active terminating quarantined maintenance]
    state: String
}
