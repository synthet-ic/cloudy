/*!
<https://github.com/grpc/grpc-go/blob/master/dialoptions.go>
*/

use std::{
    task::Context,
    net,
    time::Duration
};

use crate::{
    backoff,
    channelz::Identifier,
    credentials::{
        self,
        insecure,
        Bundle,
        TransportCredentials
    },
    internal::{
        backoff::{self as _backoff, Exponential, Strategy},
        binarylog::Logger,
        transport::ConnectOptions,
        HealthChecker, HealthCheckFunc
    },
    keepalive::ClientParameters,
    resolver::Builder,
    stats
};

fn init() {
    internal.AddGlobalDialOptions = fn(opt ...DialOption) {
        extraDialOptions = append(extraDialOptions, opt...)
    }
    internal.ClearGlobalDialOptions = fn() {
        extraDialOptions = nil
    }
    internal.WithBinaryLogger = with_binary_logger
}

// private
// DialOptions configure a Dial call. DialOptions are set by the DialOption
// values passed to Dial.
pub(crate) struct DialOptions {
    unary_int: UnaryClientInterceptor,
    stream_int: StreamClientInterceptor,

    chain_unary_ints: []UnaryClientInterceptor,
    chain_stream_ints: []StreamClientInterceptor,

    cp: Compressor,
    dc: Decompressor,
    bs: _backoff.Strategy,
    block: bool,
    return_last_error: bool,
    timeout: Duration,
    sc_chan: <-chan ServiceConfig,
    authority: String,
    binary_logger: Logger,
    copts: ConnectOptions,
    call_options: []CallOption,
    channelz_parent_id: Option<Identifier>,
    disable_service_config: bool,
    disable_retry: bool,
    disable_health_check: bool,
    health_check_func: internal.HealthChecker,
    min_connect_timeout: fn() Duration,
    default_service_config: *ServiceConfig // default_service_config is parsed from default_service_config_raw_json.,
    default_service_config_raw_json: *String,
    resolvers: []resolver.Builder,
}

// DialOption configures how we set up the connection.
type DialOption interface {
    apply(*DialOptions)
}

var extraDialOptions []DialOption

// EmptyDialOption does not alter the dial configuration. It can be embedded in
// another structure to build custom dial options.
//
// # Experimental
//
// Notice: This type is EXPERIMENTAL and may be changed or removed in a
// later release.
type EmptyDialOption struct{}

fn (EmptyDialOption) apply(*DialOptions) {}

// funcDialOption wraps a function that modifies DialOptions into an
// implementation of the DialOption interface.
type funcDialOption struct {
    f: fn(*DialOptions),
}

fn (fdo *funcDialOption) apply(do *DialOptions) {
    fdo.f(do)
}

fn new_func_dial_option(f fn(*DialOptions)) *funcDialOption {
    return &funcDialOption{
        f: f,
    }
}

/**
with_write_buffer_size determines how much data can be batched before doing a
write on the wire. The corresponding memory allocation for this buffer will
be twice the size to keep syscalls low. The default value for this buffer is
32KB.

Zero or negative values will disable the write buffer such that each write
will be on underlying connection. Note: A Send call may not directly
translate to a write.
*/
pub fn with_write_buffer_size(s: int) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.WriteBufferSize = s
    })
}

/**
Lets you set the size of read buffer, this determines how much data can be read at most for each read syscall.

The default value for this buffer is 32KB. Zero or negative values will disable read buffer for a connection so data framer can access the underlying conn directly.
*/
pub fn with_read_buffer_size(s: int) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.ReadBufferSize = s
    })
}

/**
Returns a DialOption which sets the value for initial window size on a stream. The lower bound for window size is 64K and any value smaller than that will be ignored.
*/
pub fn with_initial_window_size(s: i32) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.InitialWindowSize = s
    })
}

/**
Returns a DialOption which sets the value for initial window size on a connection. The lower bound for window size is 64K and any value smaller than that will be ignored.
*/
pub fn with_initial_conn_window_size(s: i32) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.InitialConnWindowSize = s
    })
}

/// Returns a DialOption which sets the default CallOptions for calls over the connection.
pub fn with_default_call_options(cos: ...CallOption) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.call_options = append(o.call_options, cos...)
    })
}

/**
Returns a DialOption which sets a codec for message marshaling and
unmarshaling.

Deprecated: use with_default_call_options(ForceCodec(_)) instead.  Will be
supported throughout 1.x.
*/
pub fn with_codec(c Codec) -> DialOption {
    return with_default_call_options(CallCustomCodec(c))
}

