/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/resolver.proto>
*/

use super::address::Address;

/// DNS resolution configuration which includes the underlying dns resolver addresses and options.
pub struct DNSResolutionConfig {
    /**
    A list of dns resolver addresses. If specified, the DNS client library will perform resolution
    via the underlying DNS resolvers. Otherwise, the default system resolvers
    (e.g., /etc/resolv.conf) will be used.

    [(validate.rules).repeated = {min_items: 1}]
    */
    resolvers: Vec<Address>,

    /// Configuration of DNS resolver option flags which control the behaviour of the DNS resolver.
    dns_resolver_options: DNSResolverOptions
}

/// Configuration of DNS resolver option flags which control the behaviour of the DNS resolver.
pub struct DNSResolverOptions {
    /// Use TCP for all DNS queries instead of the default protocol UDP.
    use_tcp_for_dns_lookups: bool,
  
    /// Do not use the default search domains; only query hostnames as-is or as aliases.
    no_default_search_domain: bool
}
