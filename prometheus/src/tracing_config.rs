/*!
<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#tracing_config>
*/

use std::{
    collections::HashMap,
    time::Duration
};

use serde::{Serialize, Deserialize};

use crate::tls_config::TLSConfig;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Client used to export the traces. Options are 'http' or 'grpc'.
    #[serde(default = "grpc")]
    client_type: Option<ClientType>,

    /// Endpoint to send the traces to. Should be provided in format <host>:<port>.
    endpoint: Option<String>,

    /// Sets the probability a given trace will be sampled. Must be a float from 0 through 1.
    #[serde(default = 0)]
    sampling_fraction: Option<f32>,

    /// If disabled, the client will use a secure connection.
    #[serde(default = false)]
    insecure: Option<bool>,

    /// Key-value pairs to be used as headers associated with gRPC or HTTP requests.
    headers: Option<HashMap<String, String>>,

    /// Compression key for supported compression types. Supported compression: gzip.
    compression: Option<String>,

    /// Maximum time the exporter will wait for each batch export.
    #[serde(default = "10s")]
    timeout: Option<Duration>,

    /// TLS configuration.
    tls_config: Option<TLSConfig>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientType {
    HTTP,
    #[default]
    GRPC
}
