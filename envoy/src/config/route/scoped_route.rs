/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/route/v3/scoped_route.proto>
*/

use crate::config::route::route::RouteConfiguration;

/**
Specifies a routing scope, which associates a [`Key`] to a [`crate::config::route::route::RouteConfiguration`].
The [`crate::config::route::route::RouteConfiguration`] can be obtained dynamically via RDS ([`route_configuration_name`][ScopedRouteConfiguration::route_configuration_name]) or specified inline ([`route_configuration`][ScopedRouteConfiguration::route_configuration]).

The HTTP connection manager builds up a table consisting of these Key to
`RouteConfiguration` mappings, and looks up the `RouteConfiguration` to use per
request according to the algorithm specified in the
[`scope_key_builder`][crate::extensions::filters::network::http_connection_manager::ScopedRoutes::scope_key_builder] assigned to the `HTTPConnectionManager`.

For example, with the following configurations (in YAML):

HTTPConnectionManager config:

```yaml
  ...
  scoped_routes:
    name: foo-scoped-routes
    scope_key_builder:
      fragments:
        - header_value_extractor:
            name: X-Route-Selector
            element_separator: ,
            element:
              separator: =
              key: vip
```

`ScopedRouteConfiguration` resources (specified statically via
[`ScopedRouteConfigurationsList`][crate::extensions::filters::network::http_connection_manager::ScopedRouteConfigurationsList] or obtained dynamically via SRDS):

.. code::

 (1)
  name: route-scope1
  route_configuration_name: route-config1
  key:
     fragments:
       - string_key: 172.10.10.20

 (2)
  name: route-scope2
  route_configuration_name: route-config2
  key:
    fragments:
      - string_key: 172.20.20.30

A request from a client such as:

.. code::

    GET / HTTP/1.1
    Host: foo.com
    X-Route-Selector: vip=172.10.10.20

would result in the routing table defined by the `route-config1`
`RouteConfiguration` being assigned to the HTTP request/stream.

*/
pub struct ScopedRouteConfiguration {
    /// Whether the RouteConfiguration should be loaded on demand.
    on_demand: bool,

    /// The name assigned to the routing scope.
    ///  [(validate.rules).string = {min_len: 1}];
    name: String,

    /**
    The resource name to use for a :ref:`crate::service.discovery::DiscoveryRequest` to an
    RDS server to fetch the [crate::config::route::route::RouteConfiguration] associated with this scope.

    [(udpa.annotations.field_migrate).oneof_promotion = "route_config"];
    */
    route_configuration_name: String,

    /// The [crate::config::route::route::RouteConfiguration] associated with the scope.
    /// [(udpa.annotations.field_migrate).oneof_promotion = "route_config"];
    route_configuration: RouteConfiguration,

    /// The key to match against.
    // [(validate.rules).message = {required: true}];
    key: Key
}

/**
Specifies a key which is matched against the output of the
[`scope_key_builder`][crate::extensions::filters::network::http_connection_manager::ScopedRoutes::scope_key_builder] specified in the HTTPConnectionManager. The matching is done per HTTP request and is dependent on the order of the fragments contained in the Key.
*/
pub struct Key {
    /**
    The ordered set of fragments to match against. The order must match the fragments in the corresponding [`scope_key_builder][crate::extensions::filters::network::http_connection_manager::ScopedRoutes::scope_key_builder].

    [(validate.rules).repeated = {min_items: 1}];
    */
    fragments: Vec<Fragment>
}

pub enum Fragment {
    // option (validate.required) = true;

    /// A string to match against.
    StringKey(String)
}
