/*!
<https://github.com/grpc/grpc-go/blob/master/balancer/balancer.go>
*/

use std::task::Context;

use crate::{
    channelz::Identifier,
    connectivity::State,
    credentials::{Bundle, TransportCredentials},
    internal,
    metadata::Metadata,
    resolver::{Address, ResolveNowOptions, Target},
    service_config::LoadBalancingConfig
};

var (
    // m is a map from name to balancer builder.
    m = make(HashMap<String, Builder>)
)

/**
Registers the balancer builder to the balancer map. b.Name (lowercased) will be used as the name registered with this builder.  If the
Builder implements ConfigParser, parse_config will be called when new service configs are received by the resolver, and the result will be provided to the Balancer in update_client_conn_state.

NOTE: this function must only be called during initialisation time (i.e. in
an init() function), and is not thread-safe. If multiple Balancers are
registered with the same name, the one registered last will take effect.
*/
pub fn register(b: Builder) {
    m[strings.ToLower(b.Name())] = b;
}

/**
Deletes the balancer with the given name from the
balancer map.

This function is not thread-safe.
*/
fn unregister_for_testing(name: String) {
    delete(m, name)
}

fn init() {
    internal.BalancerUnregister = unregister_for_testing
}

/*
Get returns the resolver builder registered with the given name.
Note that the compare is done in a case-insensitive fashion.
If no builder is register with the name, nil will be returned.
*/
fn Get(name: String) -> Builder {
    if b, ok = m[strings.ToLower(name)]; ok {
        return b
    }
    return nil
}

/**
A SubConn represents a single connection to a gRPC backend service.

Each SubConn contains a list of addresses.

All SubConns start in IDLE, and will not try to connect. To trigger the
connecting, Balancers must call connect.  If a connection re-enters IDLE,
Balancers must call connect again to trigger a new connection attempt.

gRPC will try to connect to the addresses in sequence, and stop trying the remainder once the first connection is successful. If an attempt to connect to all addresses encounters an error, the SubConn will enter
TRANSIENT_FAILURE for a backoff period, and then transition to IDLE.

Once established, if a connection is lost, the SubConn will transition directly to IDLE.

This interface is to be implemented by gRPC. Users should not need their own implementation of this interface. For situations like testing, any implementations should embed this interface. This allows gRPC to add new methods to this interface.
*/
pub trait SubConn {
    /// connect starts the connecting for this SubConn.
    fn connect();

    /**
    Returns a reference to the existing Producer for this
    ProducerBuilder in this SubConn, or, if one does not currently exist, creates a new one and returns it.  Returns a close function which must be called when the Producer is no longer needed.
    */
    fn get_or_build_producer(ProducerBuilder) (p Producer, close func())
}

/// NewSubConnOptions contains options to create new SubConn.
pub struct NewSubConnOptions {
    /// Whether health check service should be enabled on this SubConn
    pub health_check_enabled: bool
}

/// State contains the balancer's state relevant to the gRPC ClientConn.
pub struct State {
    /**
    State contains the connectivity state of the balancer, which is used to determine the state of the ClientConn.
    */
    pub connectivity_state: State,
    /// Picker is used to choose connections (SubConns) for RPCs.
    pub picker: Picker
}

/**
ClientConn represents a gRPC ClientConn.

This interface is to be implemented by gRPC. Users should not need a brand new implementation of this interface. For the situations like testing, the new implementation should embed this interface. This allows gRPC to add new methods to this interface.
*/
pub trait ClientConn {
    /**
    new_sub_conn is called by balancer to create a new SubConn.
    It doesn't block and wait for the connections to be established.
    Behaviors of the SubConn can be controlled by options.
    */
    fn new_sub_conn(Vec<Address>, NewSubConnOptions) (SubConn, error);

    /**
    remove_sub_conn removes the SubConn from ClientConn.
    The SubConn will be shutdown.
    */
    fn remove_sub_conn(SubConn);

    /**
    Updates the addresses used in the passed in SubConn.
    gRPC checks if the currently connected address is still in the new list.
    If so, the connection will be kept. Else, the connection will be
    gracefully closed, and a new connection will be created.

    This will trigger a state transition for the SubConn.
    */
    fn update_addresses(SubConn, Vec<Address>);

    /**
    Notifies gRPC that the balancer's internal state has changed.

    gRPC will update the connectivity state of the ClientConn, and will call
    Pick on the new Picker to pick new SubConns.
    */
    fn update_state(State);

    /// resolve_now is called by balancer to notify gRPC to do a name resolving.
    fn resolve_now(ResolveNowOptions)

