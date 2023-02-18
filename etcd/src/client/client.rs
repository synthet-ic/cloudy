/*!
<https://github.com/etcd-io/etcd/blob/main/client/v3/client.go>
*/

use std::error::Error;
use std::sync::RwLock;

use grpc::{
    client_connection::ClientConnection,
    credentials::TransportCredentials
};

use crate::client::{
    cluster::Cluster,
    config::Config,
    internal::resolver::EtcdManualResolver,
    kv::KV
};

/// Client provides and manages an etcd v3 client session.
pub struct Client {
    // Cluster,
    // KV,
    // Lease,
    // Watcher,
    // Auth,
    // Maintenance,

    connection: Option<ClientConnection>,

    config: Config,
    credentials: TransportCredentials,
    resolver: Option<EtcdManualResolver>,

    ep_mu: Option<RwLock>,
    endpoints: Vec<String>,

    context: Context,
    cancel: context::CancelFunc,

    /// `username` is a user name for authentication.
    username: String,
    /// `password` is a password for authentication.
    password: String,
    auth_token_bundle: credentials::Bundle,

    call_opts: Option<grpc::CallOption>,

    lg_mu: Option<RwLock>,
    lg: Option<zap::Logger>
}

impl Client {
    /// Creates a new etcdv3 client from a given configuration.
    pub fn new(config: Config) -> Result<Client, dyn Error> {
        if config.endpoints.len() == 0 {
            Err(ErrNoAvailableEndpoints)
        } else {
            newClient(&config)
        }
    }
}
