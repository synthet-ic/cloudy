use serde::{Serialize, Deserialize};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Authorisation {
    /// Sets the authentication type.
    #[serde(rename(serialize = "AuthorisationType::Bearer"))]
    r#type: Option<AuthorisationType>,
    /// Sets the credentials. It is mutually exclusive with `credentials_file`.
    credentials: Option<Secret>,
    /**
    Sets the credentials to the credentials read from the configured file.
    It is mutually exclusive with `credentials`.
    */
    credentials_file: Option<PathBuf>
}
