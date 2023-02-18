/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/service/discovery/v3/discovery.proto>
*/

type Any = String;
type Status = String;

use std::{
    collections::HashMap,
    time::Duration
};

use crate::config::core::base::{ControlPlane, Metadata, Node};

/// A DiscoveryRequest requests a set of versioned resources of the same type for a given Envoy node on some API.
pub struct DiscoveryRequest {
    /**
    The `version_info` provided in the request messages will be the version_info received with the most recent successfully processed response or empty on the first request. It is expected that no new request is sent after a response is received until the Envoy instance is ready to ACK/NACK the new configuration. ACK/NACK takes place by returning the new API config version as applied or the previous API config version respectively. Each type_url (see below) has an independent version associated with it.
    */
    version_info: String,

    /// The node making the request.
    node: Node,

    /**
    List of resources to subscribe to, e.g. list of cluster names or a route configuration name. If this is empty, all resources for the API are returned. LDS/CDS may have empty resource_names, which will cause all resources for the Envoy instance to be returned. The LDS and CDS responses will then imply a number of resources that need to be fetched via EDS/RDS, which will be explicitly enumerated in resource_names.
    */
    resource_names: Vec<String>,

    /**
    Alternative to `resource_names` field that allows specifying dynamic parameters along with each resource name. Clients that populate this field must be able to handle responses from the server where resources are wrapped in a Resource message.
    Note that it is legal for a request to have some resources listed in `resource_names` and others in `resource_locators`.
    */
    resource_locators: Vec<ResourceLocator>,

    /**
    Type of the resource that is being requested, e.g. "type.googleapis.com/envoy.api.v2.ClusterLoadAssignment". This is implicit in requests made via singleton xDS APIs such as CDS, LDS, etc. but is required for ADS.
    */
    type_url: String,

    /**
    nonce corresponding to DiscoveryResponse being ACK/NACKed. See above discussion on version_info and the DiscoveryResponse nonce comment. This may be empty only if 1) this is a non-persistent-stream xDS such as HTTP, or 2) the client has not yet accepted an update in this xDS stream (unlike delta, where it is populated only for new explicit ACKs).
    */
    response_nonce: String,

    /**
    This is populated when the previous [`DiscoveryResponse`][crate::service.discovery::DiscoveryResponse] failed to update configuration. The `message` field in `error_details` provides the Envoy internal exception related to the failure. It is only intended for consumption during manual debugging, the string provided is not guaranteed to be stable across Envoy versions.
    */
    error_detail: Status
}

pub struct DiscoveryResponse {
    /// The version of the response data.
    version_info: String,

    /// The response resources. These resources are typed and depend on the API being called.
    resources: Vec<Any>,

    /*
    Canary is used to support two Envoy command line flags:

    - `--terminate-on-canary-transition-failure`. When set, Envoy is able to terminate if it detects that configuration is stuck at canary. Consider this example sequence of updates:
      - Management server applies a canary config successfully.
      - Management server rolls back to a production config.
      - Envoy rejects the new production config.
      Since there is no sensible way to continue receiving configuration updates, Envoy will then terminate and apply production config from a clean slate.
    - `--dry-run-canary`. When set, a canary response will never be applied, only validated via a dry run.
    */
    canary: bool,

    /*
    Type URL for resources. Identifies the xDS API when muxing over ADS.
    Must be consistent with the type_url in the 'resources' repeated Any (if non-empty).
    */
    type_url: String,

    /**
    For gRPC based subscriptions, the nonce provides a way to explicitly ack a specific DiscoveryResponse in a following DiscoveryRequest. Additional messages may have been sent by Envoy to the management server for the previous version on the stream prior to this `DiscoveryResponse`, that were unprocessed at response send time. The nonce allows the management server to ignore any further DiscoveryRequests for the previous version until a `DiscoveryRequest` bearing the nonce. The nonce is optional and is not required for non-stream based xDS implementations.
    */
    nonce: String,

    /// The control plane instance that sent the response.
    control_plane: ControlPlane
}

