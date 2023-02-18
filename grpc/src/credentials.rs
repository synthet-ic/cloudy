/*!
<https://github.com/grpc/grpc-go/blob/master/credentials/credentials.go>
*/

use std::{
    collections::HashMap,
    net::TcpStream,
    task::Context
};

use protobuf::proto::Message;

use crate::{
    attributes::Attributes,
    internal::credentials as icredentials
};

/// Common interface for the credentials which need to attach security information to every RPC (e.g., oauth2).
pub trait PerRPCCredentials {
    /// Gets the current request metadata, refreshing tokens if required. This should be called by the transport layer on each request, and the data should be populated in headers or other context. If a status code is returned, it will be used as the status for the RPC (restricted to an allowable set of codes as defined by gRFC A54). uri is the URI of the entry point for the request.  When supported by the underlying implementation, ctx can be used for timeout and cancellation. Additionally, RequestInfo data will be available via ctx to this call.  TODO(zhaoq): Define the set of the qualified keys instead of leaving it as an arbitrary string.
    fn get_request_metadata(context: Context, uri: Vec<String>) -> Result<HashMap<String, String>>;

    /// Indicates whether the credentials requires transport security.
    fn require_transport_security() -> bool;
}

/**
Protection level on an established connection.

This API is experimental.
*/
pub enum SecurityLevel {
    /// an invalid security level.
    /// The zero SecurityLevel value is invalid for backward compatibility.
    InvalidSecurityLevel,
    /// a connection is insecure.
    NoSecurity,
    /// a connection only provides integrity protection.
    IntegrityOnly,
    /// a connection provides both privacy and integrity protection.
    PrivacyAndIntegrity
}

/// Returns SecurityLevel in a string format.
pub fn (s SecurityLevel) string() -> String {
    match s {
        NoSecurity =>
            return "NoSecurity",
        IntegrityOnly  =>
            return "IntegrityOnly",
        PrivacyAndIntegrity  =>
            return "PrivacyAndIntegrity",
    }
    return fmt.Sprintf("invalid SecurityLevel: %v", int(s))
}

/**
CommonAuthInfo contains authenticated information common to AuthInfo implementations.
It should be embedded in a struct implementing AuthInfo to provide additional information about the credentials.

This API is experimental.
*/
struct CommonAuthInfo {
    pub security_level: SecurityLevel
}

/// Returns the pointer to CommonAuthInfo struct.
pub fn (c CommonAuthInfo) get_common_auth_info() CommonAuthInfo {
    return c
}

/**
ProtocolInfo provides information regarding the gRPC wire protocol version, security protocol, security protocol version in use, server name, etc.
*/
pub struct ProtocolInfo {
    /// gRPC wire protocol version.
    pub protocol_version: String,
    /// Security protocol in use.
    pub security_protocol: String,
    /// User-configured server name.
    pub server_name: String
}

/**
AuthInfo defines the common interface for the auth information the users are interested in.
A struct that implements AuthInfo should embed CommonAuthInfo by including additional information about the credentials in it.
*/
pub trait AuthInfo {
    fn auth_type() -> String;
}

/// ErrConnDispatched indicates that rawConn has been dispatched out of gRPC and the caller should not close rawConn.
type ErrConnDispatched = errors.New("credentials: rawConn is dispatched out of gRPC");

/**
TransportCredentials defines the common interface for all the live gRPC wire protocols and supported transport security protocols (e.g., TLS, SSL).
*/
pub trait TransportCredentials {
    /**
    Does the authentication handshake specified by the corresponding authentication protocol on rawConn for clients. It returns the authenticated connection and the corresponding auth information about the connection.  The auth information should embed CommonAuthInfo to return additional information about the credentials. Implementations must use the provided context to implement timely cancellation.  gRPC will try to reconnect if the error returned is a temporary error (io.EOF, context.DeadlineExceeded or err.Temporary() == true).  If the returned error is a wrapper error, implementations should make sure that the error implements Temporary() to have the correct retry behaviors.
    Additionally, ClientHandshakeInfo data will be available via the context passed to this call.

    The second argument to this method is the `:authority` header value used while creating new streams on this connection after authentication succeeds. Implementations must use this as the server name during the
    authentication handshake.

    If the returned TcpStream is closed, it MUST close the TcpStream provided.
    */
    fn client_handshake(Context, String, TcpStream) -> (TcpStream, AuthInfo, error);

    /**
    server_handshake does the authentication handshake for servers. It returns the authenticated connection and the corresponding auth information about the connection. The auth information should embed CommonAuthInfo to return additional information about the credentials.

    If the returned TcpStream is closed, it MUST close the TcpStream provided.
    */
    fn server_handshake(TcpStream) -> (TcpStream, AuthInfo, error);

    /// Info provides the ProtocolInfo of this TransportCredentials.
    fn info() -> ProtocolInfo;

