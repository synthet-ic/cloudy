pub struct BPFMap {
    /// Contents of cache
    cache: Vec<BPFMapEntry>,

    /// Path to BPF map
    path: String
}

pub struct BPFMapEntry {
    /// Desired action to be performed
    /// Enum: [ok insert delete]
    desired_action: String,

    /// Key of map entry
    key: String,

    /// Last error seen while performing desired action
    last_error: String,

    /// Value of map entry
    value: String
}

pub struct BPFMapProperties {
    /// Name of the BPF map
    name: String,

    /// Size of the BPF map
    size: i64
}

pub struct BPFMapStatus {
    /// Ratio of total system memory to use for dynamic sizing of BPF maps
    dynamic_size_ratio: f64,

    /// BPF maps
    maps: Vec<BPFMapProperties>
}
