/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/extensions/network/dns_resolver/cares/v3/cares_dns_resolver.proto>
*/

use crate::config::core::{
    address::Address,
    resolver::DNSResolverOptions
};

/// Configuration for c-ares DNS resolver.
pub struct CaresDNSResolverConfig {
    /**
    A list of dns resolver addresses. [`use_resolvers_as_fallback`][Self::use_resolvers_as_fallback] below dictates if the DNS client should override system defaults or only use the provided resolvers if the system defaults are not available, i.e., as a fallback.
    */
    resolvers: Vec<Address>,
    // [(validate.rules).repeated = {min_items: 1}];
  
    /**
    If true use the resolvers listed in the [`resolvers`][Self::resolvers] field only if c-ares is unable to obtain a nameserver from the system (e.g., /etc/resolv.conf).
    Otherwise, the resolvers listed in the resolvers list will override the default system resolvers. Defaults to false.
    */
    use_resolvers_as_fallback: bool,
  
    /**
    The resolver will query available network interfaces and determine if there are no available interfaces for a given IP family. It will then filter these addresses from the results it presents. e.g., if there are no available IPv4 network interfaces, the resolver will not provide IPv4 addresses.
    */
    filter_unroutable_families: bool,
  
    /// Configuration of DNS resolver option flags which control the behavior of the DNS resolver.
    dns_resolver_options: DNSResolverOptions
}
