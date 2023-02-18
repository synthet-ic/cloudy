/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/extensions/transport_sockets/tls/v3/common.proto>
*/

type Any = String;

use crate::{
    config::core::{
        base::{DataSource, WatchedDirectory},
        extension::TypedExtensionConfig
    },
    types::matcher::string::StringMatcher
};

pub struct TLSParameters {
    /**
    Minimum TLS protocol version. By default, it's `TLSv1_2` for both clients and servers.

    TLS protocol versions below TLSv1_2 require setting compatible ciphers with the `cipher_suites` setting as the default ciphers no longer include compatible ciphers.

    > attention: Using TLS protocol versions below TLSv1_2 has serious security considerations and risks.

    [(validate.rules).enum = {defined_only: true}];
    */
    tls_minimum_protocol_version: TLSProtocol,

    /**
    Maximum TLS protocol version. By default, it's `TLSv1_2` for clients and `TLSv1_3` for servers.

    [(validate.rules).enum = {defined_only: true}];
    */
    tls_maximum_protocol_version: TLSProtocol,

    /**
    If specified, the TLS listener will only support the specified `cipher list
    <https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration>`_
    when negotiating TLS 1.0-1.2 (this setting has no effect when negotiating TLS 1.3).

    If not specified, a default list will be used. Defaults are different for server (downstream) and client (upstream) TLS configurations.
    Defaults will change over time in response to security considerations; If you care, configure it instead of using the default.

    In non-FIPS builds, the default server cipher list is:

    ```text
    [ECDHE-ECDSA-AES128-GCM-SHA256|ECDHE-ECDSA-CHACHA20-POLY1305]
    [ECDHE-RSA-AES128-GCM-SHA256|ECDHE-RSA-CHACHA20-POLY1305]
    ECDHE-ECDSA-AES256-GCM-SHA384
    ECDHE-RSA-AES256-GCM-SHA384
    ```

    In builds using :ref:`BoringSSL FIPS <arch_overview_ssl_fips>`, the default server cipher list is:

    ```text
    ECDHE-ECDSA-AES128-GCM-SHA256
    ECDHE-RSA-AES128-GCM-SHA256
    ECDHE-ECDSA-AES256-GCM-SHA384
    ECDHE-RSA-AES256-GCM-SHA384
    ```

    In non-FIPS builds, the default client cipher list is:

    ```text
    [ECDHE-ECDSA-AES128-GCM-SHA256|ECDHE-ECDSA-CHACHA20-POLY1305]
    [ECDHE-RSA-AES128-GCM-SHA256|ECDHE-RSA-CHACHA20-POLY1305]
    ECDHE-ECDSA-AES256-GCM-SHA384
    ECDHE-RSA-AES256-GCM-SHA384
    ```

    In builds using :ref:`BoringSSL FIPS <arch_overview_ssl_fips>`, the default client cipher list is:

    ```text
    ECDHE-ECDSA-AES128-GCM-SHA256
    ECDHE-RSA-AES128-GCM-SHA256
    ECDHE-ECDSA-AES256-GCM-SHA384
    ECDHE-RSA-AES256-GCM-SHA384
    ```
    */
    cipher_suites: Vec<String>,

    /**
    If specified, the TLS connection will only support the specified ECDH curves. If not specified, the default curves will be used.

    In non-FIPS builds, the default curves are:

    ```text
    X25519
    P-256
    ```

    In builds using :ref:`BoringSSL FIPS <arch_overview_ssl_fips>`, the default curve is:

    ```text
    P-256
    ```
    */
    ecdh_curves: Vec<String>
}

pub enum TLSProtocol {
    /// Envoy will choose the optimal TLS version.
    Auto,

    // TLS 1.0
    TLSv1_0,

    // TLS 1.1
    TLSv1_1,

    // TLS 1.2
    TLSv1_2,

    // TLS 1.3
    TLSv1_3,
}