/**
DeltaDiscoveryRequest and DeltaDiscoveryResponse are used in a new gRPC endpoint for Delta xDS.

With Delta xDS, the DeltaDiscoveryResponses do not need to include a full snapshot of the tracked resources. Instead, DeltaDiscoveryResponses are a diff to the state of a xDS client.
In Delta XDS there are per-resource versions, which allow tracking state at the resource granularity.
An xDS Delta session is always in the context of a gRPC bidirectional stream. This allows the xDS server to keep track of the state of xDS clients connected to it.

In Delta xDS the nonce field is required and used to pair DeltaDiscoveryResponse to a DeltaDiscoveryRequest ACK or NACK.
Optionally, a response message level system_version_info is present for debugging purposes only.

DeltaDiscoveryRequest plays two independent roles. Any DeltaDiscoveryRequest can be either or both of: (1) informing the server of what resources the client has gained/lost interest in (using resource_names_subscribe and resource_names_unsubscribe), or (2) (N)ACKing an earlier resource update from the server (using response_nonce, with presence of error_detail making it a NACK).
Additionally, the first message (for a given type_url) of a reconnected gRPC stream has a third role: informing the server of the resources (and their versions) that the client already possesses, using the initial_resource_versions field.

As with state-of-the-world, when multiple resource types are multiplexed (ADS), all requests/acknowledgments/updates are logically walled off by type_url: a Cluster ACK exists in a completely separate world from a prior Route NACK.
In particular, initial_resource_versions being sent at the "start" of every gRPC stream actually entails a message for each type_url, each with its own initial_resource_versions.
*/
pub struct DeltaDiscoveryRequest {
    /// The node making the request.
    node: Node,

    /*
    Type of the resource that is being requested, e.g. `type.googleapis.com/envoy.api.v2.ClusterLoadAssignment`. This does not need to be set if resources are only referenced via `xds_resource_subscribe` and `xds_resources_unsubscribe`.
    */
    type_url: String,

    /*
    DeltaDiscoveryRequests allow the client to add or remove individual resources to the set of tracked resources in the context of a stream.
    All resource names in the resource_names_subscribe list are added to the set of tracked resources and all resource names in the `resource_names_unsubscribe` list are removed from the set of tracked resources.

    *Unlike* state-of-the-world xDS, an empty `resource_names_subscribe` or `resource_names_unsubscribe` list simply means that no resources are to be added or removed to the resource list.
    *Like* state-of-the-world xDS, the server must send updates for all tracked resources, but can also send updates for resources the client has not subscribed to.

    NOTE: the server must respond with all resources listed in `resource_names_subscribe`, even if it believes the client has the most recent version of them. The reason: the client may have dropped them, but then regained interest before it had a chance to send the unsubscribe message. See DeltaSubscriptionStateTest.RemoveThenAdd.

    These two fields can be set in any DeltaDiscoveryRequest, including ACKs and initial_resource_versions.

    A list of Resource names to add to the list of tracked resources.
    */
    resource_names_subscribe: Vec<String>,

    /// A list of Resource names to remove from the list of tracked resources.
    resource_names_unsubscribe: Vec<String>,

    /**
    Alternative to `resource_names_subscribe` field that allows specifying dynamic parameters along with each resource name.
    Note that it is legal for a request to have some resources listed in `resource_names_subscribe` and others in `resource_locators_subscribe`.
    */
    resource_locators_subscribe: Vec<ResourceLocator>,

    /**
    Alternative to `resource_names_unsubscribe` field that allows specifying dynamic parameters along with each resource name.
    Note that it is legal for a request to have some resources listed in `resource_names_unsubscribe` and others in `resource_locators_unsubscribe`.
    */
    resource_locators_unsubscribe: Vec<ResourceLocator>,

    /**
    Informs the server of the versions of the resources the xDS client knows of, to enable the client to continue the same logical xDS session even in the face of gRPC stream reconnection.
    It will not be populated: (1) in the very first stream of a session, since the client will not yet have any resources, (2) in any message after the first in a stream (for a given type_url), since the server will already be correctly tracking the client's state.
    (In ADS, the first message *of each type_url* of a reconnected stream populates this map.)
    The map's keys are names of xDS resources known to the xDS client.
    The map's values are opaque resource versions.
    */
    initial_resource_versions: HashMap<String, String>,

    /**
    When the DeltaDiscoveryRequest is a ACK or NACK message in response to a previous DeltaDiscoveryResponse, the response_nonce must be the nonce in the DeltaDiscoveryResponse.
    Otherwise (unlike in DiscoveryRequest) response_nonce must be omitted.
    */
    response_nonce: String,

    /*
    This is populated when the previous [`DiscoveryResponse`][crate::service.discovery::DiscoveryResponse] failed to update configuration. The `message` field in `error_details` provides the Envoy internal exception related to the failure.
    */
    error_detail: Status
}

pub struct DeltaDiscoveryResponse {
    /// The version of the response data (used for debugging).
    system_version_info: String,

    /// The response resources. These are typed resources, whose types must match the type_url field.
    resources: Vec<Resource>,

    /// Type URL for resources. Identifies the xDS API when muxing over ADS.
    /// Must be consistent with the type_url in the Any within 'resources' if 'resources' is non-empty.
    type_url: String,

