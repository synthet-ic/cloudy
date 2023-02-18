/*!
<https://github.com/opencontainers/image-spec/blob/main/manifest.md>
*/

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use super::descriptor::Descriptor;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    schema_version: i32,
    media_type: MediaType,
    config: Descriptor,
    layers: Option<Vec<Descriptor>>,
    subject: Option<Descriptor>,
    annotations: Option<HashMap<String, String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    
}
