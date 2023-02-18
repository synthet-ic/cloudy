/*!
<https://github.com/etcd-io/etcd/blob/main/client/v3/config.go>
*/

use std::{
    task::Context,
    time::Duration
};

pub struct Config {
    /// `endpoints` is a list of URLs.
    endpoints: Vec<String>,

    /**
    `auto_sync_interval` is the interval to update endpoints with its latest members.
    0 disables auto-sync. By default auto-sync is disabled.
    */
    auto_sync_interval: Duration,

    /// dial_timeout is the timeout for failing to establish a connection.
    dial_timeout: Duration,

    /**
    dial_keep_alive_time is the time after which client pings the server to see if
    transport is alive.
    */
    dial_keep_alive_time: Duration,

    /**
    dial_keep_alive_timeout is the time that the client waits for a response for the
    keep-alive probe. If the response is not received in this time, the connection is closed.
    */
    dial_keep_alive_timeout: Duration,

    /**
    max_call_send_msg_size is the client-side request send limit in bytes.
    If 0, it defaults to 2.0 MiB (2 * 1024 * 1024).
    Make sure that "max_call_send_msg_size" < server-side default send/recv limit.
    ("--max-request-bytes" flag to etcd or "embed.Config.MaxRequestBytes").
    */
    max_call_send_msg_size: i32,

    /**
    `max_call_recv_msg_size` is the client-side response receive limit.
    If 0, it defaults to "math.MaxInt32", because range response can
    easily exceed request send limits.
    Make sure that "max_call_recv_msg_size" >= server-side default send/recv limit.
    ("--max-request-bytes" flag to etcd or "embed.Config.MaxRequestBytes").
    */
    max_call_recv_msg_size: i32,

    /// `tls` holds the client secure credentials, if any.
    tls: Option<tls::Config>,

    /// `username` is a user name for authentication.
    username: String,

    /// `password` is a password for authentication.
    password: String,

    /// `reject_old_cluster` when set will refuse to create a client against an outdated cluster.
    reject_old_cluster: bool,

    /**
    `dial_options` is a list of dial options for the grpc client (e.g., for interceptors).
    For example, pass "grpc::WithBlock()" to block until the underlying connection is up.
    Without this, Dial returns immediately and connecting the server happens in background.
    */
    dial_options: Vec<grpc::DialOption>,

    /**
    Context is the default client context; it can be used to cancel grpc dial out and
    other operations that do not have an explicit context.
    */
    context: Context,

    /**
    Logger sets client-side logger.
    If `None`, fallback to building log_config.
    */
    logger: Option<zap::Logger>,

    /**
    `log_config` configures client-side logger.
    If `None`, use the default logger.
    TODO: configure gRPC logger
    */
    log_config: Option<zap::Config>,

    /// permit_without_stream when set will allow client to send keepalive pings to server without any active streams(RPCs).
    permit_without_stream: bool

    // TODO: support custom balancer picker
}
