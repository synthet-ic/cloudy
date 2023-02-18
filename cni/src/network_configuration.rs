use std::net::IpAddr as IPAddress;

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

/// <https://www.cni.dev/docs/spec/#configuration-format>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Configuration {
    /// [Semantic Version 2.0](https://semver.org/) of CNI specification to which this configuration list and all the individual configurations conform. Currently “1.0.0”
    cni_version: String,
    /// Network name. This should be unique across all network configurations on a host (or other administrative domain). Must start with an alphanumeric character, optionally followed by any combination of one or more alphanumeric characters, underscore, dot (.) or hyphen (-).
    name: String,
    /// Either `true` or `false`. If `disable_check` is `true`, runtimes must not call `CHECK` for this network configuration list. This allows an administrator to prevent `CHECK`ing where a combination of plugins is known to return spurious errors.
    disable_check: bool,
    /// A list of CNI plugins and their configuration, which is a list of plugin configuration objects.
    plugins: Vec<PluginConfiguration>,
}

/// <https://www.cni.dev/docs/spec/#plugin-configuration-objects>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct PluginConfiguration {
    /// Matches the name of the CNI plugin binary on disc. Must not contain characters disallowed in file paths for the system (e.g. / or \).
    r#type: String,
    /// Used by the protocol.
    capabilities: Option<Capabilities>,
    // These keys are not used by the protocol, but have a standard meaning to plugins. Plugins that consume any of these configuration keys should respect their intended semantics.
    /// If supported by the plugin, sets up an IP masquerade on the host for this network. This is necessary if the host will act as a gateway to subnets that are not able to route to the IP assigned to the container.
    #[serde(rename(serialize = "ipMasq"))]
    ip_masquerade: Option<bool>,
    /// Dictionary with IPAM (IP Address Management) specific values.
    ipam: Option<IPAM>,
    /// Dictionary with DNS specific values.
    dns: Option<DNS>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPAM {
    /// Refers to the filename of the IPAM plugin executable. Must not contain characters disallowed in file paths for the system (e.g. / or \).
    r#type: String
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct DNS {
    /// List of a priority-ordered list of DNS name servers that this network is aware of. Each entry in the list is a string containing either an IPv4 or an IPv6 address.
    #[serde(rename(serialize = "nameservers"))]
    name_servers: Option<Vec<IPAddress>>,
    /// The local domain used for short hostname lookups.
    domain: Option<String>,
    /// list of priority ordered search domains for short hostname lookups. Will be preferred over [`domain`][Self::domain] by most resolvers.
    search: Option<Vec<String>>,
    /// List of options that can be passed to the resolver.
    options: Option<Vec<String>>
}