/**
BoringSSL private key method configuration. The private key methods are used for external (potentially asynchronous) signing and decryption operations. Some use cases for private key methods would be TPM support and TLS acceleration.
*/
pub struct PrivateKeyProvider {
    /**
    Private key method provider name. The name must match a supported private key method provider type.

    [(validate.rules).string = {min_len: 1}];
    */
    provider_name: String,

    /// Private key method provider specific configuration.
    config_type: ConfigType
}

pub enum ConfigType {
    TypedConfig(Any)
}

pub struct TLSCertificate {
    /**
    The TLS certificate chain.

    If `certificate_chain` is a filesystem path, a watch will be added to the
    parent directory for any file moves to support rotation. This currently
    only applies to dynamic secrets, when the `TLSCertificate` is delivered via
    SDS.
    */
    certificate_chain: DataSource,

    /**
    The TLS private key.

    If `private_key` is a filesystem path, a watch will be added to the parent directory for any file moves to support rotation. This currently only applies to dynamic secrets, when the `TLSCertificate` is delivered via SDS.

    [(udpa.annotations.sensitive) = true];
    */
    private_key: DataSource,

    /**
    `Pkcs12` data containing TLS certificate, chain, and private key.

    If `pkcs12` is a filesystem path, the file will be read, but no watch will be added to the parent directory, since `pkcs12` isn't used by SDS.
    This field is mutually exclusive with `certificate_chain`, `private_key` and `private_key_provider`.
    This can't be marked as `oneof` due to API compatibility reasons. Setting
    both :ref:`private_key <Self.private_key>`,
    :ref:`certificate_chain <Self.certificate_chain>`,
    or :ref:`private_key_provider <Self.private_key_provider>`
    and :ref:`pkcs12 <Self.pkcs12>`
    fields will result in an error. Use :ref:`password
    <Self.password>`
    to specify the password to unprotect the `PKCS12` data, if necessary.

    [(udpa.annotations.sensitive) = true];
    */
    pkcs12: DataSource,

    /**
    If specified, updates of file-based `certificate_chain` and `private_key` sources will be triggered by this watch. The certificate/key pair will be read together and validated for atomic read consistency (i.e. no intervening modification occurred between cert/key read, verified by file hash comparisons). This allows explicit control over the path watched, by default the parent directories of the filesystem paths in `certificate_chain` and `private_key` are watched if this field is not specified. This only applies when a `TLSCertificate` is delivered by SDS with references to filesystem paths. See the :ref:`SDS key rotation <sds_key_rotation>` documentation for further details.
    */
    watched_directory: WatchedDirectory,

    /**
    BoringSSL private key method provider. This is an alternative to :ref:`private_key
    <Self.private_key>` field. This can't be
    marked as `oneof` due to API compatibility reasons. Setting both :ref:`private_key
    <Self.private_key>` and
    :ref:`private_key_provider
    <Self.private_key_provider>` fields will result in an
    error.
    */
    private_key_provider: PrivateKeyProvider,

    /**
    The password to decrypt the TLS private key. If this field is not set, it is assumed that the
    TLS private key is not password encrypted.

    [(udpa.annotations.sensitive) = true];
    */
    password: DataSource,

    /**
    The OCSP response to be stapled with this certificate during the handshake.
    The response must be DER-encoded and may only be  provided via `filename` or
    `inline_bytes`. The response may pertain to only one certificate.
    */
    ocsp_staple: DataSource,

    /// [#not-implemented-hide:]
    signed_certificate_timestamp: Vec<DataSource>,
}

