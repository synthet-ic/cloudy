/*!
<https://github.com/grpc/grpc-go/blob/master/connectivity/connectivity.go>
*/

use crate::grpclog;

/**
State indicates the state of connectivity.
It can be the state of a ClientConn or SubConn.
*/
pub enum State {
    /// Idle indicates the ClientConn is idle.
    Idle,
    /// Connecting indicates the ClientConn is connecting.
    Connecting,
    /// Ready indicates the ClientConn is ready for work.
    Ready,
    /// TransientFailure indicates the ClientConn has seen a failure but expects to recover.
    TransientFailure,
    /// Shutdown indicates the ClientConn has started shutting down.
    Shutdown
}

/**
ServingMode indicates the current mode of operation of the server.

Only xDS enabled gRPC servers currently report their serving mode.
*/
pub enum ServingMode {
    /// Server is starting up.
    Starting,
    /// Server contains all required configuration and is serving RPCs.
    Serving,
    /// Server is not accepting new connections. Existing connections will be closed gracefully, allowing in-progress RPCs to complete. A server enters this mode when it does not contain the required configuration to serve RPCs.
    NotServing
}
