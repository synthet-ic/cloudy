/*!
<https://github.com/containerd/containerd/blob/main/docs/man/containerd-config.toml.5.md>
*/

pub mod plugins;

use std::{path::PathBuf, time::Duration};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Containerd {
    version: u16,
    root: PathBuf,
    plugin_dir: PathBuf,
    grpc: GRPC,
    ttrpc: TTRPC,
    debug: ContainerdDebug,
    metrics: Metrics,
    disabled_plugins: Vec<String>,
    required_plugins: Vec<String>,
    plugins: String,
    // #[serde(default = "0")]
    oom_score: u16,
    cgroup: CGroup,
    proxy_plugins: ProxyPlugins,
    timeouts: Duration,
    imports: Vec<PathBuf>,
    stream_processors: StreamProcessors
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct GRPC {
    // #[serde(default = "/run/containerd/containerd.sock")]
    address: PathBuf,
    tcp_address: PathBuf,
    tcp_tls_cert: PathBuf,
    tcp_tls_key: PathBuf,
    // #[serde(default = "0")]
    uid: u16,
    // #[serde(default = "0")]
    gid: u16,
    max_recv_message_size: u16,
    max_send_message_size: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TTRPC {
    // #[serde(default = "")]
    address: PathBuf,
    // #[serde(default = "0")]
    uid: u16,
    // #[serde(default = "0")]
    gid: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerdDebug {
    // #[serde(default = "/run/containerd/debug.sock")]
    address: PathBuf,
    // #[serde(default = "0")]
    uid: u16,
    // #[serde(default = "0")]
    gid: u16,
    // #[serde(default = "Level::Info")]
    level: Level,
    format: Format
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    Text,
    JSON
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    // #[serde(default = "")]
    address: PathBuf,
    // #[serde(default = "bool::default")]
    grpc_histogram: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CGroup {
    // #[serde(default = "")]
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyPlugins {
    // #[serde(default = "")]
    r#type: String,
    // #[serde(default = "")]
    address: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamProcessors {
    // #[serde(default = "")]
    accepts: String,
    // #[serde(default = "")]
    returns: PathBuf,
    // #[serde(default = "")]
    path: String,
    // #[serde(default = "")]
    args: PathBuf,
}
