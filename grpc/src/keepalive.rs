/*!
<https://github.com/grpc/grpc-go/blob/master/keepalive/keepalive.go>
*/

use std::time::Duration;

/**
ClientParameters is used to set keepalive parameters on the client-side.
These configure how the client will actively probe to notice when a connection is broken and send pings so intermediaries will be aware of the liveness of the connection. Make sure these parameters are set in coordination with the keepalive policy on the server, as incompatible settings can result in closing of connection.
*/
pub struct ClientParameters {
    /**
    After a duration of this time if the client doesn't see any activity it pings the server to see if the transport is still alive.
    If set below 10s, a minimum value of 10s will be used instead.
    The current default value is infinity.
    */
    pub time: Duration, 
    /**
    After having pinged for keepalive check, the client waits for a duration of timeout and if no activity is seen even after that the connection is closed.
    The current default value is 20 seconds.
    */
    pub timeout: Duration,
    /**
    If true, client sends keepalive pings even with no active RPCs. If false, when there are no active RPCs, time and timeout will be ignored and no keepalive pings will be sent.
    false by default.
    */
    pub permit_without_stream: bool
}

/**
ServerParameters is used to set keepalive and max-age parameters on the server-side.
*/
pub struct ServerParameters {
    /**
    Duration for the amount of time after which an idle connection would be closed by sending a GoAway. Idleness duration is defined since the most recent time the number of outstanding RPCs became zero or the connection establishment.
    The current default value is infinity.
    */
    pub max_connection_idle: Duration,
    /**
    Duration for the maximum amount of time a connection may exist before it will be closed by sending a GoAway. A random jitter of +/-10% will be added to max_connection_age to spread out connection storms.
    The current default value is infinity.
    */
    pub max_connection_age: Duration,
    /**
    Additive period after max_connection_age after which the connection will be forcibly closed.
    The current default value is infinity.
    */
    pub max_connection_age_grace: Duration,
    /**
    After a duration of this time if the server doesn't see any activity it pings the client to see if the transport is still alive.
    If set below 1s, a minimum value of 1s will be used instead.
    The current default value is 2 hours.
    */
    pub time: Duration,
    /**
    After having pinged for keepalive check, the server waits for a duration of timeout and if no activity is seen even after that the connection is closed.
    The current default value is 20 seconds.
    */
    pub timeout: Duration
}

/**
EnforcementPolicy is used to set keepalive enforcement policy on the server-side. Server will close connection with a client that violates this policy.
*/
pub struct EnforcementPolicy {
    /**
    Minimum amount of time a client should wait before sending a keepalive ping.
    The current default value is 5 minutes.
    */
    pub min_time: Duration,  
    /**
    If true, server allows keepalive pings even when there are no active streams(RPCs). If false, and client sends ping when there are no active streams, server will send GOAWAY and close the connection.
    false by default.
    */
    pub permit_without_stream: bool
}
