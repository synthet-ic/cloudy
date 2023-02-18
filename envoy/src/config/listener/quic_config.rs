/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/listener/v3/quic_config.proto>
*/

use std::time::Duration;

use crate::config::core::{
    base::RuntimeFeatureFlag,
    extension::TypedExtensionConfig,
    protocol
};

/// Configuration specific to the UDP QUIC listener.
pub struct QUICProtocolOptions {
    quic_protocol_options: protocol::QUICProtocolOptions,

    /**
    Maximum number of milliseconds that connection will be alive when there is no network activity.

    If it is less than 1ms, Envoy will use 1ms. 300000ms if not specified.
    */
    idle_timeout: Duration,

    /**
    Connection timeout in milliseconds before the crypto handshake is finished.

    If it is less than 5000ms, Envoy will use 5000ms. 20000ms if not specified.
    */
    crypto_handshake_timeout: Duration,

    /**
    Runtime flag that controls whether the listener is enabled or not. If not specified, defaults to enabled.
    */
    enabled: RuntimeFeatureFlag,

    /**
    A multiplier to number of connections which is used to determine how many packets to read per event loop. A reasonable number should allow the listener to process enough payload but not starve TCP and other UDP sockets and also prevent long event loop duration.
    The default value is 32. This means if there are N QUIC connections, the total number of packets to read in each read event will be 32 * N.
    The actual number of packets to read in total by the UDP listener is also
    bound by 6000, regardless of this field or how many connections there are.

    [(validate.rules).u32 = {gte: 1}];
    */
    packets_to_read_to_connection_count_ratio: u32,
        

    /**
    Configure which implementation of `quic::QuicCryptoClientStreamBase` to be used for this listener.
    If not specified the [QUICHE default one configured by][crate::extensions::quic.crypto_stream::CryptoServerStreamConfig] will be used.
    [#extension-category: envoy.quic.server.crypto_stream]
    */
    crypto_stream_config: TypedExtensionConfig,

    /**
    Configure which implementation of `quic::ProofSource` to be used for this listener.
    If not specified the [default one configured by][crate::extensions::quic.proof_source::ProofSourceConfig] will be used.
    [#extension-category: envoy.quic.proof_source]
    */
    proof_source_config: TypedExtensionConfig,

    /**
    Config which implementation of `quic::ConnectionIdGeneratorInterface` to be used for this listener.
    If not specified the [default one configured by][crate::extensions::quic.connection_id_generator::DeterministicConnectionIdGeneratorConfig] will be used.
    [#extension-category: envoy.quic.connection_id_generator]
    */
    connection_id_generator_config: TypedExtensionConfig,
}
