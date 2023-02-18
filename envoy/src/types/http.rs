
/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/listener/v3/listener.proto>
*/

pub mod path_transformation;

pub enum CodecClientType {
    HTTP1,
    HTTP2,

    /**
    [#not-implemented-hide:] QUIC implementation is not production ready yet. Use this enum with caution to prevent accidental execution of QUIC code. I.e. `!= HTTP2` is no longer sufficient to distinguish HTTP1 and HTTP2 traffic.
    */
    HTTP3
}
