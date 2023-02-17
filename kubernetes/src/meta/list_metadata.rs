//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/>

use kfl::Decode;

#[derive(Debug, Decode)]
pub struct ListMeta {
    r#continue: String,
    remaining_item_count: Option<i64>,
    resource_version: Option<String>,
    self_link: Option<String>
}