    /// Clone makes a copy of this TransportCredentials.
    fn clone() -> TransportCredentials;
    /**
    override_server_name specifies the value used for the following:
    - verifying the hostname on the returned certificates
    - as SNI in the client's handshake to support virtual hosting
    - as the value for `:authority` header at stream creation time

    Deprecated: use grpc.WithAuthority instead. Will be supported
    throughout 1.x.
    */
    fn override_server_name(String) -> error;
}

/**
Bundle is a combination of TransportCredentials and PerRPCCredentials.

It also contains a mode switching method, so it can be used as a combination of different credential policies.

Bundle cannot be used together with individual TransportCredentials.
PerRPCCredentials from Bundle will be appended to other PerRPCCredentials.

This API is experimental.
*/
pub trait Bundle {
    /**
    Returns the transport credentials from the Bundle.

    Implementations must return non-nil transport credentials. If transport security is not needed by the Bundle, implementations may choose to return insecure.NewCredentials().
    */
    fn transport_credentials() -> TransportCredentials;

    /**
    Returns the per-RPC credentials from the Bundle.

    May be nil if per-RPC credentials are not needed.
    */
    fn per_rpc_credentials() -> PerRPCCredentials;

    /**
    Should make a copy of Bundle, and switch mode. Modifying the existing Bundle may cause races.

    new_with_mode returns nil if the requested mode is not supported.
    */
    fn new_with_mode(mode: String) -> Result<Bundle>;
}

/**
RequestInfo contains request data attached to the context passed to get_request_metadata calls.

This API is experimental.
*/
pub struct RequestInfo {
    /// The method passed to Invoke or NewStream for this RPC. (For proto methods, this has the format "/some.Service/Method")
    pub method: String,
    /// AuthInfo contains the information from a security handshake (TransportCredentials.client_handshake, TransportCredentials.server_handshake)
    pub auth_info: AuthInfo
}

/**
Extracts the RequestInfo from the context if it exists.

This API is experimental.
*/
pub fn request_info_from_context(context: Context) -> (ri RequestInfo, ok bool) {
    ri, ok = icredentials.request_info_from_context(context).(RequestInfo)
    return ri, ok
}

/**
ClientHandshakeInfo holds data to be passed to client_handshake. This makes it possible to pass arbitrary data to the handshaker from gRPC, resolver, balancer etc. Individual credential implementations control the actual format of the data that they are willing to receive.

This API is experimental.
*/
pub struct ClientHandshakeInfo {
    /// Attributes contains the attributes for the address. It could be provided by the gRPC, resolver, balancer etc.
    pub attributes: Option<Attributes>
}

/**
Returns the ClientHandshakeInfo struct stored in context.

This API is experimental.
*/
pub fn client_handshake_info_from_context(context: Context) -> ClientHandshakeInfo
{
    chi, _ = icredentials.client_handshake_info_from_context(context).(ClientHandshakeInfo);
    return chi
}

/**
Checks if a connection's security level is greater than or equal to the specified one.
It returns success if 1) the condition is satisified or 2) AuthInfo struct does not implement get_common_auth_info() method or 3) CommonAuthInfo.SecurityLevel has an invalid zero value. For 2) and 3), it is for the purpose of backward-compatibility.

This API is experimental.
*/
fn check_security_level(ai: AuthInfo, level: SecurityLevel) -> Result<()> {
    type internalInfo interface {
        get_common_auth_info() CommonAuthInfo
    }
    
    if ai == nil {
        return errors.New("AuthInfo is nil")
    }
    if ci, ok = ai.(internalInfo); ok {
        // CommonAuthInfo.SecurityLevel has an invalid value.
        if ci.get_common_auth_info().SecurityLevel == InvalidSecurityLevel {
            return nil
        }
        if ci.get_common_auth_info().SecurityLevel < level {
            return fmt.Errorf("requires SecurityLevel %v; connection has %v", level, ci.GetCommonAuthInfo().SecurityLevel)
        }
    }
    // The condition is satisfied or AuthInfo struct does not implement get_common_auth_info() method.
    return nil
}

/**
ChannelzSecurityInfo defines the interface that security protocols should implement in order to provide security info to channelz.

This API is experimental.
*/
pub trait ChannelzSecurityInfo {
    fn get_security_value() -> ChannelzSecurityValue;
}

/**
ChannelzSecurityValue defines the interface that get_security_value() return value should satisfy. This interface should only be satisfied by *TLSChannelzSecurityValue and *OtherChannelzSecurityValue.

This API is experimental.
*/
pub trait ChannelzSecurityValue {
    fn is_channelz_security_value();
}

/**
OtherChannelzSecurityValue defines the struct that non-TLS protocol should return from get_security_value(), which contains protocol specific security info. Note the Value field will be sent to users of channelz requesting channel info, and thus sensitive info should better be avoided.

This API is experimental.
*/
pub struct OtherChannelzSecurityValue {
    ChannelzSecurityValue
    pub name: String,
    pub value: Message
}
