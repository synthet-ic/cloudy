/*!
<https://github.com/opencontainers/image-spec>
*/

pub mod artifact;
pub mod configuration;
pub mod descriptor;
pub mod index;
pub mod layout;
pub mod manifest;

use std::{
    collections::HashMap,
    path::PathBuf
};

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Image {
    
}
