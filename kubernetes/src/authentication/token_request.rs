//! References <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/>

use kfl::Decode;

use crate::{
    meta::metadata::Metadata,
    time::Time
};

#[derive(Debug, Decode)]
pub struct TokenRequest {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// TokenRequestSpec contains client provided parameters of a token request.
/// 
/// <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequestSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    /// Intendend audiences of the token. A recipient of a token must identify themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences.
    audiences: Vec<String>,
    /// Reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The API server's TokenReview endpoint will validate the BoundObjectRef, but other audiences may not. Keep ExpirationSeconds small if you want prompt revocation.
    bound_object_ref: Option<BoundObjectReference>,
    /// Requested duration of validity of the request. The token issuer may return a token with a different validity duration so a client needs to check the `expiration` field in a response.
    expiration_seconds: u32
}

#[derive(Debug, Decode)]
pub struct BoundObjectReference {
    kind: Option<bound_object_ref::Kind>,
    name: Option<String>,
    uid: Option<String>
}

pub mod bound_object_ref {
    use kfl::DecodeScalar;

    #[derive(Debug, DecodeScalar)]
    pub enum Kind {
        Pop,
        Secret
    }
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequestStatus>
#[derive(Debug, Decode)]
pub struct Status {
    /// Time of expiration of the returned token.
    expiration_timestamp: Time,
    /// Token is the opaque bearer token.
    token: String
}
