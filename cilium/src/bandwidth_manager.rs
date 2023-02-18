pub struct BandwidthManager {
    /// congestion control
    /// Enum: [cubic bbr]
    congestion_control: String,

    /// devices
    devices: Vec<String>,

    /// Is bandwidth manager enabled
    enabled: bool
}
