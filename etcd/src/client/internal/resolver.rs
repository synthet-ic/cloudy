/*!
<https://github.com/etcd-io/etcd/tree/main/client/v3/internal/resolver>
*/

use grpc::{
    resolver::manual::Resolver,
    service_config::ParseResult
};

/**
EtcdManualResolver is a Resolver (and resolver.Builder) that can be updated
using SetEndpoints.
*/
pub struct EtcdManualResolver {
    // Option<Resolver>,
    endpoints: Vec<String>,
    service_config: Option<ParseResult>
}
