/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/listener/v3/udp_listener_config.proto>
*/

use crate::config::core::{
    extension::TypedExtensionConfig,
    udp_socket_config::UDPSocketConfig,
};
use super::quic_config::QUICProtocolOptions;

pub struct UDPListenerConfig {
    /**
    UDP socket configuration for the listener. The default for
    [`prefer_gro`][crate::config::core::UDPSocketConfig.prefer_gro] is false for listener sockets. If receiving a large amount of datagrams from a small number of sources, it may be worthwhile to enable this option after performance testing.
    */
    downstream_socket_config: UDPSocketConfig,

    /**
    Configuration for QUIC protocol. If empty, QUIC will not be enabled on this listener. Set to the default object to enable QUIC without modifying any additional options.
    */
    quic_options: QUICProtocolOptions,

    /**
    Configuration for the UDP packet writer. If empty, HTTP/3 will use GSO if available ([`UdpDefaultWriterFactory`][crate::extensions::udp_packet_writer::UdpGsoBatchWriterFactory>`) or the default kernel sendmsg if not, (:ref:`UdpDefaultWriterFactory <crate::extensions::udp_packet_writer::UdpDefaultWriterFactory]) and raw UDP will use kernel sendmsg.
    [#extension-category: envoy.udp_packet_writer]
    */
    udp_packet_packet_writer_config: TypedExtensionConfig,
}

pub struct ActiveRawUDPListenerConfig {
}
