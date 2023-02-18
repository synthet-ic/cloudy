/*!
<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#remote_read>
*/

use http::Uri;

use serde::{Serialize, Deserialize};

use crate::{
    authorisation::Authorisation,
    basic_auth::BasicAuth,
};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteRead {
    /// The URL of the endpoint to query from.
    url: Uri,

    /// Name of the remote read config, which if specified must be unique among remote read configs.
    /// The name will be used in metrics and logging in place of a generated value to help users distinguish between
    /// remote read configs.
    name: Option<String>,

    /// An optional list of equality matchers which have to be
    /// present in a selector to query the remote read endpoint.
    required_matchers: Option<HashMap<labelname, labelvalue>>,

    /// Timeout for requests to the remote read endpoint.
    #[serde(default = "1m")]
    remote_timeout: Option<Duration>,

    /// Custom HTTP headers to be sent along with each remote read request.
    /// Be aware that headers that are set by Prometheus itself can't be overwritten.
    headers: Option<HashMap<String, String>>,

    /// Whether reads should be made for queries for time ranges that
    /// the local storage should have complete data for.
    #[serde(default = "false")]
    read_recent: Option<bool>,

    /// Sets the `Authorization` header on every remote read request with the
    /// configured username and password.
    /// password and password_file are mutually exclusive.
    basic_auth: Option<BasicAuth>,

    /// Optional `Authorization` header configuration.
    #[serde(rename(serialize = "authorization"))]
    authorisation: Option<Authorisation>,

    /// Optional OAuth 2.0 configuration.
    /// Cannot be used at the same time as basic_auth or authorization.
    oauth2: Option<OAuth2>,

    /// Configures the remote read request's TLS settings.
    tls_config: Option<TLSConfig>,

    /// Optional proxy URL.
    proxy_url: Option<Uri>,

    /// Configure whether HTTP requests follow HTTP 3xx redirects.
    #[serde(default = "true")]
    follow_redirects: Option<bool>,

    /// Whether to enable HTTP2.
    #[serde(default = "true")]
    enable_http2: Option<bool>,

    /// Whether to use the external labels as selectors for the remote read endpoint.
    #[serde(default = "true")]
    filter_external_labels: Option<bool>,
}
