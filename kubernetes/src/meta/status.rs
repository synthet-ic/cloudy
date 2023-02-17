//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/status/>

use kfl::Decode;

use crate::meta::list_metadata::ListMeta;

#[derive(Debug, Decode)]
pub struct Status {
    code: Option<i32>,
    details: Option<StatusDetails>,
    kind: Option<String>,
    message: Option<String>,
    metadata: Option<ListMeta>,
    reason: Option<String>,
    status: Option<StatusStatus>
}

#[derive(Debug, Decode)]
pub struct StatusDetails {
    
}

#[derive(Debug, Decode)]
pub enum StatusStatus {
    Success,
    Failure
}
