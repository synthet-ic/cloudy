//! - Concepts <https://kubernetes.io/docs/concepts/configuration/configmap/>
//! - Tasks <https://kubernetes.io/docs/tasks/configure-pod-container/configure-pod-configmap/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/config-map-v1/>

use std::collections::HashMap;
use kfl::Decode;

use crate::meta::metadata::Metadata;

/// ConfigMap holds configuration data for pods to consume.
/// 
/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/config-map-v1/#ConfigMap>
#[derive(Debug, Decode)]
pub struct ConfigMap {
    metadata: Metadata,
    /// BinaryData contains the binary data. Each key must consist of alphanumeric characters, '-', '_' or '.'. BinaryData can contain byte sequences that are not in the UTF-8 range. The keys stored in BinaryData must not overlap with the ones in the Data field, this is enforced during validation process. Using this field will require 1.10+ apiserver and kubelet.
    binary_data: HashMap<String, Vec<u8>>,
    /// Data contains the configuration data. Each key must consist of alphanumeric characters, '-', '_' or '.'. Values with non-UTF-8 byte sequences must use the BinaryData field. The keys stored in Data must not overlap with the keys in the BinaryData field, this is enforced during validation process.
    data: HashMap<String, String>,
    /// Immutable, if set to true, ensures that data stored in the ConfigMap cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.
    immutable: Option<bool>
}
