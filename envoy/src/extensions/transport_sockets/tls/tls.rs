/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/extensions/transport_sockets/tls/v3/tls.proto>
*/

use std::time::Duration;

use crate::{
    config::core::{
        address::CIDRRange,
        extension::TypedExtensionConfig
    },
    extensions::transport_sockets::tls::{
        common::{
            CertificateProviderPluginInstance, CertificateValidationContext,
            TLSCertificate, TLSParameters, TLSSessionTicketKeys
        },
        secret::SDSSecretConfig
    }
};

pub struct  UpstreamTLSContext {
    /**
    Common TLS context settings.

    > attention: Server certificate verification is not enabled by default. Configure
      [`trusted_ca`][crate::extensions::transport_sockets.tls.v3.CertificateValidationContext.trusted_ca] to enable
      verification.
    */
    common_tls_context: CommonTLSContext,
  
    /// SNI string to use when creating TLS backend connections.
    // [(validate.rules).string = {max_bytes: 255}];
    sni: String,
  
    /**
    If `true`, server-initiated TLS renegotiation will be allowed.

    > attention: TLS renegotiation is considered insecure and shouldn't be used unless absolutely necessary.
    */
    allow_renegotiation: bool,
  
    /**
    Maximum number of session keys (Pre-Shared Keys for TLSv1.3+, Session IDs and Session Tickets for TLSv1.2 and older) to store for the purpose of session resumption.

    Defaults to `1`, setting this to 0 disables session resumption.
    */
    max_session_keys: u32,
}
  
pub struct DownstreamTLSContext {
    /// Common TLS context settings.
    common_tls_context: CommonTLSContext,
  
    /**
    If specified, Envoy will reject connections without a valid client certificate.
    */
    require_client_certificate: bool,
  
    /**
    If specified, Envoy will reject connections without a valid and matching SNI.
    [#not-implemented-hide:]
    */
    require_sni: bool,
  
    session_ticket_keys_type: SessionTicketKeysType,
  
    /**
    If specified, `session_timeout` will change the maximum lifetime (in seconds) of the TLS session.
    Currently this value is used as a hint for the `TLS session ticket lifetime (for TLSv1.2) <https://tools.ietf.org/html/rfc5077#section-5.6>`_.
    Only seconds can be specified (fractional seconds are ignored).
    */
    // [(validate.rules).duration = {
    //   lt {seconds: 4294967296}
    //   gte {}
    // }];
    session_timeout: Duration,
  
    /**
    Config for whether to use certificates if they do not have an accompanying OCSP response or if the response expires at runtime.
    Defaults to LENIENT_STAPLING
    */
    // [(validate.rules).enum = {defined_only: true}];
    ocsp_staple_policy: OCSPStaplePolicy,
  
    /**
    Multiple certificates are allowed in Downstream transport socket to serve different SNI.
    If the client provides SNI but no such cert matched, it will decide to full scan certificates or not based on this config.
    Defaults to false. See more details in :ref:`Multiple TLS certificates <arch_overview_ssl_cert_select>`.
    */
    full_scan_certs_on_sni_mismatch: bool,
}

pub enum SessionTicketKeysType {
    /// TLS session ticket key settings.
    SessionTicketKeys(TLSSessionTicketKeys),

    /// Config for fetching TLS session ticket keys via SDS API.
    SessionTicketKeysSDSSecretConfig(SDSSecretConfig),

    /**
    Config for controlling stateless TLS session resumption: setting this to `true` will cause the TLS server to not issue TLS session tickets for the purposes of stateless TLS session resumption.
    If set to false, the TLS server will issue TLS session tickets and encrypt/decrypt them using the keys specified through either [`SessionTicketKeys`][SessionTicketKeysType::SessionTicketKeys] or [`SessionTicketKeysSDSSecretConfig`][SessionTicketKeysType::SessionTicketKeysSDSSecretConfig].
    If this config is set to false and no keys are explicitly configured, the TLS server will issue TLS session tickets and encrypt/decrypt them using an internally-generated and managed key, with the implication that sessions cannot be resumed across hot restarts or on different hosts.
    */
    DisableStatelessSessionResumption(bool),
}

