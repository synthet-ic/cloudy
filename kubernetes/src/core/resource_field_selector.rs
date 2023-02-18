//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/resource-field-selector/>

use kfl::Decode;

use crate::quantity::Quantity;

#[derive(Debug, Decode)]
pub struct ResourceFieldSelector {
    resource: String,
    container_name: Option<String>,
    divisor: Quantity
}