    /*
    Target returns the dial target for this ClientConn.

    Deprecated: Use the Target field in the BuildOptions instead.
    */
    fn Target() -> String;
}

/// BuildOptions contains additional information for Build.
pub struct BuildOptions {
    /**
    dial_creds is the transport credentials to use when communicating with a
    remote load balancer server. Balancer implementations which do not
    communicate with a remote load balancer server can ignore this field.
    */
    pub dial_creds: TransportCredentials,
    /**
    creds_bundle is the credentials bundle to use when communicating with a
    remote load balancer server. Balancer implementations which do not
    communicate with a remote load balancer server can ignore this field.
    */
    pub creds_bundle; Bundle,
    /**
    Dialer is the custom dialer to use when communicating with a remote load
    balancer server. Balancer implementations which do not communicate with a
    remote load balancer server can ignore this field.
    */
    pub dialer: fn(Context, String) -> (net.Conn, error);
    /**
    authority is the server name to use as part of the authentication handshake when communicating with a remote load balancer server. Balancer implementations which do not communicate with a remote load balancer server can ignore this field.
    */
    pub authority: String,
    /// channelz_parent_id is the parent ClientConn's channelz ID.
    pub channelz_parent_id: *Identifier
    /**
    custom_user_agent is the custom user agent set on the parent ClientConn.
    The balancer should set the same custom user agent if it creates a
    ClientConn.
    */
    pub custom_user_agent: String,
    /**
    Target contains the parsed address info of the dial target. It is the same resolver.Target as passed to the resolver. See the documentation for the resolver.Target type for details about what it contains.
    */
    pub target: Target
}

/// Builder creates a balancer.
pub trait Builder {
    /// Build creates a new balancer with the ClientConn.
    fn build(cc: ClientConn, opts: BuildOptions) -> Balancer;
    
    /**
    Name returns the name of balancers built by this builder.
    It will be used to pick balancers (for example in service config).
    */
    fn name() -> String;
}

/// ConfigParser parses load balancer configs.
pub trait ConfigParser {
    /**
    Parses the JSON load balancer config provided into an internal form or returns an error if the config is invalid. For future compatibility reasons, unknown fields in the config should be ignored.
    */
    fn parse_config(LoadBalancingConfigJSON: json.RawMessage) -> (LoadBalancingConfig, error)
}

/// PickInfo contains additional information for the Pick operation.
pub struct PickInfo {
    /**
    full_method_name is the method name that NewClientStream() is called
    with. The canonical format is /service/Method.
    */
    pub full_method_name: String,
    /**
    context is the RPC's context, and may contain relevant RPC-level information
    like the outgoing header metadata.
    */
    pub context: Context
}

/// DoneInfo contains additional information for done.
pub struct DoneInfo {
    /// Err is the rpc error the RPC finished with. It could be nil.
    pub err: error,
    /// trailer contains the metadata from the RPC's trailer, if present.
    pub trailer: Metadata,
    /// bytes_sent indicates if any bytes have been sent to the server.
    pub bytes_sent: bool,
    /// bytes_received indicates if any byte has been received from the server.
    pub bytes_received: bool,
    /**
    server_load is the load received from server. It's usually sent as part of trailing metadata.

    The only supported type now is *orca_v3.LoadReport.
    */
    pub server_load: interface{}
}

var (
    // ErrNoSubConnAvailable indicates no SubConn is available for pick().
    // gRPC will block the RPC until a new picker is available via update_state().
    ErrNoSubConnAvailable = errors.New("no SubConn is available")
    // ErrTransientFailure indicates all SubConns are in TransientFailure.
    // WaitForReady RPCs will block, non-WaitForReady RPCs will fail.
    //
    // Deprecated: return an appropriate error based on the last resolution or
    // connection attempt instead.  The behavior is the same for any non-gRPC
    // status error.
    ErrTransientFailure = errors.New("all SubConns are in TransientFailure")
)

/// PickResult contains information related to a connection chosen for an RPC.
pub struct PickResult {
    /**
    SubConn is the connection to use for this pick, if its state is Ready.
    If the state is not Ready, gRPC will block the RPC until a new Picker is
    provided by the balancer (using ClientConn.update_state).  The SubConn
    must be one returned by ClientConn.new_sub_conn.
    */
    pub sub_conn: SubConn,

    /**
    Done is called when the RPC is completed.  If the SubConn is not ready,
    this will be called with a nil parameter.  If the SubConn is not a valid
    type, Done may not be called.  May be nil if the balancer does not wish
    to be notified when the RPC completes.
    */
    pub done: fn(DoneInfo)
}