pub struct TLSSessionTicketKeys {
    /**
    Keys for encrypting and decrypting TLS session tickets. The first key in the array contains the key to encrypt all new sessions created by this context.
    All keys are candidates for decrypting received tickets. This allows for easy rotation of keys by, for example, putting the new key first, and the previous key second.

    If [`SessionTicketKeys`][crate::extensions::transport_sockets::tls::tls::SessionTicketKeysType::SessionTicketKeys] is not specified, the TLS library will still support resuming sessions via tickets, but it will use an internally-generated and managed key, so sessions cannot be resumed across hot restarts or on different hosts.

    Each key must contain exactly 80 bytes of cryptographically-secure random data. For example, the output of `openssl rand 80`.

    > attention: Using this feature has serious security considerations and risks. Improper handling of keys may result in loss of secrecy in connections, even if ciphers supporting perfect forward secrecy are used. See <https://www.imperialviolet.org/2013/06/27/botchingpfs.htm>l for some discussion. To minimize the risk, you must:
    > - Keep the session ticket keys at least as secure as your TLS certificate private keys
    > - Rotate session ticket keys at least daily, and preferably hourly
    > - Always generate keys using a cryptographically-secure random data source
    
    [(validate.rules).repeated = {min_items: 1}, (udpa.annotations.sensitive) = true];
    */
    keys: Vec<DataSource>
}

/**
Indicates a certificate to be obtained from a named CertificateProvider plugin instance.
The plugin instances are defined in the client's bootstrap file.
The plugin allows certificates to be fetched/refreshed over the network asynchronously with respect to the TLS handshake.
*/
pub struct CertificateProviderPluginInstance {
    /**
    Provider instance name. If not present, defaults to "default".

    Instance names should generally be defined not in terms of the underlying provider implementation (e.g., "file_watcher") but rather in terms of the function of the certificates (e.g., "foo_deployment_identity").
    */
    instance_name: String,

    /**
    Opaque name used to specify certificate instances or types. For example, "ROOTCA" to specify a root-certificate (validation context) or "example.com" to specify a certificate for a particular domain. Not all provider instances will actually use this field, so the value defaults to the empty string.
    */
    certificate_name: String,
}

/// Matcher for subject alternative names, to match both type and value of the SAN.
pub struct SubjectAltNameMatcher {
    /// Specification of type of SAN. Note that the default enum value is an invalid choice.
    // [(validate.rules).enum = {defined_only: true not_in: 0}];
    san_type: SanType,

    /// Matcher for SAN value.
    // [(validate.rules).message = {required: true}];
    matcher: StringMatcher
}

/**
Indicates the choice of GeneralName as defined in section 4.2.1.5 of RFC 5280 to match against.
*/
pub enum SanType {
    SanTypeUnspecified,
    Email,
    DNS,
    URI,
    IPAddress,
}

pub struct CertificateValidationContext {
    /**
    TLS certificate data containing certificate authority certificates to use in verifying a presented peer certificate (e.g. server certificate for clusters or client certificate for listeners). If not specified and a peer certificate is presented it will not be verified. By default, a client certificate is optional, unless one of the additional options ([`require_client_certificate`][crate::extensions::transport_sockets::tls::tls::DownstreamTLSContext::require_client_certificate], [`verify_certificate_spki`][Self::verify_certificate_spki], [`verify_certificate_hash`][Self::verify_certificate_hash], or [`match_typed_subject_alt_names`][Self::match_typed_subject_alt_names]) is also specified.

    It can optionally contain certificate revocation lists, in which case Envoy will verify that the presented peer certificate has not been revoked by one of the included CRLs. Note that if a CRL is provided for any certificate authority in a trust chain, a CRL must be provided for all certificate authorities in that chain. Failure to do so will result in verification failure for both revoked and unrevoked certificates from that chain.
    The behaviour of requiring all certificates to contain CRLs if any do can be altered by setting [`only_verify_leaf_cert_crl`][Self::only_verify_leaf_cert_crl] `true`. If set to true, only the final certificate in the chain undergoes CRL verification.

    See :ref:`the TLS overview <arch_overview_ssl_enabling_verification>` for a list of common system CA locations.

    If `trusted_ca` is a filesystem path, a watch will be added to the parent directory for any file moves to support rotation. This currently only applies to dynamic secrets, when the `Self` is delivered via SDS.

    X509_V_FLAG_PARTIAL_CHAIN is set by default, so non-root/intermediate ca certificate in `trusted_ca` can be treated as trust anchor as well. It allows verification with building valid partial chain instead of a full chain.

    Only one of `trusted_ca` and `ca_certificate_provider_instance` may be specified.

    > #next-major-version: This field and watched_directory below should ideally be moved into a
    separate sub-message, since there's no point in specifying the latter field without this one.

    [(udpa.annotations.field_migrate).oneof_promotion = "ca_cert_source"];
    */
    trusted_ca: DataSource,

