/*!
References <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-review-v1/>
*/

use kfl::Decode;

use crate::{
    meta::metadata::Metadata,
    time::Time
};

#[derive(Debug, Decode)]
pub struct TokenReview {
    metadata: Metadata,
    spec: TokenReviewSpec,
    status: Option<TokenReviewStatus>
}

/**
<https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-review-v1/#TokenReviewSpec>

TokenReviewSpec is a description of the token authentication request.
*/
#[derive(Debug, Decode)]
pub struct TokenReviewSpec {
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-review-v1/#TokenReviewStatus>
#[derive(Debug, Decode)]
pub struct TokenReviewStatus {
}