/**
TLS key log configuration.
The key log file format is "format used by NSS for its SSLKEYLOGFILE debugging output" (text taken from openssl man page)
*/
pub struct TLSKeyLog {
    /// The path to save the TLS key log.
    // [(validate.rules).string = {min_len: 1}];
    path: String,
  
    /**
    The local IP address that will be used to filter the connection which should save the TLS key log
    If it is not set, any local IP address  will be matched.
    */
    local_address_range: CIDRRange,
  
    /**
    The remote IP address that will be used to filter the connection which should save the TLS key log
    If it is not set, any remote IP address will be matched.
    */
    remote_address_range: CIDRRange,
}
  
/**
TLS context shared by both client and server TLS contexts.
*/
pub struct CommonTLSContext {
    /// TLS protocol versions, cipher suites etc.
    tls_params: TLSParameters,
  
    /**
    Only a single TLS certificate is supported in client contexts. In server contexts,
    :ref:`Multiple TLS certificates <arch_overview_ssl_cert_select>` can be associated with the
    same context to allow both RSA and ECDSA certificates and support SNI-based selection.

    Only one of `tls_certificates`, `tls_certificate_sds_secret_configs`,
    and `tls_certificate_provider_instance` may be used.
    [#next-major-version: These mutually exclusive fields should ideally be in a oneof, but it's
    not legal to put a repeated field in a oneof. In the next major version, we should rework
    this to avoid this problem.]
    */
    tls_certificates: Vec<TLSCertificate>,
  
    /**
    Configs for fetching TLS certificates via SDS API. Note SDS API allows certificates to be
    fetched/refreshed over the network asynchronously with respect to the TLS handshake.

    The same number and types of certificates as [`tls_certificates`][crate::extensions::transport_sockets.tls.v3.CommonTLSContext.tls_certificates]
    are valid in the the certificates fetched through this setting.

    Only one of `tls_certificates`, `tls_certificate_sds_secret_configs`,
    and `tls_certificate_provider_instance` may be used.
    [#next-major-version: These mutually exclusive fields should ideally be in a oneof, but it's
    not legal to put a repeated field in a oneof. In the next major version, we should rework
    this to avoid this problem.]
    */
    // [(validate.rules).repeated = {max_items: 2}];
    tls_certificate_sds_secret_configs: Vec<SDSSecretConfig>,
  
    /**
    Certificate provider instance for fetching TLS certs.

    Only one of `tls_certificates`, `tls_certificate_sds_secret_configs`, and `tls_certificate_provider_instance` may be used.
    */
    tls_certificate_provider_instance: CertificateProviderPluginInstance,

  
    validation_context_type: ValidationContextType,
    
    /**
    Supplies the list of ALPN protocols that the listener should expose. In
    practice this is likely to be set to one of two values (see the [`codec_type`][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager::codec_type] parameter in the HTTP connection manager for more information):

    - "h2,http/1.1" If the listener is going to support both HTTP/2 and HTTP/1.1.
    - "http/1.1" If the listener is only going to support HTTP/1.1.

    There is no default for this parameter. If empty, Envoy will not expose ALPN.
    */
    alpn_protocols: Vec<String>,
  
    /**
    Custom TLS handshaker. If empty, defaults to native TLS handshaking behaviour.
    */
    custom_handshaker: TypedExtensionConfig,
  
    /// TLS key log configuration
    key_log: TLSKeyLog,
}

pub enum ValidationContextType {
    /// How to validate peer certificates.
    ValidationContext(CertificateValidationContext),