/**
Returns e.  It exists for backward compatibility and will be deleted soon.

Deprecated: no longer necessary, picker errors are treated this way by
default.
*/
pub fn transient_failure_error(e: error) -> error { return e }

/**
Picker is used by gRPC to pick a SubConn to send an RPC.
Balancer is expected to generate a new picker from its snapshot every time its
internal state has changed.

The pickers used by gRPC can be updated by ClientConn.update_state().
*/
pub trait Picker {
    /**
    Pick returns the connection to use for this RPC and related information.

    Pick should not block.  If the balancer needs to do I/O or any blocking
    or time-consuming work to service this call, it should return
    ErrNoSubConnAvailable, and the Pick call will be repeated by gRPC when
    the Picker is updated (using ClientConn.update_state).

    If an error is returned:

    - If the error is ErrNoSubConnAvailable, gRPC will block until a new
      Picker is provided by the balancer (using ClientConn.update_state).

    - If the error is a status error (implemented by the grpc/status
      package), gRPC will terminate the RPC with the code and message
      provided.

    - For all other errors, wait for ready RPCs will wait, but non-wait for
      ready RPCs will be terminated with this error's Error() string and
      status code Unavailable.
    */
    fn pick(info: PickInfo) -> (PickResult, error);
}

/**
Balancer takes input from gRPC, manages SubConns, and collects and aggregates the connectivity states.

It also generates and updates the Picker used by gRPC to pick SubConns for RPCs.

update_client_conn_state, resolver_error, update_sub_conn_state, and close are guaranteed to be called synchronously from the same goroutine.  There's no guarantee on picker.Pick, it may be called anytime.
*/
pub trait Balancer {
    /**
    update_client_conn_state is called by gRPC when the state of the ClientConn
    changes.  If the error returned is ErrBadResolverState, the ClientConn
    will begin calling resolve_now on the active name resolver with
    exponential backoff until a subsequent call to update_client_conn_state
    returns a nil error.  Any other errors are currently ignored.
    */
    fn update_client_conn_state(ClientConnState) error;

    /// resolver_error is called by gRPC when the name resolver reports an error.
    fn resolver_error(error);

    /// update_sub_conn_state is called by gRPC when the state of a SubConn
    /// changes.
    fn update_sub_conn_state(SubConn, SubConnState);

    /// close closes the balancer. The balancer is not required to call
    /// ClientConn.remove_sub_conn for its existing SubConns.
    fn close();
}

/**
ExitIdler is an optional interface for balancers to implement.  If implemented, exit_idle will be called when ClientConn.connect is called, if the ClientConn is idle.  If unimplemented, ClientConn.connect will cause all SubConns to connect.

Notice: it will be required for all balancers to implement this in a future
release.
*/
pub trait ExitIdler {
    /**
    Instructs the LB policy to reconnect to backends / exit the
    IDLE state, if appropriate and possible.  Note that SubConns that enter
    the IDLE state will not reconnect until SubConn.connect is called.
    */
    fn exit_idle();
}

/// SubConnState describes the state of a SubConn.
pub struct SubConnState {
    /// pub connectivity_state is the connectivity state of the SubConn.
    pub connectivity_state: State,
    /// connection_error is set if the pub connectivity_state is TransientFailure,
    /// describing the reason the SubConn failed.  Otherwise, it is nil.
    pub connection_error: error
}

// ClientConnState describes the state of a ClientConn relevant to the
// balancer.
pub struct ClientConnState {
    pub resolver_state: State
    /**
    The parsed load balancing configuration returned by the builder's
    parse_config method, if implemented.
    */
    pub balancer_config: LoadBalancingConfig
}

// ErrBadResolverState may be returned by update_client_conn_state to indicate a
// problem with the provided name resolver data.
var ErrBadResolverState = errors.New("bad resolver state")

/**
A ProducerBuilder is a simple constructor for a Producer.  It is used by the
SubConn to create producers when needed.
*/
pub trait ProducerBuilder {
    /**
    Build creates a Producer.  The first parameter is always a
    grpc.ClientConnInterface (a type to allow creating RPCs/streams on the
    associated SubConn), but is declared as interface{} to avoid a
    dependency cycle.  Should also return a close function that will be
    called when all references to the Producer have been given up.
    */
    fn build(grpcClientConnInterface: interface{}) -> (p Producer, close func())
}

/**
A Producer is a type shared among potentially many consumers. It is associated with a SubConn, and an implementation will typically contain other methods to provide additional functionality, e.g. configuration or subscription registration.
*/
pub struct Producer {
}
