/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/route/v3/route.proto>
*/

type Any = String;
type Struct = String;

use std::collections::HashMap;

use crate::config::{
    core::{
        base::HeaderValueOption,
        config_source::ConfigSource,
    },
    route::route_components::{
        ClusterSpecifierPlugin, RequestMirrorPolicy, VirtualHost
    }
};

pub struct RouteConfiguration {
    /**
    The name of the route configuration. For example, it might match
    [`route_config_name`][crate::extensions::filters::network::http_connection_manager::RDS::route_config_name] in [crate::extensions::filters::network::http_connection_manager::RDS].
    */
    name: String,

    /// An array of virtual hosts that make up the route table.
    virtual_hosts: Vec<VirtualHost>,

    /**
    An array of virtual hosts will be dynamically loaded via the VHDS API.
    Both `virtual_hosts` and `vhds` fields will be used when present. `virtual_hosts` can be used for a base routing table or for infrequently changing virtual hosts. `vhds` is used for on-demand discovery of virtual hosts. The contents of these two fields will be merged to generate a routing table for a given RouteConfiguration, with `vhds` derived configuration taking precedence.
    */
    vhds: VHDS,

    /**
    Optionally specifies a list of HTTP headers that the connection manager will consider to be internal only. If they are found on external requests they will be cleaned prior to filter invocation. See :ref:`config_http_conn_man_headers_x-envoy-internal` for more information.

    [
      (validate.rules).repeated = {items {string {well_known_regex: HTTP_HEADER_NAME strict: false}}}
    ];
    */
    internal_only_headers: Vec<String>,

    /**
    Specifies a list of HTTP headers that should be added to each response that the connection manager encodes. Headers specified at this level are applied after headers from any enclosed [crate::config::route::route_components::VirtualHost] or [crate::config::route::route_components::RouteAction]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    [response_headers_to_add.len() <= 1000]
    */
    response_headers_to_add: Vec<HeaderValueOption>,

    /**
    Specifies a list of HTTP headers that should be removed from each response that the connection manager encodes.

    [
      (validate.rules).repeated = {items {string {well_known_regex: HTTP_HEADER_NAME strict: false}}}
    ];
    */
    response_headers_to_remove: Vec<String>,

    /**
    Specifies a list of HTTP headers that should be added to each request routed by the HTTP connection manager. Headers specified at this level are applied after headers from any enclosed [crate::config::route::route_components::VirtualHost] or [crate::config::route::route_components::RouteAction]. For more information, including details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.

    [request_headers_to_add.len() <= 1000]
    */
    request_headers_to_add: Vec<HeaderValueOption>,

    /**
    Specifies a list of HTTP headers that should be removed from each request routed by the HTTP connection manager.

    [
      (validate.rules).repeated = {items {string {well_known_regex: HTTP_HEADER_NAME strict: false}}}
    ];
    */
    request_headers_to_remove: Vec<String>, 

    /**
    By default, headers that should be added/removed are evaluated from most to least specific:

    - route level
    - virtual host level
    - connection manager level

    To allow setting overrides at the route or virtual host level, this order can be reversed by setting this option to true. Defaults to false.
    */
    most_specific_header_mutations_wins: bool,

    /**
    An optional boolean that specifies whether the clusters that the route table refers to will be validated by the cluster manager. If set to true and a route refers to a non-existent cluster, the route table will not load. If set to false and a route refers to a non-existent cluster, the route table will load and the router filter will return a 404 if the route is selected at runtime. This setting defaults to true if the route table is statically defined via the [`route_config`][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.route_config>` option. This setting default to false if the route table is loaded dynamically via the :ref:`rds <crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager.rds] option. Users may wish to override the default behaviour in certain cases (for example when using CDS with a static route table).
    */
    validate_clusters: bool,

    /**
    The maximum bytes of the response [direct response body][crate::config::route::DirectResponseAction.body] size. If not specified the default is 4096.

    > WARNING: Envoy currently holds the content of [direct response body][crate::config::route::DirectResponseAction.body] in memory. Be careful setting this to be larger than the default 4KB, since the allocated memory for direct response body is not subject to data plane buffering controls.
    */
    max_direct_response_body_size_bytes: u32,

    /**
    A list of plugins and their configurations which may be used by a [cluster specifier plugin name][crate::config::route::route_components::ClusterSpecifier::ClusterSpecifierPlugin] within the route. All `extension.name` fields in this list must be unique.
    */
    cluster_specifier_plugins: Vec<ClusterSpecifierPlugin>,

    /// Specify a set of default request mirroring policies which apply to all routes under its virtual hosts.
    /// Note that policies are not merged, the most specific non-empty one becomes the mirror policies.
    request_mirror_policies: Vec<RequestMirrorPolicy>,

    /**
    By default, port in :authority header (if any) is used in host matching.
    With this option enabled, Envoy will ignore the port number in the :authority header (if any) when picking VirtualHost::
    NOTE: this option will not strip the port number (if any) contained in route config [crate::config::route::route_components::VirtualHost].domains field.
    */
    ignore_port_in_host_matching: bool,

    /**
    Ignore path-parameters in path-matching.
    Before RFC3986, URI were like(RFC1808): `<scheme>://<net_loc>/<path>;<params>?<query>#<fragment>`
    Envoy by default takes ":path" as `"<path>;<params>"`.
    For users who want to only match path on the `"<path>"` portion, this option should be true.
    */
    ignore_path_parameters_in_path_matching: bool,

    /**
    The typed_per_filter_config field can be used to provide RouteConfiguration level per filter config.
    The key should match the [filter config name][crate::extensions::filters::network::http_connection_manager::HTTPFilter.name].
    The canonical filter name (e.g., `envoy.filters.http.buffer` for the HTTP buffer filter) can also be used for the backwards compatibility. If there is no entry referred by the filter config name, the entry referred by the canonical filter name will be provided to the filters as fallback.

    Use of this field is filter specific; see the :ref:`HTTP filter documentation <config_http_filters>` for if and how it is utilized.
    
    > #comment: An entry's value may be wrapped in a [`FilterConfig`][crate::config::route::route_components::FilterConfig] message to specify additional options.]
    */
    typed_per_filter_config: HashMap<String, Any>,
}

pub struct VHDS {
    /// Configuration source specifier for VHDS.
    // [(validate.rules).message = {required: true}];
    config_source: ConfigSource
}
