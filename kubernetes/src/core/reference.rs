//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-reference/>

use kfl::Decode;

#[derive(Debug, Decode)]
pub struct Reference {
    field_path: String,
    kind: String,
    name: String,
    namespace: String,
    resource_version: String,
    uid: String
}
