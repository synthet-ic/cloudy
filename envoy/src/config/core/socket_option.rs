/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/socket_option.proto>
*/

/**
Generic socket option message. This would be used to set socket options that might not exist in upstream kernels or precompiled Envoy binaries.

For example:

```json
{
  "description": "support tcp keep alive",
  "state": 0,
  "level": 1,
  "name": 9,
  "int_value": 1,
}
```

1 means SOL_SOCKET and 9 means SO_KEEPALIVE on Linux.
With the above configuration, [TCP Keep-Alives](https://www.freesoft.org/CIE/RFC/1122/114.htm>) can be enabled in socket with Linux, which can be used in
[listener's][crate::config::listener::listener::Listener::socket_options] or
[admin's][crate::config::bootstrap::Admin::socket_options] socket_options etc.

It should be noted that the name or level may have different values on different platforms.
*/
pub struct SocketOption {
    /**
    An optional name to give this socket option for debugging, etc.
    Uniqueness is not required and no special meaning is assumed.
    */
    description: String,

    /// Corresponding to the level value passed to setsockopt, such as IPPROTO_TCP
    level: i64,

    /// The numeric name as passed to setsockopt
    name: i64,

    value: Value,

    /// The state in which the option will be applied. When used in BindConfig
    /// STATE_PREBIND is currently the only valid value.
    // [(validate.rules).enum = {defined_only: true}];
    state: SocketState
}

pub enum Value {
    // option (validate.required) = true;

    /// Because many sockopts take an int value.
    IntValue(i64),

    /// Otherwise it's a byte buffer.
    BufValue(Vec<u8>)
}

pub enum SocketState {
    /// Socket options are applied after socket creation but before binding the socket to a port
    Prebind,

    /// Socket options are applied after binding the socket to a port but before calling listen()
    Bound,

    /// Socket options are applied after calling listen()
    Listening
}

pub struct SocketOptionsOverride {
    socket_options: Vec<SocketOption>
}