    /// Resources names of resources that have be deleted and to be removed from the xDS Client.
    /// Removed resources for missing resources can be ignored.
    removed_resources: Vec<String>,

    /// Alternative to removed_resources that allows specifying which variant of a resource is being removed. This variant must be used for any resource for which dynamic parameter constraints were sent to the client.
    removed_resource_names: Vec<ResourceName>,

    /// The nonce provides a way for DeltaDiscoveryRequests to uniquely reference a DeltaDiscoveryResponse when (N)ACKing. The nonce is required.
    nonce: String,

    /// The control plane instance that sent the response.
    control_plane: ControlPlane
}

/// Specifies a resource to be subscribed to.
pub struct ResourceLocator {
    /// The resource name to subscribe to.
    name: String,
  
    /*
    A set of dynamic parameters used to match against the dynamic parameter constraints on the resource. This allows clients to select between multiple variants of the same resource.
    */
    dynamic_parameters: HashMap<String, String>
}
  
/// Specifies a concrete resource name.
pub struct ResourceName {
    /// The name of the resource.
    name: String,
  
    /*
    Dynamic parameter constraints associated with this resource. To be used by client-side caches (including xDS proxies) when matching subscribed resource locators.
    */
    dynamic_parameter_constraints: DynamicParameterConstraints
}

/**
A set of dynamic parameter constraints associated with a variant of an individual xDS resource.
These constraints determine whether the resource matches a subscription based on the set of dynamic parameters in the subscription, as specified in the [`ResourceLocator.dynamic_parameters`][crate::service.discovery::ResourceLocator.dynamic_parameters] field. This allows xDS implementations (clients, servers, and caching proxies) to determine which variant of a resource is appropriate for a given client.
*/
pub struct DynamicParameterConstraints {  
    r#type: Box<Type>
}

pub enum Type {
    /// A single constraint to evaluate.
    Constraint(SingleConstraint),

    /// A list of constraints that match if any one constraint in the list matches.
    OrConstraints(ConstraintList),

    /// A list of constraints that must all match.
    AndConstraints(ConstraintList),

    /// The inverse (NOT) of a set of constraints.
    NotConstraints(DynamicParameterConstraints)
}

/// A single constraint for a given key.
pub struct SingleConstraint {
    /// The key to match against.
    key: String,

    constraint_type: ConstraintType
}

pub enum ConstraintType {
    // option (validate.required) = true;

    /// Matches this exact value.
    Value(String),

    /**
    Key is present (matches any value except for the key being absent).
    This allows setting a default constraint for clients that do not send a key at all, while there may be other clients that need special configuration based on that key.
    */
    Exists(Exists)
}

pub struct Exists {
}

pub struct ConstraintList {
    constraints: Vec<DynamicParameterConstraints>
}
  
pub struct Resource {  
    /**
    The resource's name, to distinguish it from others of the same type of resource.
    Only one of `name` or `resource_name` may be set.
    */
    name: String,
  
    /**
    Alternative to the `name` field, to be used when the server supports multiple variants of the named resource that are differentiated by dynamic parameter constraints.
    Only one of `name` or `resource_name` may be set.
    */
    resource_name: ResourceName,
  
    /// The aliases are a list of other names that this resource can go by.
    aliases: Vec<String>,
  
    /// The resource level version. It allows xDS to track the state of individual resources.
    version: String,
  
    /// The resource being tracked.
    resource: Any,
  
    /**
    Time-to-live value for the resource. For each resource, a timer is started. The timer is reset each time the resource is received with a new TTL. If the resource is received with no TTL set, the timer is removed for the resource. Upon expiration of the timer, the configuration for the resource will be removed.

    The TTL can be refreshed or changed by sending a response that doesn't change the resource version. In this case the resource field does not need to be populated, which allows for light-weight "heartbeat" updates to keep a resource with a TTL alive.

    The TTL feature is meant to support configurations that should be removed in the event of a management server failure. For example, the feature may be used for fault injection testing where the fault injection should be terminated in the event that Envoy loses contact with the management server.
    */
    ttl: Duration,
  
    /// Cache control properties for the resource.
    cache_control: CacheControl,
  
    /**
    The Metadata field can be used to provide additional information for the resource.
    E.g. the trace data for debugging.
    */
    metadata: Metadata
}

/// Cache control properties for the resource.
pub struct CacheControl {
    /**
    If `true`, xDS proxies may not cache this resource.
    Note that this does not apply to clients other than xDS proxies, which must cache resources for their own use, regardless of the value of this field.
    */
    do_not_cache: bool
}
