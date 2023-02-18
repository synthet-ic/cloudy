/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/extensions/transport_sockets/tls/v3/secret.proto>
*/

use crate::{
    config::core::{
        base::DataSource,
        config_source::ConfigSource
    },
    extensions::transport_sockets::tls::common::{
        CertificateValidationContext, TLSCertificate, TLSSessionTicketKeys
    }
};

pub struct GenericSecret {
    /// Secret of generic type and is available to filters.
    /// [(udpa.annotations.sensitive) = true];
    secret: DataSource
}

pub struct SDSSecretConfig {
    /**
    Name by which the secret can be uniquely referred to. When both name and config are specified, then secret can be fetched and/or reloaded via SDS. When only name is specified, then secret will be loaded from static resources.

    [(validate.rules).string = {min_len: 1}];
    */
    name: String,

    sds_config: ConfigSource
}

pub struct Secret {
    /// Name (FQDN, UUID, SPKI, SHA256, etc.) by which the secret can be uniquely referred to.
    name: String,

    r#type: Type
}

pub enum Type {
    TLSCertificate(TLSCertificate),

    SessionTicketKeys(TLSSessionTicketKeys),

    ValidationContext(CertificateValidationContext),

    GenericSecret(GenericSecret)
}
