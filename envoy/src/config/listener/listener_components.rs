/*!
- <https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/listener/v3/listener_components.proto>
- <https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/listener/v3/listener_components.proto>
*/

type Any = String;
type Struct = String;

use std::time::Duration;

use crate::{
    config::core::{
        address::CIDRRange,
        base::{Metadata, TransportSocket},
        config_source::ExtensionConfigSource 
    },
    types::range::I32Range
};

pub struct Filter {
    /// The name of the filter configuration.
    /// [!name.is_empty()]
    name: String,

    config_type: ConfigType
}

pub enum ConfigType {
    /**
    Filter specific configuration which depends on the filter being instantiated. See the supported filters for further documentation.
    [#extension-category: envoy.filters.network]
    */
    TypedConfig(Any),

    /**
    Configuration source specifier for an extension configuration discovery service. In case of a failure and without the default configuration, the listener closes the connections.
    */
    ConfigDiscovery(ExtensionConfigSource)
}

/**
Specifies the match criteria for selecting a specific filter chain for a
listener.

In order for a filter chain to be selected, *ALL* of its criteria must be
fulfilled by the incoming connection, properties of which are set by the
networking stack and/or listener filters.

The following order applies:

1. Destination port.
2. Destination IP address.
3. Server name (e.g. SNI for TLS protocol),
4. Transport protocol.
5. Application protocols (e.g. ALPN for TLS protocol).
6. Directly connected source IP address (this will only be different from the source IP address when using a listener filter that overrides the source address, such as the :ref:`Proxy Protocol listener filter <config_listener_filters_proxy_protocol>`).
7. Source type (e.g. any, local or external network).
8. Source IP address.
9. Source port.

For criteria that allow ranges or wildcards, the most specific value in any of the configured filter chains that matches the incoming connection is going to be used (e.g. for SNI `www.example.com` the most specific match would be `www.example.com`, then `*.example.com`, then `*.com`, then any filter chain without `server_names` requirements).

A different way to reason about the filter chain matches:
Suppose there exists N filter chains. Prune the filter chain set using the above 8 steps.
In each step, filter chains which most specifically matches the attributes continue to the next step.
The listener guarantees at most 1 filter chain is left after all of the steps.

Example:

For destination port, filter chains specifying the destination port of incoming traffic are the most specific match. If none of the filter chains specifies the exact destination port, the filter chains which do not specify ports are the most specific match. Filter chains specifying the wrong port can never be the most specific match.

[#comment: Implemented rules are kept in the preference order, with deprecated fields listed at the end, because that's how we want to list them in the docs.

[#comment:TODO(PiotrSikora): Add support for configurable precedence of the rules]
*/
pub struct FilterChainMatch {
    /// Optional destination port to consider when use_original_dst is set on the listener in determining a filter chain match.
    // [destination_port > 0]
    destination_port: u16,

    /// If non-empty, an IP address and prefix length to match addresses when the  listener is bound to 0.0.0.0/:: or when use_original_dst is specified.
    prefix_ranges: Vec<CIDRRange>,

    /// If non-empty, an IP address and suffix length to match addresses when the listener is bound to 0.0.0.0/:: or when use_original_dst is specified.
    address_suffix: String,

    /// 
    suffix_len: u32,

    /// The criteria is satisfied if the directly connected source IP address of the downstream connection is contained in at least one of the specified subnets. If the parameter is not specified or the list is empty, the directly connected source IP address is ignored.
    direct_source_prefix_ranges: Vec<CIDRRange>,

    /// Specifies the connection source IP match type. Can be any, local or external network.
    // [(validate.rules).enum = {defined_only: true}]
    source_type: ConnectionSourceType,

    /// The criteria is satisfied if the source IP address of the downstream connection is contained in at least one of the specified subnets. If the  parameter is not specified or the list is empty, the source IP address is  ignored.
    source_prefix_ranges: Vec<CIDRRange>,

    /**
    The criteria is satisfied if the source port of the downstream connection is contained in at least one of the specified ports. If the parameter is not specified, the source port is ignored.
    [(validate.rules).repeated = {items {u32 {lte: 65535 gte: 1}}}]
    */
    source_ports: Vec<u32>,

    /**
    If non-empty, a list of server names (e.g. SNI for TLS protocol) to consider when determining a filter chain match. Those values will be compared against the server names of a new connection, when detected by one of the listener filters.

    The server name will be matched against all wildcard domains, i.e. `www.example.com` will be first matched against `www.example.com`, then `*.example.com`, then `*.com`.

    Note that partial wildcards are not supported, and values like `*w.example.com` are invalid.
    The value `*` is also not supported, and `server_names` should be omitted instead.

    > ATTENTION: See the :ref:`FAQ entry <faq_how_to_setup_sni>` on how to configure SNI for more information.
    */
    server_names: Vec<String>,

    /**
    If non-empty, a transport protocol to consider when determining a filter chain match.
    This value will be compared against the transport protocol of a new connection, when it's detected by one of the listener filters.

    Suggested values include:

    - `raw_buffer` - default, used when no transport protocol is detected,
    - `tls` - set by :ref:`envoy.filters.listener.tls_inspector <config_listener_filters_tls_inspector>` when TLS protocol is detected.
    */
    transport_protocol: String,