    /**
    Certificate provider instance for fetching TLS certificates.

    Only one of `trusted_ca` and `ca_certificate_provider_instance` may be specified.

    [(udpa.annotations.field_migrate).oneof_promotion = "ca_cert_source"];
    */
    ca_certificate_provider_instance: CertificateProviderPluginInstance,

    /**
    If specified, updates of a file-based `trusted_ca` source will be triggered by this watch. This allows explicit control over the path watched, by default the parent directory of the filesystem path in `trusted_ca` is watched if this field is not specified. This only applies when a `CertificateValidationContext` is delivered by SDS with references to filesystem paths. See the :ref:`SDS key rotation <sds_key_rotation>` documentation for further details.
    */
    watched_directory: WatchedDirectory,

    /**
    An optional list of base64-encoded SHA-256 hashes. If specified, Envoy will verify that the SHA-256 of the DER-encoded Subject Public Key Information (SPKI) of the presented certificate matches one of the specified values.

    A base64-encoded SHA-256 of the Subject Public Key Information (SPKI) of the certificate can be generated with the following command:

    ```sh
    $ openssl x509 -in path/to/client.crt -noout -pubkey
        | openssl pkey -pubin -outform DER
        | openssl dgst -sha256 -binary
        | openssl enc -base64
    NvqYIYSbgK2vCJpQhObf77vv+bQWtc5ek5RIOwPiC9A=
    ```

    This is the format used in HTTP Public Key Pinning.

    When both:
    [`verify_certificate_hash`][Self::verify_certificate_hash] and
    [`verify_certificate_spki`][Self::verify_certificate_spki] are specified, a hash matching value from either of the lists will result in the certificate being accepted.

    > attention: This option is preferred over [`verify_certificate_hash`][Self::verify_certificate_hash], because SPKI is tied to a private key, so it doesn't change when the certificate is renewed using the same private key.
    
    [(validate.rules).repeated = {items {string {min_len: 44 max_bytes: 44}}}];
    */
    verify_certificate_spki: Vec<String>,
        

    /**
    An optional list of hex-encoded SHA-256 hashes. If specified, Envoy will verify that the SHA-256 of the DER-encoded presented certificate matches one of the specified values.

    A hex-encoded SHA-256 of the certificate can be generated with the following command:

    ```sh
    $ openssl x509 -in path/to/client.crt -outform DER | openssl dgst -sha256 | cut -d" " -f2
    df6ff72fe9116521268f6f2dd4966f51df479883fe7037b39f75916ac3049d1a
    ```

    A long hex-encoded and colon-separated SHA-256 (a.k.a. "fingerprint") of the certificate can be generated with the following command:

    ```sh
    $ openssl x509 -in path/to/client.crt -noout -fingerprint -sha256 | cut -d"=" -f2
    DF:6F:F7:2F:E9:11:65:21:26:8F:6F:2D:D4:96:6F:51:DF:47:98:83:FE:70:37:B3:9F:75:91:6A:C3:04:9D:1A
    ```

    Both of those formats are acceptable.

    When both: [`verify_certificate_hash`][Self::verify_certificate_hash] and
    [`verify_certificate_spki`][Self::verify_certificate_spki] are specified, a hash matching value from either of the lists will result in the certificate being accepted.

    [(validate.rules).repeated = {items {string {min_len: 64 max_bytes: 95}}}];
    */
    verify_certificate_hash: Vec<String>,
        

