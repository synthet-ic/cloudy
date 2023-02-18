/*!
<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#remote_write>
*/

use std::{
  collections::HashMap,
  path::PathBuf
};

use serde::{Serialize, Deserialize};

use crate::{
    authorisation::Authorisation,
    basic_auth::BasicAuth,
    oauth2::OAuth2,
    tls_config::TLSConfig
};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteWrite {
    /// The URL of the endpoint to send samples to.
    url: <String>,

    /// Timeout for requests to the remote write endpoint.
    #[serde(default = "30s")]
    remote_timeout: Option<Duration>,

    /**
    Custom HTTP headers to be sent along with each remote write request.
    Be aware that headers that are set by Prometheus itself can't be overwritten.
    */
    headers: Option<HashMap<String, String>>,

    /// List of remote write relabel configurations.
    write_relabel_configs: Option<Vec<RelabelConfig>>,

    /**
    Name of the remote write config, which if specified must be unique among remote write configs.
    The name will be used in metrics and logging in place of a generated value to help users distinguish between
    remote write configs.
    */
    name: Option<String>,

    /// Enables sending of exemplars over remote write. Note that exemplar storage itself must be enabled for exemplars to be scraped in the first place.
    #[serde(default = "false")]
    send_exemplars: Option<bool>,

    /**
    Sets the `Authorization` header on every remote write request with the
    configured username and password.
    password and password_file are mutually exclusive.
    */
    basic_auth: Option<BasicAuth>,

    /// Optional `Authorization` header configuration.
    #[serde(rename(serialize = "authorization"))]
    authorisation: Option<Authorisation>,

    /**
    Optionally configures AWS's Signature Verification 4 signing process to
    sign requests. Cannot be set at the same time as basic_auth, authorization, or oauth2.
    To use the default credentials from the AWS SDK, use `sigv4: {}`.
    */
    sigv4: Option<Sigv4>,

    /**
    Optional OAuth 2.0 configuration.
    Cannot be used at the same time as basic_auth, authorization, or sigv4.
    */
    oauth2: Option<OAuth2>,

    /// Configures the remote write request's TLS settings.
    tls_config: Option<TLSConfig>,

    /// Optional proxy URL.
    proxy_url: Option<String>,

    /// Configure whether HTTP requests follow HTTP 3xx redirects.
    #[serde(rename(serialize = "true"))]
    follow_redirects: Option<bool>,

    /// Whether to enable HTTP2.
    #[serde(rename(serialize = "true"))]
    enable_http2: Option<bool>,

    /// Configures the queue used to write to remote storage.
    queue_config: Option<QueueConfig>,

    /**
    Configures the sending of series metadata to remote storage.
    Metadata configuration is subject to change at any point
    or be removed in future releases.
    */
    metadata_config: Option<MetadataConfig>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthorisationType {
    Bearer
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sigv4 {
    /// The AWS region. If blank, the region from the default credentials chain
    /// is used.
    region: Optoin<String>,

    /// The AWS API keys. If blank, the environment variables `AWS_ACCESS_KEY_ID`
    /// and `AWS_SECRET_ACCESS_KEY` are used.
    access_key: Optoin<String>,
    secret_key: Optoin<Secret>,

    /// Named AWS profile used to authenticate.
    profile: Optoin<String>,

    /// AWS Role ARN, an alternative to using AWS API keys.
    role_arn: Optoin<String>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct QueueConfig {
    /**
    Number of samples to buffer per shard before we block reading of more
    samples from the WAL. It is recommended to have enough capacity in each
    shard to buffer several requests to keep throughput up while processing
    occasional slow remote requests.
    */
    #[serde(rename(serialize = "2500"))]
    capacity: Optoin<i32>,
    /// Maximum number of shards, i.e. amount of concurrency.
    #[serde(rename(serialize = "200"))]
    max_shards: Optoin<i32>,
    /// Minimum number of shards, i.e. amount of concurrency.
    #[serde(rename(serialize = "1"))]
    min_shards: Optoin<i32>,
    /// Maximum number of samples per send.
    #[serde(rename(serialize = "500"))]
    max_samples_per_send: Optoin<i32>,
    /// Maximum time a sample will wait in buffer.
    #[serde(rename(serialize = "5s"))]
    batch_send_deadline: Optoin<Duration>,
    /// Initial retry delay. Gets doubled for every retry.
    #[serde(rename(serialize = "30ms"))]
    min_backoff: Optoin<Duration>,
    /// Maximum retry delay.
    #[serde(rename(serialize = "5s"))]
    max_backoff: Optoin<Duration>,
    /**
    Retry upon receiving a 429 status code from the remote-write storage.
    This is experimental and might change in the future.
    */
    #[serde(rename(serialize = "false"))]
    retry_on_http_429: Optoin<bool>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataConfig {
    /// Whether metric metadata is sent to remote storage or not.
    #[serde(default = "true")]
    send: Optoin<bool>,
    /// How frequently metric metadata is sent to remote storage.
    #[serde(default = "1m")]
    send_interval: Optoin<Duration>,
    /// Maximum number of samples per send.
    #[serde(default = "500")]
    max_samples_per_send: Optoin<i32>
}
