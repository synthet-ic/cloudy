/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/rbac/v3/rbac.proto>
*/

// google.api.expr.v1alpha1.Expr
type Expr = String;
// google.api.expr.v1alpha1.CheckedExpr
type CheckedExpr = String;


use std::collections::HashMap;

use crate::{
    config::{
        core::{
            address::CIDRRange,
            extension::TypedExtensionConfig,
        },
        route::route_components::HeaderMatcher
    },
    types::{
        matcher::{
            filter_state::FilterStateMatcher,
            metadata::MetadataMatcher,
            path::PathMatcher,
            string::StringMatcher
        },
        range::I32Range,
    }
};

/**
Role Based Access Control (RBAC) provides service-level and method-level access control for a service. Requests are allowed or denied based on the `action` and whether a matching policy is found. For instance, if the action is ALLOW and a matching policy is found the request should be allowed.

RBAC can also be used to make access logging decisions by communicating with access loggers through dynamic metadata. When the action is LOG and at least one policy matches, the `access_log_hint` value in the shared key namespace 'envoy.common' is set to `true` indicating the request should be logged.

Here is an example of RBAC configuration. It has two policies:

- Service account `cluster.local/ns/default/sa/admin` has full access to the service, and so does "cluster.local/ns/default/sa/superuser".

- Any user can read (`GET`) the service at paths with prefix `/products`, so long as the destination port is either 80 or 443.

```yaml
action: ALLOW
policies:
  "service-admin":
    permissions:
    - any: true
    principals:
    - authenticated:
        principal_name:
            exact: "cluster.local/ns/default/sa/admin"
    - authenticated:
        principal_name:
            exact: "cluster.local/ns/default/sa/superuser"
  "product-viewer":
    permissions:
    - and_rules:
        rules:
        - header:
            name: ":method"
            string_match:
                exact: "GET"
        - url_path:
            path: { prefix: "/products" }
        - or_rules:
            rules:
                - destination_port: 80
                - destination_port: 443
    principals:
    - any: true
```
*/
pub struct RBAC {
    /**
    The action to take if a policy matches. Every action either allows or denies a request, and can also carry out action-specific operations.

    Actions:

    - `ALLOW`: Allows the request if and only if there is a policy that matches the request.
    - `DENY`: Allows the request if and only if there are no policies that match the request.
    - `LOG`: Allows all requests. If at least one policy matches, the dynamic metadata key `access_log_hint` is set to the value `true` under the shared key namespace `envoy.common`. If no policies match, it is set to `false`.  Other actions do not modify this key.

    [(validate.rules).enum = {defined_only: true}];
    */
    action: RBACAction,

    /**
    Maps from policy name to policy. A match occurs when at least one policy matches the request.
    The policies are evaluated in lexicographic order of the policy name.
    */
    policies: HashMap<String, Policy>
}

/// Should we do safe-list or block-list style access control?
pub enum RBACAction {
    /// The policies grant access to principals. The rest are denied. This is safe-list style access control. This is the default type.
    Allow,

    /// The policies deny access to principals. The rest are allowed. This is block-list style access control.
    Deny,

    /**
    The policies set the `access_log_hint` dynamic metadata key based on if requests match.
    All requests are allowed.
    */
    Log
}

/**
Policy specifies a role and the principals that are assigned/denied the role.
A policy matches if and only if at least one of its permissions match the action taking place AND at least one of its principals match the downstream AND the condition is true if specified.
*/
pub struct Policy {
    /**
    Required. The set of permissions that define a role. Each permission is matched with OR semantics. To match all actions for this policy, a single
    Permission with the `any` field set to true should be used.

    [(validate.rules).repeated = {min_items: 1}];
    */
    permissions: Vec<Permission>,

    /**
    Required. The set of principals that are assigned/denied the role based on
    “action”. Each principal is matched with OR semantics. To match all downstreams for this policy, a single Principal with the `any` field set to
    `true` should be used.

    [(validate.rules).repeated = {min_items: 1}];
    */
    principals: Vec<Principal>,

    /**
    An optional symbolic expression specifying an access control :ref:`condition <arch_overview_condition>`. The condition is combined with the permissions and the principals as a clause with AND semantics.
    Only be used when checked_condition is not used.

    [(udpa.annotations.field_migrate).oneof_promotion = "expression_specifier"];
    */
    condition: Expr,
        

    /**
    [#not-implemented-hide:]
    An optional symbolic expression that has been successfully type checked.
    Only be used when condition is not used.

    [(udpa.annotations.field_migrate).oneof_promotion = "expression_specifier"];
    */
    checked_condition: CheckedExpr
        
}

/**
Permission defines an action (or actions) that a principal can take.
*/
pub struct Permission {
    rule: Box<Rule>
}

pub enum Rule {
    // option (validate.required) = true;

    /// A set of rules that all must match in order to define the action.
    AndRules(PermissionSet),

    /// A set of rules where at least one must match in order to define the action.
    OrRules(PermissionSet),

    /// When any is set, it matches any action.
    // [(validate.rules).bool = {const: true}];
    Any(bool),

    /**
    A header (or pseudo-header such as :path or :method) on the incoming HTTP request. Only available for HTTP request.
    Note: the pseudo-header :path includes the query and fragment string. Use the `url_path` field if you want to match the URL path without the query and fragment string.
    */
    Header(HeaderMatcher),

