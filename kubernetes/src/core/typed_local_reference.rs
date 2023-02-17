/*!
Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/typed-local-object-reference/>
*/

use kfl::Decode;

#[derive(Debug, Decode)]
pub struct TypedLocalReference {
    kind: String,
    name: String,
    api_group: Option<String>
}

#[derive(Debug, Decode)]
pub enum TypedLocalReferenceKind {

}
