/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/proxy_protocol.proto>
*/

pub struct ProxyProtocolConfig {
    /// The PROXY protocol version to use. See <https://www.haproxy.org/download/2.1/doc/proxy-protocol.txt> for details
    version: Version
}

pub enum Version {
    /// PROXY protocol version 1. Human readable format.
    V1,

    /// PROXY protocol version 2. Binary format.
    V2
}
