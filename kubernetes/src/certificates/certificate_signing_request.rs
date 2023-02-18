//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/certificate-signing-request-v1/>

use std::collections::HashMap;

use kfl::Decode;

use crate::meta::{
    condition::Condition,
    metadata::Metadata
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/certificate-signing-request-v1/#CertificateSigningRequest>
///
///
/// CertificateSigningRequest objects provide a mechanism to obtain x509 certificates by submitting a certificate signing request, and having it asynchronously approved and issued.
///
/// Kubelets use this API to obtain:
///
/// 1. client certificates to authenticate to kube-apiserver (with the `"kubernetes.io/kube-apiserver-client-kubelet"` [`signer_name`][Spec::signer_name]).
///
/// 2. serving certificates for TLS endpoints kube-apiserver can connect to securely (with the `"kubernetes.io/kubelet-serving"` [`signer_name`][Spec::signer_name]).
///
/// This API can be used to request client certificates to authenticate to kube-apiserver (with the "kubernetes.io/kube-apiserver-client" [`signer_name`][Spec::signer_name]), or to obtain certificates from custom non-Kubernetes signers.
#[derive(Debug, Decode)]
pub struct CertificateSigningRequest {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/**
<https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/certificate-signing-request-v1/#CertificateSigningRequestSpec>

Spec contains the certificate request.
*/
#[derive(Debug, Decode)]
pub struct Spec {
    request: Vec<u8>,
    signer_name: String,
    expiration_seconds: Option<u16>,
    extra: HashMap<String, String>,
    groups: Vec<String>,
    uid: Option<String>,
    usages: Vec<String>,
    username: Option<String>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/certificate-signing-request-v1/#CertificateSigningRequestStatus>
///
/// Status contains conditions used to indicate approved/denied/failed status of the request, and the issued certificate.
#[derive(Debug, Decode)]
pub struct Status {
    certificate: Vec<u8>,
    conditions: Option<Vec<Condition<CertificateConditionType>>>
}

/**
Type of the condition. Known conditions are `Approved`, `Denied`, and `Failed`.

- An `Approved` condition is added via the /approval subresource, indicating the request was approved and should be issued by the signer.

- A `Denied` condition is added via the /approval subresource, indicating the request was denied and should not be issued by the signer.

- A `Failed` condition is added via the /status subresource, indicating the signer failed to issue the certificate.

`Approved` and `Denied` conditions are mutually exclusive. `Approved`, `Denied`, and `Failed` conditions cannot be removed once added.

Only one condition of a given type is allowed.
*/
#[derive(Debug, Decode)]
pub enum CertificateConditionType {
    Approved,
    Denied,
    Failed
}
