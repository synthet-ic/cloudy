//! References <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/custom-resource-definition-v1/>

use std::path::PathBuf;

use kfl::Decode;

use crate::meta::metadata::Metadata;

/// CustomResourceDefinition represents a resource that should be exposed on the API server. Its name MUST be in the format <.spec.name>.<.spec.group>.
/// 
/// <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/custom-resource-definition-v1/#CustomResourceDefinition>
#[derive(Debug, Decode)]
pub struct CustomResourceDefinition {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear.
/// 
/// <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/custom-resource-definition-v1/#CustomResourceDefinitionSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    /// group is the API group of the defined custom resource. The custom resources are served under /apis/\<group>/.... Must match the name of the CustomResourceDefinition (in the form \<names.plural>.\<group>).
    group: String,
    /// Specify the resource and kind names for the custom resource.
    names: Names,
    /// scope indicates whether the defined custom resource is cluster- or namespace-scoped. Allowed values are Cluster and Namespaced.
    scope: String,
    /// versions is the list of all API versions of the defined custom resource. Version names are used to compute the order in which served versions are listed in API discovery. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    version: Vec<Version>,
    /// Conversion settings for the CRD.
    conversion: Option<Conversion>,
}

/// Names indicates the names to serve this CustomResourceDefinition
#[derive(Debug, Decode)]
pub struct Names {
    /// kind is the serialised kind of the resource. It is normally CamelCase and singular. Custom resource instances will use this value as the kind attribute in API calls.
    kind: String,
    /// plural is the plural name of the resource to serve. The custom resources are served under `/apis/<group>/<version>/.../<plural>`. Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`). Must be all lowercase.
    plural: String,
    /// categories is a list of grouped resources this custom resource belongs to (e.g. 'all'). This is published in API discovery documents, and used by clients to support invocations like `kubectl get all`.
    categories: Vec<String>,
    /// listKind is the serialized kind of the list for this resource. Defaults to "kindList".
    #[kfl(default = "kindList".into())]
    list_kind: String,
    /// Short names for the resource, exposed in API discovery documents, and used by clients to support invocations like `kubectl get <shortname>`. It must be all lowercase.
    #[kfl(children)]
    short_names: Vec<String>,
    #[kfl(default = "kind".into())]
    singular: String
}

/// CustomResourceDefinitionVersion describes a version for CRD.
#[derive(Debug, Decode)]
pub struct Version {
    /// Version name, e.g. `"v1"`, `"v2beta1"`, etc. The custom resources are served under this version at `/apis/<group>/<version>/...` if `served` is true.
    name: String,
    /// Flag enabling/disabling this version from being served via REST APIs.
    served: bool,
    /// Indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true.
    storage: bool,
    /// Specifies additional columns returned in Table output. See <https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables> for details. If no columns are specified, a single column displaying the age of the custom resource is used.
    additional_printer_columns: Vec<Column>,
    /// Indicates this version of the custom resource API is deprecated. When set to true, API requests to this version receive a warning header in the server response. Defaults to false.
    #[kfl(default)]
    deprecated: bool,
    /// Overrides the default warning returned to API clients. May only be set when deprecated is true. The default warning indicates this version is deprecated and recommends use of the newest served version of equal or greater stability, if one exists.
    deprecation_warning: Option<String>,
    /// Describes the schema used for validation, pruning, and defaulting of this version of the custom resource.
    schema: Option<CustomResourceValidation>,
    /// Specify what subresources this version of the defined custom resource have.
    subresources: Option<Subresources>
}

/// CustomResourceValidation is a list of validation methods for CustomResources.


#[derive(Debug, Decode)]
pub struct CustomResourceValidation {
    /// OpenAPI v3 schema to use for validation and pruning.
    openapiv3_schema: JSONSchemaProps
}

// TODO
type JSONSchemaProps = String;

/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[derive(Debug, Decode)]
pub struct Subresources {
    /// Indicates the custom resource should serve a `/scale` subresource that returns an `autoscaling/v1` Scale object.
    scale: Option<Scale>,
    /// Indicates the custom resource should serve a `/status` subresource. When enabled: 1. requests to the custom resource primary endpoint ignore changes to the status stanza of the object. 2. requests to the custom resource `/status` subresource ignore changes to anything other than the status stanza of the object.
    status: subresource::Status
}

/// CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.
#[derive(Debug, Decode)]
pub struct Scale {
    /// Defines the JSON path inside of a custom resource that corresponds to Scale `spec.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.spec`. If there is no value under the given path in the custom resource, the `/scale` subresource will return an error on GET.
    spec_replicas_path: PathBuf,
    /// Defines the JSON path inside of a custom resource that corresponds to Scale `status.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status`. If there is no value under the given path in the custom resource, the `status.replicas` value in the `/scale` subresource will default to 0.
    status_replicas_path: PathBuf,
    /// Defines the JSON path inside of a custom resource that corresponds to Scale `status.selector`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status` or `.spec`. Must be set to work with HorizontalPodAutoscaler. The field pointed by this JSON path must be a string field (not a complex selector struct) which contains a serialized label selector in string form. More info: <https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions#scale-subresource> If there is no value under the given path in the custom resource, the `status.selector value in the `/scale` subresource will default to the empty string.
    label_selector_path: Option<PathBuf>,
}

pub mod subresource {
    use kfl::Decode;

    /// Status defines how to serve the status subresource for CustomResources. Status is represented by the .status JSON path inside of a CustomResource. When set, * exposes a /status subresource for the custom resource * PUT requests to the /status subresource take a custom resource object, and ignore changes to anything except the status stanza * PUT/POST/PATCH requests to the custom resource ignore changes to the status stanza
    #[derive(Debug, Decode)]
    pub struct Status;
}

/// CustomResourceColumnDefinition specifies a column for server side printing.
#[derive(Debug, Decode)]
pub struct Column {
    /// Simple JSON path (i.e. with array notation) which is evaluated against each custom resource to produce the value for this column.
    json_path: PathBuf,
    /// Human readable name for the column.
    name: String,
    /// OpenAPI type definition for this column. See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types> for details.
    r#type: String,
    /// Human readable description of this column.
    description: Option<String>,
    /// Optional OpenAPI type definition for this column. The 'name' format is applied to the primary identifier column to assist in clients identifying column is the resource name. See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types> for details.
    format: String,
    /// Integer defining the relative importance of this column compared to others. Lower numbers are considered higher priority. Columns that may be omitted in limited space scenarios should be given a priority greater than 0.
    priority: Option<u16>,
}

/// Conversion describes how to convert different versions of a CR.
#[derive(Debug, Decode)]
pub struct Conversion {
    /// How custom resources are converted between versions.
    strategy: conversion::Strategy,
    // TODO(rnarkk) webhook
}

pub mod conversion {
    use kfl::DecodeScalar;

    #[derive(Debug, DecodeScalar)]
    pub enum Strategy {
        /// The converter only change the apiVersion and would not touch any other field in the custom resource.
        None,
        /// API Server will call to an external webhook to do the conversion. Additional information is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhook to be set.
        Webhook
    }
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/extend-resources/custom-resource-definition-v1/#CustomResourceDefinitionStatus>
#[derive(Debug, Decode)]
pub struct Status {}
