pub mod authorisation;
pub mod basic_auth;
pub mod kubernetes_sd_config;
pub mod remote_read;
pub mod oauth2;
pub mod remote_write;
pub mod scrape_config;
pub mod tls_config;
pub mod tracing_config;

use std::time::Duration;

use serde::{Serialize, Deserialize};

use scrape_config::ScrapeConfig;
use tracing_config::TracingConfig;

pub struct Prometheus {
    global: Option<Global>,

    /// How long until a scrape request times out.
    #[serde(default = "10s")]
    scrape_timeout: Option<Duration>,

    /// How frequently to evaluate rules.
    #[serde(default = "1m")]
    evaluation_interval: Option<Duration>,

    /// The labels to add to any time series or alerts when communicating with
    /// external systems (federation, remote storage, Alertmanager).
    external_labels: Option<HashMap<labelname, labelvalue>>,

    /// File to which PromQL queries are logged.
    /// Reloading the configuration will reopen the file.
    query_log_file: Option<String>,

    /// Rule files specifies a list of globs. Rules and alerts are read from
    /// all matching files.
    rule_files: Option<Vec<filepath_glob>>,

    /// A list of scrape configurations.
    scrape_configs: Option<Vec<ScrapeConfig>>,

    /// Alerting specifies settings related to the Alertmanager.
    alerting: Option<Alerting>,

    /// Settings related to the remote write feature.
    remote_write: Option<Vec<RemoteWrite>>,

    /// Settings related to the remote read feature.
    remote_read: Option<Vec<RemoteRead>>,

    /// Storage related settings that are runtime reloadable.
    storage: Option<Storage>,

    /// Configures exporting traces.
    tracing: Option<TracingConfig>
}

pub struct Global {
    /// How frequently to scrape targets by default.
    #[serde(default = "1m")]
    scrape_interval: Option<Duration>
}

pub struct Alerting {
    alert_relabel_configs: Option<Vec<RelabelConfig>>,
    #[alertmanagers]
    alert_managers: Option<Vec<AlertManagerConfig>>
}

pub struct Storage {
    tsdb: Option<TSDB>,
    exemplars: Option<Exemplars>
}
