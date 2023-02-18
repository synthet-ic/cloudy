/*!
- Concepts <https://kubernetes.io/docs/concepts/services-networking/ingress/>
- Reference <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-v1/>
*/

use kfl::Decode;

use crate::{
    core::typed_local_reference::TypedLocalReference,
    meta::metadata::Metadata,
    // port_status::PortStatus,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-v1/#Ingress>
/// Ingress is a collection of rules that allow inbound connections to reach the endpoints defined by a backend. An Ingress can be configured to give services externally-reachable urls, load balance traffic, terminate SSL, offer name based virtual hosting etc.
#[derive(Debug, Decode)]
pub struct Ingress {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-v1/#IngressSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    /// `default_backend` is the backend that should handle requests that don't match any rule. If [`rules`][Self::rules] are not specified, `default_backend` must be specified. If `default_backend` is not set, the handling of requests that do not match any of the rules will be up to the Ingress controller.
    default_backend: Option<Backend>,
    /// `ingress_class_name` is the name of an IngressClass cluster resource. Ingress controller implementations use this field to know whether they should be serving this Ingress resource, by a transitive connection (controller -> IngressClass -> Ingress resource). Although the kubernetes.io/ingress.class annotation (simple constant name) was never formally defined, it was widely supported by Ingress controllers to create a direct binding between Ingress controller and Ingress resources. Newly created Ingress resources should prefer using the field. However, even though the annotation is officially deprecated, for backwards compatibility reasons, ingress controllers should still honour that annotation if present.
    ingress_class_name: Option<String>,
    /// A list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    rules: Vec<Rule>,
    /// TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    tls: Vec<IngressTLS>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-v1/#IngressBackend>
///
/// Backend describes all endpoints for a given service and port.
#[derive(Debug, Decode)]
pub enum Backend {
    /// Resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object. If resource is specified, a [`service.name`][Service::name] and [`service.port`][Service::port] must not be specified. This is a mutually exclusive setting with [`service`][Self::service].
    Resource(TypedLocalReference),
    /// Service references a Service as a Backend. This is a mutually exclusive setting with [`resource`][Self::resource].
    Service(backend::Service),
}

pub mod backend {
    use kfl::Decode;

    /// IngressServiceBackend references a Kubernetes Service as a Backend.
    #[derive(Debug, Decode)]
    pub struct Service {
        /// Name is the referenced service. The service must exist in the same namespace as the Ingress object.
        name: String,
        /// Port of the referenced service. A port name or port number is required for a Service.
        port: Port
    }

    #[derive(Debug, DecodeScalar)]
    pub enum Port {
        /// Name is the name of the port on the Service. This is a mutually exclusive setting with [`number`][Self::number].
        Name(String),
        /// Number is the numerical port number (e.g. `80`) on the Service. This is a mutually exclusive setting with [`name`][Self::name].
        Number(u16)
    }
}

/// Rule represents the rules mapping the paths under a specified host to the related backend services. Incoming requests are first evaluated for a host match, then routed to the backend associated with the matching IngressRuleValue.
#[derive(Debug, Decode)]
pub struct Rule {
    /// Host is the fully qualified domain name of a network host, as defined by [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986). Note the following deviations from the 'host' part of the URI as defined in RFC 3986:
    ///
    /// 1. IPs are not allowed. Currently an IngressRuleValue can only apply to the IP in the Spec of the parent Ingress.
    ///
    /// 2. The : delimiter is not respected because ports are not allowed. Currently the port of an Ingress is implicitly :80 for http and :443 for https. Both these may change in the future. Incoming requests are matched against the host before the IngressRuleValue. If the host is unspecified, the Ingress routes all traffic based on the specified IngressRuleValue.
    ///
    /// Host can be 'precise' which is a domain name without the terminating dot of a network host (e.g. `"foo.bar.com"`) or 'wildcard', which is a domain name prefixed with a single wildcard label (e.g. `".foo.com"`). The wildcard character `*` must appear by itself as the first DNS label and matches only a single label. You cannot have a wildcard label by itself (e.g. `host = "*"`). Requests will be matched against the Host field in the following way:
    ///
    /// 1. If `host` is precise, the request matches this rule if the http host header is equal to `host`.
    ///
    /// 2. If `host` is a wildcard, then the request matches this rule if the http host header is to equal to the suffix (removing the first label) of the wildcard rule.
    host: Option<String>,
    http: Option<rule::Http>
}

pub mod rule {
    use kfl::{Decode, DecodeScalar};

    /// Http is a list of http selectors pointing to backends. In the example: http:///? -> backend where where parts of the url correspond to [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339), this resource will be used to match against everything after the last '/' and before the first '?' or '#'.
    #[derive(Debug, Decode)]
    pub struct Http {
        /// A collection of paths that map requests to backends.
        paths: Vec<Path>,
    }

    /// Path associates a path with a backend. Incoming urls matching the path are forwarded to the backend.
    #[derive(Debug, Decode)]
    pub struct Path {
        /// Backend defines the referenced service endpoint to which the traffic will be forwarded to.
        backend: Backend,
        /// PathType determines the interpretation of the Path matching.
        r#type: Type,
        /// `path` is matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional 'path' part of a URL as defined by [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986). Paths must begin with a '/' and must be present when using [`path_type`][Self::path_type] with value `Exact` or `Prefix`.
        path: Option<String>
    }

    #[derive(Debug, DecodeScalar)]
    pub enum Type {
        /// Matches the URL path exactly.
        Exact,
        /// Matches based on a URL path prefix split by '/'. Matching is done on a path element by element basis. A path element refers is the list of labels in the path split by the '/' separator. A request is a match for path p if every p is an element-wise prefix of p of the request path. Note that if the last element of the path is a substring of the last element in request path, it is not a match (e.g. /foo/bar matches /foo/bar/baz, but does not match /foo/barbaz).
        Prefix,
        /// Interpretation of the Path matching is up to the IngressClass. Implementations can treat this as a separate PathType or treat it identically to `Prefix` or `Exact` path types. Implementations are required to support all path types.
        ImplementationSpecific
    }
}

#[derive(Debug, Decode)]
pub struct IngressTLS {
    hosts: Vec<String>,
    secret_name: Option<String>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/service-resources/ingress-v1/#IngressStatus>
///
/// IngressStatus describe the current state of the Ingress.
#[derive(Debug, Decode)]
pub struct Status {
    /// LoadBalancer contains the current status of the load-balancer.
    load_balancer: Option<status::LoadBalancer>,
}

pub mod status {
    use kfl::Decode;
    use crate::port_status::Port;

    /// LoadBalancer represents the status of a load-balancer.
    #[derive(Debug, Decode)]
    pub struct LoadBalancer {
        /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
        ingress: Vec<Ingress>,
    }

    /// Ingress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.
    #[derive(Debug, Decode)]
    pub struct Ingress {
        /// Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
        host_names: Option<String>,
        /// IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
        ip: Option<String>,
        /// Ports is a list of records of service ports If used, every port defined in the service should have an entry in it.
        ports: Vec<Port>
    }
}
