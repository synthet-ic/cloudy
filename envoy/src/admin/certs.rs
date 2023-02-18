/*!
- <https://github.com/envoyproxy/envoy/blob/main/api/envoy/admin/v3/certs.proto>
- <https://www.envoyproxy.io/docs/envoy/latest/api-v3/admin/v3/certs.proto>
*/

use std::time::SystemTime;

/**
Proto representation of certificate details. Admin endpoint uses this wrapper for `/certs` to display certificate information. See :ref:`/certs <operations_admin_interface_certs>` for more information.
*/
pub struct Certificates {
    /// List of certificates known to an Envoy.
    certificates: Vec<Certificate>,
}

pub struct Certificate {
    /// Details of CA certificate.
    ca_cert: Vec<CertificateDetails>,

    /// Details of Certificate Chain
    cert_chain: Vec<CertificateDetails>,
}

pub struct CertificateDetails {
    /// Path of the certificate.
    path: String,

    /// Certificate Serial Number.
    serial_number: String,

    /// List of Subject Alternate names.
    subject_alt_names: Vec<SubjectAlternateName>,

    /// Minimum of days until expiration of certificate and it's chain.
    days_until_expiration: u64,

    /// Indicates the time from which the certificate is valid.
    valid_from: SystemTime,

    /// Indicates the time at which the certificate expires.
    expiration_time: SystemTime,

    /// Details related to the OCSP response associated with this certificate, if any.
    ocsp_details: OCSPDetails,
}

pub struct OCSPDetails {
    /// Indicates the time from which the OCSP response is valid.
    valid_from: SystemTime,

    /// Indicates the time at which the OCSP response expires.
    expiration: SystemTime,
}

/// Subject Alternate Name.
pub enum SubjectAlternateName {
    DNS(String),

    URI(String),

    IPAddress(String)
}
