//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/local-object-reference/>

use kfl::Decode;

#[derive(Debug, Decode)]
pub struct LocalReference {
    name: String
}
