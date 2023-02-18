use serde::{Serialize, Deserialize};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BasicAuth {
    username: Option<String>,
    password: Option<Secret>,
    password_file: Option<String>
}