/**
Configures the ClientConn to use the provided ConnectParams for creating and maintaining connections to servers.

The backoff configuration specified as part of the ConnectParams overrides all defaults specified in <https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md>. Consider using the backoff.DefaultConfig as a base, in cases where you want to override only a subset of the backoff configuration.
*/
pub fn with_connect_params(p: ConnectParams) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.bs = Exponential { Config: p.Backoff }
        o.min_connect_timeout = || p.MinConnectTimeout;
    })
}

/**
Sets the backoff strategy used for connectRetryNum after a failed connection attempt.

This can be exported if arbitrary backoff strategies are allowed by gRPC.
*/
fn with_backoff(bs: Strategy) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.bs = bs
    })
}

/**
with_block returns a DialOption which makes callers of Dial block until the
underlying connection is up. Without this, Dial returns immediately and
connecting the server happens in background.
*/
pub fn with_block() -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.block = true
    })
}

/**
with_return_connection_error returns a DialOption which makes the client connection
return a string containing both the last connection error that occurred and
the context.DeadlineExceeded error.
Implies with_block()

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a
later release.
*/
pub fn with_return_connection_error() -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.block = true
        o.return_last_error = true
    })
}

/**
with_no_proxy returns a DialOption which disables the use of proxies for this
ClientConn. This is ignored if with_dialer or with_context_dialer are used.

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a
later release.
*/
pub fn with_no_proxy() -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.UseProxy = false
    })
}

/**
Returns a DialOption which configures a connection level security credentials (e.g., TLS/SSL). This should not be used together with with_credentials_bundle.
*/
pub fn with_transport_credentials(creds: TransportCredentials) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.TransportCredentials = creds
    })
}

/// Returns a DialOption which sets credentials and places auth state on each outbound RPC.
pub fn with_per_rpc_credentials(creds credentials.PerRPCCredentials) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.PerRPCCredentials = append(o.copts.PerRPCCredentials, creds)
    })
}

/**
Returns a DialOption to set a credentials bundle for the ClientConn.WithCreds. This should not be used together with with_transport_credentials.

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a
later release.
*/
pub fn with_credentials_bundle(b credentials.Bundle) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.CredsBundle = b
    })
}

/**
Returns a DialOption that sets a dialer to create connections. If fail_on_non_temp_dial_error() is set to true, and an error is returned by f, gRPC checks the error's Temporary() method to decide if it should try to reconnect to the network address.
*/
pub fn with_context_dialer(f fn(context.Context, String) (net.Conn, error)) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.Dialer = f
    })
}

fn init() {
    internal.WithHealthCheckFunc = with_health_check_func
}

/// Returns a DialOption that specifies the stats handler for all the RPCs and underlying network connections in this ClientConn.
pub fn with_stats_handler(h stats.Handler) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        if h == nil {
            logger.Error("ignoring nil parameter in grpc.with_stats_handler ClientOption")
            // Do not allow a nil stats handler, which would otherwise cause
            // panics.
            return
        }
        o.copts.StatsHandlers = append(o.copts.StatsHandlers, h)
    })
}

/// Returns a DialOption that specifies the binary logger for this ClientConn.
fn with_binary_logger(bl: Logger) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.binary_logger = bl
    })
}

/**
Returns a DialOption that specifies if gRPC fails on non-temporary dial errors. If f is true, and dialer returns a non-temporary error, gRPC will fail the connection to the network address and won't try to reconnect. The default value of fail_on_non_temp_dial_error is false.

fail_on_non_temp_dial_error only affects the initial dial, and does not do
anything useful unless you are also using with_block().

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a
later release.
*/
pub fn fail_on_non_temp_dial_error(f bool) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.fail_on_non_temp_dial_error = f
    })
}

/// Returns a DialOption that specifies a user agent string for all the RPCs.
pub fn with_user_agent(s String) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.UserAgent = s
    })
}

/// Returns a DialOption that specifies keepalive parameters for the client transport.
pub fn with_keepalive_params(kp: ClientParameters) -> DialOption {
    if kp.Time < internal.KeepaliveMinPingTime {
        logger.Warningf("Adjusting keepalive ping interval to minimum period of %v", internal.KeepaliveMinPingTime)
        kp.Time = internal.KeepaliveMinPingTime
    }
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.KeepaliveParams = kp
    })
}

