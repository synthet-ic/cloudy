/*!
<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#oauth2>
*/

use std::{
    collections::HashMap,
    path::PathBuf
};

use serde::{Serialize, Deserialize};

use crate::tls_config::TLSConfig;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2 {
    client_id: String,
    client_secret: Option<secret>,

    /// Read the client secret from a file.
    /// It is mutually exclusive with `client_secret`.
    client_secret_file: Option<PathBuf>,

    /// Scopes for the token request.
    scopes: Option<Vec<String>>,

    /// The URL to fetch the token from.
    token_url: String,

    /// Optional parameters to append to the token URL.
    endpoint_params: Option<HashMap<String, String>>,

    /// Configures the token request's TLS settings.
    tls_config: Option<TLSConfig>,

    /// Optional proxy URL.
    proxy_url: Option<String>
}
