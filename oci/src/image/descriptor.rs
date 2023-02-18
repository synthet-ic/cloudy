/*!
<https://github.com/opencontainers/image-spec/blob/main/descriptor.md>
*/

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Descriptor {
    media_type: MediaType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    
}