/// Returns a DialOption that specifies the interceptor for unary RPCs.
pub fn with_unary_interceptor(f: UnaryClientInterceptor) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.unary_int = f
    })
}

/**
Returns a DialOption that specifies the chained interceptor for unary RPCs. The first interceptor will be the outer most, while the last interceptor will be the inner most wrapper around the real call.
All interceptors added by this method will be chained, and the interceptor defined by with_unary_interceptor will always be prepended to the chain.
*/
pub fn with_chain_unary_interceptor(interceptors: ...UnaryClientInterceptor) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.chain_unary_ints = append(o.chain_unary_ints, interceptors...)
    })
}

/// Returns a DialOption that specifies the interceptor for streaming RPCs.
pub fn with_stream_interceptor(f: StreamClientInterceptor) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.stream_int = f
    })
}

/**
Returns a DialOption that specifies the chained interceptor for streaming RPCs. The first interceptor will be the outer most, while the last interceptor will be the inner most wrapper around the real call.
All interceptors added by this method will be chained, and the interceptor defined by with_stream_interceptor will always be prepended to the chain.
*/
pub fn with_chain_stream_interceptor(interceptors: ...StreamClientInterceptor) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.chainStreamInts = append(o.chainStreamInts, interceptors...)
    })
}

/**
Returns a DialOption that specifies the value to be used as the :authority pseudo-header and as the server name in authentication handshake.
*/
pub fn with_authority(a: String) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.authority = a
    })
}

/**
Returns a DialOption that specifies the channelz ID of current ClientConn's parent. This function is used in nested channel creation
(e.g. grpclb dial).

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a later release.
*/
pub fn with_channelz_parent_id(id *channelz.Identifier) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.channelz_parent_id = id
    })
}

/**
Returns a DialOption that causes gRPC to ignore any service config provided by the resolver and provides a hint to the resolver to not fetch service configs.

Note that this dial option only disables service config from resolver. If default service config is provided, gRPC will use the default service config.
*/
pub fn with_disable_service_config() -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.disable_service_config = true
    })
}

/**
Returns a DialOption that configures the default service config, which will be used in cases where:

1. with_disable_service_config is also used, or

2. The name resolver does not provide a service config or provides an
invalid service config.

The parameter s is the JSON representation of the default service config.
For more information about service configs, see:
<https://github.com/grpc/grpc/blob/master/doc/service_config.md>
For a simple example of usage, see: examples/features/load_balancing/client/main.go
*/
pub fn with_default_service_config(s: String) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.default_service_config_raw_json = &s
    })
}

/**
Returns a DialOption that disables retries, even if the service config enables them.  This does not impact transparent retries, which will happen automatically if no data is written to the wire or if the RPC is unprocessed by the remote server.
*/
pub fn with_disable_retry() -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.disable_retry = true
    })
}

/**
Returns a DialOption that specifies the maximum
(uncompressed) size of header list that the client is prepared to accept.
*/
pub fn with_max_header_list_size(s: u32) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.copts.MaxHeaderListSize = &s
    })
}

/**
Disables the LB channel health checking for all SubConns of this ClientConn.

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a
later release.
*/
pub fn with_disable_health_check() -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.disable_health_check = true
    })
}

/**
Replaces the default health check function with the provided one. It makes tests easier to change the health check function.

For testing purpose only.
*/
fn with_health_check_func(f: HealthChecker) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.health_check_func = f
    })
}

fn default_dial_options() -> DialOptions {
    return DialOptions {
        health_check_func: internal.HealthCheckFunc,
        copts: transport.ConnectOptions{
            WriteBufferSize: defaultWriteBufSize,
            ReadBufferSize:  defaultReadBufSize,
            UseProxy:        true,
        },
    }
}

/**
Specifies the function that clientconn uses to get minConnectDeadline. This can be used to make connection attempts happen faster/slower.

For testing purpose only.
*/
fn with_min_connect_deadline(f: fn() -> Duration) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.min_connect_timeout = f
    })
}

/**
Allows a list of resolver implementations to be registered locally with the ClientConn without needing to be globally registered via resolver.Register.  They will be matched against the scheme used for the current Dial only, and will take precedence over the global registry.

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a
later release.
*/
pub fn with_resolvers(rs: ...Builder) -> DialOption {
    return new_func_dial_option(fn(o *DialOptions) {
        o.resolvers = append(o.resolvers, rs...)
    })
}