    // A URL path on the incoming HTTP request. Only available for HTTP.
    URLPath(PathMatcher),

    /// A CIDR block that describes the destination IP.
    DestinationIP(CIDRRange),

    /// A port number that describes the destination port connecting to.
    DestinationPort(u16),

    /// A port number range that describes a range of destination ports connecting to.
    DestinationPortRange(I32Range),

    /// Metadata that describes additional information about the action.
    Metadata(MetadataMatcher),

    /**
    Negates matching the provided permission. For instance, if the value of `not_rule` would match, this permission would not match. Conversely, if the value of `not_rule` would not match, this permission would match.
    */
    NotRule(Permission),

    /**
    The request server from the client's connection request. This is typically TLS SNI.

    > attention: The behaviour of this field may be affected by how Envoy is configured as explained below.
    > - If the :ref:`TLS Inspector <config_listener_filters_tls_inspector>` filter is not added, and if a `FilterChainMatch` is not defined for the [server name][crate::config::listener::listener_components::FilterChainMatch::server_names], a TLS connection's requested SNI server name will be treated as if it wasn't present.
    > - A :ref:`listener filter <arch_overview_listener_filters>` may overwrite a connection's requested server name within Envoy.

    Please refer to :ref:`this FAQ entry <faq_how_to_setup_sni>` to learn to
    setup SNI.
    */
    RequestedServerName(StringMatcher),

    // Extension for configuring custom matchers for RBAC.
    // [#extension-category: envoy.rbac.matchers]
    Matcher(TypedExtensionConfig),
}

/**
Used in the `and_rules` and `or_rules` fields in the `rule` oneof. Depending on the context, each are applied with the associated behaviour.
*/
pub struct PermissionSet {
    // [(validate.rules).repeated = {min_items: 1}];
    rules: Vec<Permission>
}

/**
Principal defines an identity or a group of identities for a downstream
subject.
*/
pub struct Principal {
    identifier: Box<Identifier>
}

pub enum Identifier {
    // option (validate.required) = true;

    /// A set of identifiers that all must match in order to define the downstream.
    AndIds(Principal),

    /// A set of identifiers at least one must match in order to define the downstream.
    OrIds(Principal),

    /// When any is set, it matches any downstream.
    // [(validate.rules).bool = {const: true}];
    Any(bool),

    /// Authenticated attributes that identify the downstream.
    Authenticated(Authenticated),

    /**
    A CIDR block that describes the downstream remote/origin address.
    Note: This is always the physical peer even if the
    :ref:`remote_ip [crate::config::rbac::Principal::remote_ip]` is
    inferred from for example the x-forwarder-for header, proxy protocol,
    etc.
    */
    DirectRemoteIP(CIDRRange),

    /**
    A CIDR block that describes the downstream remote/origin address.
    Note: This may not be the physical peer and could be different from the :ref:`direct_remote_ip [crate::config::rbac::Principal::direct_remote_ip]`. E.g, if the remote ip is inferred from for example the x-forwarder-for header, proxy protocol, etc.
    */
    RemoteIP(CIDRRange),

    /**
    A header (or pseudo-header such as :path or :method) on the incoming HTTP request. Only available for HTTP request. Note: the pseudo-header :path includes the query and fragment string. Use the `url_path` field if you want to match the URL path without the query and fragment string.
    */
    Header(HeaderMatcher),

    /// A URL path on the incoming HTTP request. Only available for HTTP.
    URLPath(PathMatcher),

    /// Metadata that describes additional information about the principal.
    Metadata(MetadataMatcher),

    /// Identifies the principal using a filter state object.
    FilterState(FilterStateMatcher),

    /**
    Negates matching the provided principal. For instance, if the value of `not_id` would match, this principal would not match. Conversely, if the value of `not_id` would not match, this principal would match.
    */
    NotId(Principal),
}

/**
Used in the `and_ids` and `or_ids` fields in the `identifier` oneof.
Depending on the context, each are applied with the associated behaviour.
*/
pub struct PrincipalSet {
    // [(validate.rules).repeated = {min_items: 1}];
    ids: Vec<Principal>
}

/// Authentication attributes for a downstream.
pub struct Authenticated {
    /**
    The name of the principal. If set, The URI SAN or DNS SAN in that order is used from the certificate, otherwise the subject field is used. If unset, it applies to any user that is authenticated.
    */
    principal_name: StringMatcher
}

/// Action defines the result of allowance or denial when a request matches the matcher.
pub struct Action {
    /// The name indicates the policy name.
    // [!is_empty()]
    name: String,

    /**
    The action to take if the matcher matches. Every action either allows or denies a request, and can also carry out action-specific operations.

    Actions:

    - `Allow`: If the request gets matched on ALLOW, it is permitted.
    - `Deny`: If the request gets matched on DENY, it is not permitted.
    - `Log`: If the request gets matched on LOG, it is permitted. Besides, the
       dynamic metadata key `access_log_hint` under the shared key namespace
       `envoy.common` will be set to the value `true`.
    - If the request cannot get matched, it will fallback to `DENY`.

    Log behaviour:

    If the RBAC matcher contains at least one LOG action, the dynamic metadata key `access_log_hint` will be set based on if the request get matched on the LOG action.

    */
    action: RBACAction
}
