/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/service/route/v3/rds.proto>
*/

use crate::service::discovery::{
    DeltaDiscoveryRequest, DeltaDiscoveryResponse, DiscoveryRequest, DiscoveryResponse
};

/**
The `resource_names` field in `DiscoveryRequest` specifies a route configuration.
This allows an Envoy configuration with multiple HTTP listeners (and associated HTTP connection manager filters) to use different route configurations. Each listener will bind its HTTP connection manager filter to a route table via this identifier.
*/
pub trait RouteDiscovery {
    // option (envoy.annotations.resource).type = "envoy.config.route::RouteConfiguration";

    fn stream_routes(r: /*stream*/ DiscoveryRequest) -> /*stream*/ DiscoveryResponse;

    fn delta_routes(r: /*stream*/ DeltaDiscoveryRequest) -> /*stream*/ DeltaDiscoveryResponse;

    fn fetch_routes(r: DiscoveryRequest) -> DiscoveryResponse;
        // option (google.api.http).post = "/v3/discovery:routes";
        // option (google.api.http).body = "*";
}

/**
Virtual Host Discovery Service (VHDS) is used to dynamically update the list of virtual hosts for a given `RouteConfiguration`. If VHDS is configured a virtual host list update will be triggered during the processing of an HTTP request if a route for the request cannot be resolved. The [`resource_names_subscribe`][crate::service::discovery::DeltaDiscoveryRequest::resource_names_subscribe] field contains a list of virtual host names or aliases to track. The contents of an alias would be the contents of a `host` or `authority` header used to make an http request. An xDS server will match an alias to a virtual host based on the content of [domains][crate::config::route::route_components::VirtualHost::domains] field. The `resource_names_unsubscribe` field contains a list of virtual host names that have been :ref:`unsubscribed <xds_protocol_unsubscribe] from the routing table associated with the RouteConfiguration.
*/
pub trait VirtualHostDiscovery {
    // option (envoy.annotations.resource).type = "envoy.config.route::route_components::VirtualHost";
  
    fn delta_virtual_hosts(r: /*stream*/ DeltaDiscoveryRequest)
        -> /*stream*/ DeltaDiscoveryResponse;
}
  
/**
Not configuration. Workaround C++ protobuf issue with importing services: <https://github.com/google/protobuf/issues/4221> and protoxform to upgrade the file.
*/
pub struct RDSDummy {
}
