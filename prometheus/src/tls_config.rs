/*!
<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#tls_config>
*/

use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TLSConfig {
    /// CA certificate to validate API server certificate with.
    ca_file: Option<PathBuf>,

    /// Certificate and key files for client cert authentication to the server.
    cert_file: Option<PathBuf>,
    key_file: Option<PathBuf>,

    /// ServerName extension to indicate the name of the server.
    /// https://tools.ietf.org/html/rfc4366#section-3.1
    server_name: Option<String>,

    /// Disable validation of the server certificate.
    insecure_skip_verify: Option<bool>,

    /// Minimum acceptable TLS version. Accepted values: TLS10 (TLS 1.0), TLS11 (TLS
    /// 1.1), TLS12 (TLS 1.2), TLS13 (TLS 1.3).
    /// If unset, Prometheus will use Go default minimum version, which is TLS 1.2.
    /// See MinVersion in https://pkg.go.dev/crypto/tls#Config.
    min_version: Option<String>
}
