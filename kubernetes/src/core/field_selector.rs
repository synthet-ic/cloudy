//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-field-selector/>

use std::path::PathBuf;

use kfl::Decode;

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Debug, Decode)]
pub struct FieldSelector {
    /// Path of the field to select in the specified API version.
    #[kfl(property)]
    field_path: PathBuf,
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[kfl(property, default = "v1".into())]
    api_version: String
}