    /**
    If non-empty, a list of application protocols (e.g. ALPN for TLS protocol) to consider when determining a filter chain match. Those values will be compared against the application protocols of a new connection, when detected by one of the listener filters.

    Suggested values include:

    - `http/1.1` - set by :ref:`envoy.filters.listener.tls_inspector
      <config_listener_filters_tls_inspector>`,
    - `h2` - set by :ref:`envoy.filters.listener.tls_inspector <config_listener_filters_tls_inspector>`

    > ATTENTION: Currently, only :ref:`TLS Inspector <config_listener_filters_tls_inspector>` provides application protocol detection based on the requested [ALPN](https://en.wikipedia.org/wiki/Application-Layer_Protocol_Negotiation)_ values.
    >
    > However, the use of ALPN is pretty much limited to the HTTP/2 traffic on the Internet, and matching on values other than `h2` is going to lead to a lot of false negatives, unless all connecting clients are known to use ALPN.
    */
    application_protocols: Vec<String>
}

pub enum ConnectionSourceType {
    /// Any connection source matches.
    Any,

    /// Match a connection originating from the same host.
    SameIPOrLoopback,

    /// Match a connection originating from a different host.
    External
}

/// A filter chain wraps a set of match criteria, an option TLS context, a set of filters, and various other parameters.
pub struct FilterChain {
    /// The criteria to use when matching a connection to this filter chain.
    filter_chain_match: FilterChainMatch,

    /**
    A list of individual network filters that make up the filter chain for connections established with the listener. Order matters as the filters are processed sequentially as connection events happen. Note: If the filter list is empty, the connection will close by default.

    For QUIC listeners, network filters other than HTTP Connection Manager (HCM) can be created, but due to differences in the connection implementation compared to TCP, the onData() method will never be called. Therefore, network filters for QUIC listeners should only expect to do work at the start of a new connection (i.e. in onNewConnection()). HCM must be the last (or only) filter in the chain.
    */
    filters: Vec<Filter>,

    /// [#not-implemented-hide:] Filter chain metadata.
    metadata: Metadata,

    /**
    Optional custom transport socket implementation to use for downstream connections.
    To setup TLS, set a transport socket with name `envoy.transport_sockets.tls` and [`DownstreamTLSContext`][crate::extensions::transport_sockets::tls::tls::DownstreamTLSContext] in the `typed_config`.
    If no transport socket configuration is specified, new connections
    will be set up with plaintext.
    [#extension-category: envoy.transport_sockets.downstream]
    */
    transport_socket: TransportSocket,

    /**
    If present and nonzero, the amount of time to allow incoming connections to complete any transport socket negotiations. If this expires before the transport reports connection establishment, the connection is summarily closed.
    */
    transport_socket_connect_timeout: Duration,

    /**
    The unique name (or empty) by which this filter chain is known.
    Note: [`filter_chain_matcher`][crate::config::listener::listener::Listener::filter_chain_matcher] requires that filter chains are uniquely named within a listener.
    */
    name: String,

    /**
    [#not-implemented-hide:]
    The configuration to specify whether the filter chain will be built on-demand.
    If this field is not empty, the filter chain will be built on-demand.
    Otherwise, the filter chain will be built normally and block listener warming.
    */
    on_demand_configuration: OnDemandConfiguration
}

/**
The configuration for on-demand filter chain. If this field is not empty in FilterChain message, a filter chain will be built on-demand.
On-demand filter chains help speedup the warming up of listeners since the building and initialisation of an on-demand filter chain will be postponed to the arrival of new connection requests that require this filter chain.
Filter chains that are not often used can be set as on-demand.
*/
pub struct OnDemandConfiguration {
    /**
    The timeout to wait for filter chain placeholders to complete rebuilding.
    1. If this field is set to 0, timeout is disabled.
    2. If not specified, a default timeout of 15s is used.
    Rebuilding will wait until dependencies are ready, have failed, or this timeout is reached.
    Upon failure or timeout, all connections related to this filter chain will be closed.
    Rebuilding will start again on the next new connection.
    */
    rebuild_timeout: Duration
}

/**
Listener filter chain match configuration. This is a recursive structure which allows complex nested match configurations to be built using various logical operators.

Examples:

- Matches if the destination port is 3306.

```yaml
destination-port-range:
  start: 3306
  end: 3307
```

- Matches if the destination port is 3306 or 15000.

```yaml
or_match:
  rules:
  - destination_port_range:
      start: 3306
      end: 3307
  - destination_port_range:
      start: 15000
      end: 15001
```
*/
pub struct ListenerFilterChainMatchPredicate {
    rule: Box<Rule>
}

/// A set of match configurations used for logical operations.
pub struct MatchSet {
    /// The list of rules that make up the set.
    /// [rules.len() >= 2]
    rules: Vec<ListenerFilterChainMatchPredicate>
}

pub enum Rule {
    // option (validate.required) = true;

    /// A set that describes a logical OR. If any member of the set matches, the match configuration matches.
    OrMatch(MatchSet),

    /// A set that describes a logical AND. If all members of the set match, the match configuration matches.
    AndMatch(MatchSet),

    /// A negation match. The match configuration will match if the negated match condition matches.
    NotMatch(ListenerFilterChainMatchPredicate),

    /// The match configuration will always match.
    // [(validate.rules).bool = {const: true}]
    AnyMatch(bool),

    /// Match destination port. Particularly, the match evaluation must use the recovered local port if the owning listener filter is after :ref:`an original_dst listener filter <config_listener_filters_original_dst>`.
    DestinationPortRange(I32Range),
}

// [#next-free-field: 6]
pub struct ListenerFilter {
    /// The name of the filter configuration.
    /// [!name.is_empty()]
    name: String,

    /*
    Optional match predicate used to disable the filter. The filter is enabled when this field is empty.
    See [`ListenerFilterChainMatchPredicate`][crate::config::listener::ListenerFilterChainMatchPredicate] for further examples.
    */
    filter_disabled: ListenerFilterChainMatchPredicate
}