    /**
    Config for fetching validation context via SDS API. Note SDS API allows certificates to be fetched/refreshed over the network asynchronously with respect to the TLS handshake.
    */
    ValidationContextSDSSecretConfig(SDSSecretConfig),

    /**
    Combined certificate validation context holds a default CertificateValidationContext and SDS config. When SDS server returns dynamic CertificateValidationContext, both dynamic and default CertificateValidationContext are merged into a new CertificateValidationContext for validation. This merge is done by Message::MergeFrom(), so dynamic `CertificateValidationContext` overwrites singular fields in default `CertificateValidationContext, and concatenates repeated fields to default `CertificateValidationContext`, and logical OR is applied to boolean fields.
    */
    CombinedValidationContext(CombinedCertificateValidationContext),
}

/**
Config for Certificate provider to get certificates. This provider should allow certificates to be fetched/refreshed over the network asynchronously with respect to the TLS handshake.

DEPRECATED: This message is not currently used, but if we ever do need it, we will want to move it out of CommonTLSContext and into common.proto, similar to the existing
CertificateProviderPluginInstance message.

[#not-implemented-hide:]
*/
pub struct CertificateProvider {
    /**
    Opaque name used to specify certificate instances or types. For example, "ROOTCA" to specify a root-certificate (validation context) or "TLS" to specify a new tls-certificate.
    */
    // [(validate.rules).string = {min_len: 1}];
    name: String,

    /**
    Provider specific config.
    Note: an implementation is expected to dedup multiple instances of the same config to maintain a single certificate-provider instance. The sharing can happen, for example, among multiple clusters or between the tls_certificate and validation_context certificate providers of a cluster.
    This config could be supplied inline or (in future) a named xDS resource.
    */
    config: TypedExtensionConfig
}
  
/**
Similar to CertificateProvider above, but allows the provider instances to be configured on the client side instead of being sent from the control plane.

DEPRECATED: This message was moved outside of CommonTLSContext
and now lives in common.proto.

[#not-implemented-hide:]
*/
pub struct CertificateProviderInstance {
    /**
    Provider instance name. This name must be defined in the client's configuration (e.g., a
    bootstrap file) to correspond to a provider instance (i.e., the same data in the typed_config
    field that would be sent in the CertificateProvider message if the config was sent by the
    control plane). If not present, defaults to "default".

    Instance names should generally be defined not in terms of the underlying provider
    implementation (e.g., "file_watcher") but rather in terms of the function of the
    certificates (e.g., "foo_deployment_identity").
    */
    instance_name: String,

    /**
    Opaque name used to specify certificate instances or types. For example, "ROOTCA" to specify
    a root-certificate (validation context) or "example.com" to specify a certificate for a
    particular domain. Not all provider instances will actually use this field, so the value
    defaults to the empty string.
    */
    certificate_name: String,
}
  
pub struct CombinedCertificateValidationContext {
    /// How to validate peer certificates.
    // [(validate.rules).message = {required: true}];
    default_validation_context: CertificateValidationContext,

    /**
    Config for fetching validation context via SDS API. Note SDS API allows certificates to be
    fetched/refreshed over the network asynchronously with respect to the TLS handshake.
    */
    // [(validate.rules).message = {required: true}];
    validation_context_sds_secret_config: SDSSecretConfig,
}

pub enum OCSPStaplePolicy {
    /**
    OCSP responses are optional. If an OCSP response is absent or expired, the associated certificate will be used for connections without an OCSP staple.
    */
    LenientStapling,

    /**
    OCSP responses are optional. If an OCSP response is absent, the associated certificate will be used without an OCSP staple. If a response is provided but is expired, the associated certificate will not be used for subsequent connections. If no suitable certificate is found, the connection is rejected.
    */
    StrictStapling,

    /**
    OCSP responses are required. Configuration will fail if a certificate is provided without an OCSP response. If a response expires, the associated certificate will not be used connections. If no suitable certificate is found, the connection is rejected.
    */
    MustStaple,
}
