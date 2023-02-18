/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/address.proto>
*/

use super::socket_option::{SocketOption, SocketOptionsOverride};

/**
Addresses specify either a logical or physical address and port, which are used to tell Envoy where to bind/listen, connect to upstream and find management servers.
*/
pub enum Address {
    // option (validate.required) = true;
    SocketAddress(SocketAddress),
    Pipe(Pipe),
    /// Specifies a user-space address handled by [internal listeners][crate::config::listener::listener::ListenerSpecifier::InternalListener].
    EnvoyInternalAddress(EnvoyInternalAddress)
}

pub struct SocketAddress {
    /// (validate.rules).enum = {defined_only: true}
    protocol: Protocol,
  
    /**
    The address for this socket. :ref:`Listeners <config_listeners>` will bind to the address. An empty address is not allowed. Specify `0.0.0.0` or `::` to bind to any address. [#comment:TODO(zuercher) reinstate when implemented:
    It is possible to distinguish a Listener address via the prefix/suffix matching in [`FilterChainMatch`][crate::config::listener::listener_components::FilterChainMatch>`.] When used within an upstream [`BindConfig`], the address controls the source address of outbound connections. For [`clusters`][crate::config::cluster::cluster::Cluster], the cluster type determines whether the address must be an IP (`STATIC` or `EDS` clusters) or a hostname resolved by DNS (`STRICT_DNS` or `LOGICAL_DNS` clusters). Address resolution can be customised via :ref:`resolver_name <crate::config::core::SocketAddress.resolver_name].

    [(validate.rules).string = {min_len: 1}]
    */
    address: String,
  
    port_specifier: PortSpecifier,
  
    /**
    The name of the custom resolver. This must have been registered with Envoy. If this is empty, a context dependent default applies. If the address is a concrete IP address, no resolution will occur. If address is a hostname this should be set for resolution other than DNS. Specifying a custom resolver with `STRICT_DNS` or `LOGICAL_DNS` will generate an error at runtime.
    */
    resolver_name: String,
  
    /**
    When binding to an IPv6 address above, this enables [IPv4 compatibility](https://www.rfc-editor.org/rfc/rfc3493#page-11). Binding to `::` will allow both IPv4 and IPv6 connections, with peer IPv4 addresses mapped into
    IPv6 space as `::FFFF:<IPv4-address>`.
    */
    ipv4_compat: bool
}

pub enum Protocol {
    TCP,
    UDP
}

pub struct Pipe {
    /**
    Unix Domain Socket path. On Linux, paths starting with '@' will use the abstract namespace. The starting '@' is replaced by a null byte by Envoy.
    Paths starting with '@' will result in an error in environments other than Linux.
    */
    // [!is_empty()]
    path: String,

    /// The mode for the Pipe. Not applicable for abstract sockets.
    // !{lte: 511}
    mode: u16
}

pub struct BindConfig {
    /// The address to bind to when creating a socket.
    /// (validate.rules).message = {required: true}
    source_address: SocketAddress,
  
    /**
    Whether to set the `IP_FREEBIND` option when creating the socket. When this flag is set to true, allows the [`source_address`][Self::source_address] to be an IP address that is not configured on the system running Envoy. When this flag is set to `false`, the option `IP_FREEBIND` is disabled on the socket. When this flag is not set (default), the socket is not modified, i.e. the option is neither enabled nor disabled.
    */
    free_bind: bool,
  
    /// Additional socket options that may not be present in Envoy source code or precompiled binaries.
    socket_options: Vec<SocketOption>,
  
    /**
    Extra source addresses appended to the address specified in the `source_address` field. This enables to specify multiple source addresses. Currently, only one extra address can be supported, and the extra address should have a different IP version with the address in the `source_address` field. The address which has the same IP version with the target host's address IP version will be used as bind address. If more than one extra address specified, only the first address matched IP version will be returned. If there is no same IP version address found, the address in the `source_address` will be returned.
    */
    extra_source_addresses: Vec<ExtraSourceAddress>,
}

pub enum PortSpecifier {
    // option (validate.required) = true;

    PortValue(u16),

    /*
    This is only valid if [`resolver_name`]][crate::config::core::SocketAddress::resolver_name] is specified below and the named resolver is capable of named port resolution.
    */
    NamedPort(String)
}

pub struct TCPKeepalive {
    /**
    Maximum number of keepalive probes to send without response before deciding the connection is dead. Default is to use the OS level configuration (unless overridden, Linux defaults to `9`.)
    */
    keepalive_probes: u32,
  
    /**
    The number of seconds a connection needs to be idle before keep-alive probes start being sent. Default is to use the OS level configuration (unless overridden, Linux defaults to 7200s (i.e., 2 hours.)
    */
    keepalive_time: u32,
  
    /**
    The number of seconds between keep-alive probes. Default is to use the OS level configuration (unless overridden, Linux defaults to 75s.)
    */
    keepalive_interval: u32,
}

pub struct ExtraSourceAddress {
    /// The additional address to bind.
    /// (validate.rules).message = {required: true}
    address: SocketAddress,
  
    /**
    Additional socket options that may not be present in Envoy source code or precompiled binaries. If specified, this will override the [`socket_options`][BindConfig::socket_options] in the BindConfig. If specified with no [`socket_options`][SocketOptionsOverride::socket_options] or an empty list of [`socket_options`][SocketOptionsOverride::socket_options], it means no socket option will apply.
    */
    socket_options: SocketOptionsOverride
}

/// The address represents an envoy internal listener.
/// [#comment: TODO(asraa): When address available, remove workaround from test/server/server_fuzz_test.cc:30.]
pub struct EnvoyInternalAddress {
    address_name_specifier: AddressNameSpecifier,
  
    /**
    Specifies an endpoint identifier to distinguish between multiple endpoints for the same internal listener in a single upstream pool. Only used in the upstream addresses for tracking changes to individual endpoints. This, for example, may be set to the final destination IP for the target internal listener.
    */
    endpoint_id: String
}

pub enum AddressNameSpecifier {
    // option (validate.required) = true;

    /**
    Specifies the [`name`][crate::config::listener::listener::Listener::name] of the internal listener.
    */
    ServerListenerName(String)
}

/**
CidrRange specifies an IP Address and a prefix length to construct the subnet mask for a [CIDR](https://www.rfc-editor.org/rfc/rfc4632) range.
*/
pub struct CIDRRange {
    /// IPv4 or IPv6 address, e.g. `192.0.0.0` or `2001:db8::`.
    // [!is_empty()];
    address_prefix: String,
  
    /// Length of prefix, e.g. 0, 32. Defaults to 0 when unset.
    // [(validate.rules).u32 = {lte: 128}];
    prefix_len: u8
}