    /**
    An optional list of Subject Alternative name matchers. If specified, Envoy will verify that the Subject Alternative Name of the presented certificate matches one of the specified matchers.
    The matching uses "any" semantics, that is to say, the SAN is verified if at least one matcher is matched.

    When a certificate has wildcard DNS SAN entries, to match a specific client, it should be configured with exact match type in the [string matcher][crate::types::matcher::string::StringMatcher].
    For example if the certificate has "\*.example.com" as DNS SAN entry, to allow only "api.example.com", it should be configured as shown below.

    ```yaml
    match_typed_subject_alt_names:
    - san_type: DNS
      matcher:
        exact: "api.example.com"
    ```

    > ATTENTION: Subject Alternative Names are easily spoofable and verifying only them is insecure,  therefore this option must be used together with :ref:`trusted_ca <Self::trusted_ca>`.
    */
    match_typed_subject_alt_names: Vec<SubjectAltNameMatcher>,

    /// [#not-implemented-hide:] Must present signed certificate time-stamp.
    require_signed_certificate_timestamp: bool,

    /**
    An optional [certificate revocation list](https://en.wikipedia.org/wiki/Certificate_revocation_list) (in PEM format). If specified, Envoy will verify that the presented peer certificate has not been revoked by this CRL. If this DataSource contains multiple CRLs, all of them will be used. Note that if a CRL is provided for any certificate authority in a trust chain, a CRL must be provided for all certificate authorities in that chain. Failure to do so will result in verification failure for both revoked and unrevoked certificates from that chain. This default behaviour can be altered by setting [`only_verify_leaf_cert_crl`][Self::only_verify_leaf_cert_crl] to `true`.
    */
    crl: DataSource,

    /// If specified, Envoy will not reject expired certificates.
    allow_expired_certificate: bool,

    /// Certificate trust chain verification mode.
    // [(validate.rules).enum = {defined_only: true}];
    trust_chain_verification: TrustChainVerification,

    /**
    The configuration of an extension specific certificate validator.
    If specified, all validation is done by the specified validator, and the behaviour of all other validation settings is defined by the specifie validator (and may be entirely ignored, unused, and unvalidated).
    Refer to the documentation for the specified validator. If you do not want a custom validation algorithm, do not set this field.
    [#extension-category: envoy.tls.cert_validator]
    */
    custom_validator_config: TypedExtensionConfig,

    /**
    If this option is set to true, only the certificate at the end of the certificate chain will be subject to validation by :ref:`CRL <Self::crl>`.
    */
    only_verify_leaf_cert_crl: bool,

    /**
    Defines maximum depth of a certificate chain accepted in verification, the default limit is 100, though this can be system-dependent.
    This number does not include the leaf, so a depth of 1 allows the leaf and one CA certificate. If a trusted issuer appears in the chain, but in a depth larger than configured, the certificate validation will fail.
    See [BoringSSL SSL_CTX_set_verify_depth](https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#SSL_CTX_set_verify_depth)
    If you use OpenSSL, its behaviour is different from BoringSSL, this will define a limit on the number of certificates between the end-entity and trust-anchor certificates.
    Neither the end-entity nor the trust-anchor certificates count against depth.
    See [OpenSSL SSL set_verify_depth](https://www.openssl.org/docs/man1.1.1/man3/SSL_CTX_set_verify_depth.html).
    Trusted issues are specified by setting [`trusted_ca`][Self::trusted_ca].

    [(validate.rules).u32 = {lte: 100}];
    */
    max_verify_depth: u32
}

/// Peer certificate verification mode.
pub enum TrustChainVerification {
    /// Perform default certificate verification (e.g., against CA / verification lists)
    VerifyTrustChain,

    /**
    Connections where the certificate fails verification will be permitted.
    For HTTP connections, the result of certificate verification can be used in route matching. (
    see :[`validated`][crate::config::route::RouteMatch.TLSContextMatchOptions.validated]).
    */
    AcceptUntrusted,
}
