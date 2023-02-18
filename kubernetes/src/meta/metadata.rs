//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/>

use std::collections::HashMap;
use kfl::{Decode, DecodeScalar};

use crate::time::Time;

/// Metadata that all persisted resources must have, which includes all objects users must create.
#[derive(Debug, Decode)]
pub struct Metadata {
    /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: <http://kubernetes.io/docs/user-guide/identifiers#names>
    #[kfl(argument)]
    pub name: Option<String>,
    /// Optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.
    ///
    /// If this field is specified and the generated name exists, the server will return a 409.
    ///
    /// Applied only if Name is not specified. More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency>
    pub generate_name: Option<String>,
    /// Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
    /// 
    /// Must be a `DNS_LABEL`. Cannot be updated. More info: <http://kubernetes.io/docs/user-guide/namespaces>
    pub namespace: Option<String>,
    /// Map of string keys and values that can be used to organise and categorise (scope and select) objects. May match selectors of replication controllers and services. More info: <http://kubernetes.io/docs/user-guide/labels>
    pub labels: HashMap<String, String>,
    /// Unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: <http://kubernetes.io/docs/user-guide/annotations>
    pub annotations: HashMap<String, String>,

    // System

    /// <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#System>
    finalisers: Vec<String>,
    managed_fields: Vec<ManagedFieldsEntry>,
    owner_references: Vec<OwnerReference>,
    // Read-Only
    /// <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#Read-only>
    creation_timestamp: Option<Time>,
    deletion_grace_period_seconds: Option<i64>,
    deletion_timestamp: Option<Time>,
    generation: Option<i64>,
    resource_version: Option<String>,
    self_link: Option<String>,
    uid: Option<String>
}

#[derive(Debug, Decode)]
pub struct ManagedFieldsEntry {
    api_version: Option<String>,
    fields_type: Option<FieldsType>,
    // fieldsV1: Option<fieldsV1>,
    manager: Option<String>,
    operation: Option<ManagedFieldsEntryOperation>,
    subresource: Option<String>,
    time: Option<Time>
}

#[derive(Debug, DecodeScalar)]
pub enum FieldsType {
    FieldsV1
}

#[derive(Debug, DecodeScalar)]
pub enum ManagedFieldsEntryOperation {
    Apply,
    Update
}

#[derive(Debug, Decode)]
pub struct OwnerReference {
    api_version: String,
    kind: String,
    name: String,
    uid: String,
    block_owner_deletion: Option<bool>,
    controller: Option<bool>
}
