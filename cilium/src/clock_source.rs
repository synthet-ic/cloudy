pub struct ClockSource {
    /// Kernel Hz
    hertz: i64,

    /// Datapath clock source
    /// Enum: [ktime jiffies]
    mode: String
}
