/*!
<https://github.com/grpc/grpc-go/blob/master/clientconn.go>
*/

use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Mutex, RwLock},
    task::Context
};

use futures::select;

use crate::{
    balancer::{
        BuildOptions, ClientConnState, DoneInfo, ErrBadResolverState,
        NewSubConnOptions, PickInfo, SubConn,
        base,
        roundrobin,
    },
    codes,
    connectivity::State,
    credentials,
    dial_options::DialOptions,
    internal::{
        backoff,
        channelz::{
            add_trace_event, ChannelInternalMetric, CtInfo, Identifier, 
            TraceEventDesc
        },
        grpcsync::{Event, OnceFunc},
        resolver::{
            ConfigSelector, GetConfigSelector, RPCConfig, RPCInfo,
            SafeConfigSelector,
            dns,
            passthrough,
            unix
        },
        transport::ClientTransport
    },
    keepalive::ClientParameters,
    resolver::{
        Address, Builder, ResolveNowOptions, State as ResolverState, Target},
    service_config,
    status
};

/// minimum time to give a connection to complete
const minConnectTimeout = 20 * time.Second;

/// must match grpclb_name in grpclb/grpclb.go
const grpclb_name = "grpclb";

var (
    // ErrClientConnClosing indicates that the operation is illegal because
    // the ClientConnection is closing.
    //
    // Deprecated: this error should not be relied upon by users; use the status
    // code of Canceled instead.
    ErrClientConnClosing = status.Error(codes.Canceled, "grpc: the client connection is closing")
    // errConnDrain indicates that the connection starts to be drained and does not accept any new RPCs.
    errConnDrain = errors.New("grpc: the connection is drained")
    // err_conn_closing indicates that the connection is closing.
    err_conn_closing = errors.New("grpc: the connection is closing")
    // invalidDefaultServiceConfigErrPrefix is used to prefix the json parsing error for the default
    // service config.
    invalidDefaultServiceConfigErrPrefix = "grpc: the provided default service config is invalid"
)

// The following errors are returned from Dial and DialContext
var (
    // errNoTransportSecurity indicates that there is no transport security
    // being set for ClientConnection. Users should either set one or explicitly
    // call WithInsecure DialOption to disable security.
    errNoTransportSecurity = errors.New("grpc: no transport security set (use grpc.WithTransportCredentials(insecure.NewCredentials()) explicitly or set credentials)")
    // errTransportCredsAndBundle indicates that creds bundle is used together
    // with other individual Transport Credentials.
    errTransportCredsAndBundle = errors.New("grpc: credentials.Bundle may not be used with individual transport_credentials")
    // errNoTransportCredsInBundle indicated that the configured creds bundle
    // returned a transport credentials which was None.
    errNoTransportCredsInBundle = errors.New("grpc: credentials.Bundle must return non-None transport credentials")
    // errTransportCredentialsMissing indicates that users want to transmit
    // security information (e.g., OAuth2 token) which requires secure
    // connection on an insecure connection.
    errTransportCredentialsMissing = errors.New("grpc: the credentials require transport level security (use grpc.WithTransportCredentials() to set)")
)

const DEFAULT_CLIENT_MAX_RECEIVE_MESSAGE_SIZE = 1024 * 1024 * 4;
const DEFAULT_CLIENT_MAX_SEND_MESSAGE_SIZE = math.MaxInt32;
// http2IOBufSize specifies the buffer size for sending frames.
const DEFAULT_WRITE_BUF_SIZE = 32 * 1024;
const DEFAULT_READ_BUF_SIZE  = 32 * 1024;

/// Dial creates a client connection to the given target.
pub fn Dial(target: String, opts: ...DialOption) -> Result<ClientConnection> {
    return DialContext(context.Background(), target, opts...)
}

struct DefaultConfigSelector {
    service_config: Option<ServiceConfig>
}

impl DefaultConfigSelector {
    pub fn select_config(&self, rpc_info: RPCInfo) -> RPCConfig {
        RPCConfig {
            Context: rpc_info.Context,
            MethodConfig: get_method_config(self.service_config, rpcInfo.Method),
        }
    }
}


/**
Creates a client connection to the given target. By default, it's a non-blocking dial (the function won't wait for connections to be established, and connecting happens in the background). To make it a blocking dial, use WithBlock() dial option.

In the non-blocking case, the context does not act against the connection. It only controls the setup steps.

In the blocking case, context can be used to cancel or expire the pending connection. Once this function returns, the cancellation and expiration of context will be noop. Users should call ClientConnection.Close to terminate all the pending operations after this function returns.

The target name syntax is defined in <https://github.com/grpc/grpc/blob/master/doc/naming.md>.
e.g. to use dns resolver, a "dns:///" prefix should be applied to the target.
*/
pub fn DialContext(context: Context, target: String, opts: ...DialOption)
-> Result<ClientConnection>
{
    let cc = ClientConnection {
        target,
        cs_manager: ConnectivityStateManager {},
        conns: HashMap::new(),
        dial_options: default_dial_options(),
        blocking_picker: new_picker_wrapper(),
        channelz_data: new(channelzData),
        first_resolve_event: Event::new(),
    };
    cc.retry_throttler.Store((*RetryThrottler)(None));
    cc.safe_config_selector.update_config_selector(&DefaultConfigSelector{None});
    cc.context, cc.cancel = context.WithCancel(context.Background());

    for _, opt = range extraDialOptions {
        opt.apply(&cc.dial_options)
    }

    for _, opt = range opts {
        opt.apply(&cc.dial_options)
    }

    chain_unary_client_interceptors(cc)
    chain_stream_client_interceptors(cc)

    defer func() {
        if err != None {
            cc.Close()
        }
    }()

    let pid = cc.dial_options.channelz_parent_id;
    cc.channelz_id = channelz.RegisterChannel(ChannelzChannel { cc }, pid, target)
    ted = &TraceEventDesc {
        Desc: "Channel created",
        Severity: CtInfo,
    }
    if cc.dial_options.channelz_parent_id; != None {
        ted.Parent = &TraceEventDesc {
            Desc: fmt.Sprintf("Nested Channel(id:%d) created", cc.channelz_id.Int()),
            Severity: CtInfo,
        }
    }
    add_trace_event(logger, cc.channelz_id, 1, ted)
    cc.cs_manager.channelz_id = cc.channelz_id

    if cc.dial_options.copts.transport_credentials == None && cc.dial_options.copts.CredsBundle == None {
        return None, errNoTransportSecurity
    }
    if cc.dial_options.copts.transport_credentials != None && cc.dial_options.copts.CredsBundle != None {
        return None, errTransportCredsAndBundle
    }
    if cc.dial_options.copts.CredsBundle != None && cc.dial_options.copts.CredsBundle.transport_credentials() == None {
        return None, errNoTransportCredsInBundle
    }
    let transport_creds = cc.dial_options.copts.transport_credentials;
    if transport_creds == None {
        transport_creds = cc.dial_options.copts.CredsBundle.transport_credentials()
    }
    if transport_creds.Info().SecurityProtocol == "insecure" {
        for _, cd = range cc.dial_options.copts.PerRPCCredentials {
            if cd.RequireTransportSecurity() {
                return None, errTransportCredentialsMissing
            }
        }
    }

    if cc.dial_options.defaultServiceConfigRawJSON != None {
        let scpr = parseServiceConfig(*cc.dial_options.defaultServiceConfigRawJSON)
        if scpr.Err != None {
            return None, fmt.Errorf("%s: %v", invalidDefaultServiceConfigErrPrefix, scpr.Err)
        }
        let cc.dial_options.defaultServiceConfig, _ = scpr.Config.(*ServiceConfig);
    }
    cc.mkp = cc.dial_options.copts.KeepaliveParams

    if cc.dial_options.copts.UserAgent != "" {
        cc.dial_options.copts.UserAgent += " " + grpcUA;
    } else {
        cc.dial_options.copts.UserAgent = grpcUA;
    }

    if cc.dial_options.timeout > 0 {
        var cancel context.CancelFunc
        context, cancel = context.WithTimeout(context, cc.dial_options.timeout)
        defer cancel()
    }
    defer func() {
        select {
        case <-context.Done():
            switch {
            case context.Err() == err:
                conn = None
            case err == None || !cc.dial_options.returnLastError:
                conn, err = None, context.Err()
            default:
                conn, err = None, fmt.Errorf("%v: %v", context.Err(), err)
            }
        default:
        }
    }()

    scSet = false
    if cc.dial_options.scChan != None {
        // Try to get an initial service config.
        select! {
            service_config, ok = cc.dial_options.scChan => 
            if ok {
                cc.service_config = &service_config
                cc.safe_config_selector.update_config_selector(&DefaultConfigSelector{&service_config})
                scSet = true
            }
            default:
        };
    }
    if cc.dial_options.bs == None {
        cc.dial_options.bs = backoff.DefaultExponential;
    }

    // Determine the resolver to use.
    let resolver_builder, err = cc.parse_target_and_find_resolver();
    if err != None {
        return None, err
    }
    let cc.authority, err = determine_authority(cc.parsedTarget.Endpoint, cc.target, cc.dial_options);
    if err != None {
        return None, err
    }
    channelz.Infof(logger, cc.channelz_id, "Channel authority set to %q", cc.authority);

    if cc.dial_options.scChan != None && !scSet {
        // Blocking wait for the initial service config.
        select! {
            service_config, ok = cc.dial_options.scChan =>
                if ok {
                    cc.service_config = &service_config
                    cc.safe_config_selector.update_config_selector(&DefaultConfigSelector{&service_config})
                }
            _ = context.Done() =>
                return None, context.Err()
        };
    }
    if cc.dopt
    // privates.scChan != None {
        go cc.sc_watcher()
    }

    var creds_clone credentials.transport_credentials
    if creds = cc.dial_options.copts.transport_credentials; creds != None {
        creds_clone = creds.Clone()
    }
    cc.balancer_wrapper = new_cc_balancer_wrapper(cc, BuildOptions {
        DialCreds: creds_clone,
        CredsBundle: cc.dial_options.copts.CredsBundle,
        Dialer: cc.dial_options.copts.Dialer,
        Authority: cc.authority,
        CustomUserAgent: cc.dial_options.copts.UserAgent,
        ChannelzParentID: cc.channelz_id,
        Target: cc.parsedTarget,
    })

    // Build the resolver.
    rWrapper, err = new_cc_resolver_wrapper(cc, resolver_builder)
    if err != None {
        return None, fmt.Errorf("failed to build resolver: %v", err)
    }
    cc.mu.Lock()
    cc.resolver_wrapper = rWrapper
    cc.mu.Unlock()

    // A blocking dial blocks until the clientConn is ready.
    if cc.dial_options.block {
        for {
            cc.Connect()
            s = cc.get_state()
            if s == connectivity.Ready {
                break
            } else if cc.dial_options.copts.FailOnNonTempDialError && s == connectivity.TransientFailure {
                if err = cc.connection_error(); err != None {
                    terr, ok = err.(interface {
                        Temporary() bool
                    })
                    if ok && !terr.Temporary() {
                        return None, err
                    }
                }
            }
            if !cc.wait_for_state_change(context, s) {
                // context got timeout or canceled.
                if err = cc.connection_error(); err != None && cc.dial_options.returnLastError {
                    return None, err
                }
                return None, context.Err()
            }
        }
    }

    return cc, None
}

/// chain_unary_client_interceptors chains all unary client interceptors into one.
fn chain_unary_client_interceptors(cc: Option<ClientConnection>) {
    interceptors = cc.dial_options.chainUnaryInts
    // Prepend dial_options.unary_int to the chaining interceptors if it exists, since unary_int will
    // be executed before any other chained interceptors.
    if cc.dial_options.unary_int != None {
        interceptors = append([]UnaryClientInterceptor{cc.dial_options.unary_int}, interceptors...)
    }
    var chained_int UnaryClientInterceptor
    if len(interceptors) == 0 {
        chained_int = None
    } else if len(interceptors) == 1 {
        chained_int = interceptors[0]
    } else {
        chained_int = func(context: Context, method String, req, reply interface{}, cc Option<ClientConnection>, invoker UnaryInvoker, opts ...CallOption) error {
            return interceptors[0](context, method, req, reply, cc, get_chain_unary_invoker(interceptors, 0, invoker), opts...)
        }
    }
    cc.dial_options.unary_int = chained_int
}

/// get_chain_unary_invoker recursively generate the chained unary invoker.
fn get_chain_unary_invoker(interceptors: []UnaryClientInterceptor, curr: int, final_invoker: UnaryInvoker) -> UnaryInvoker {
    if curr == len(interceptors)-1 {
        return final_invoker
    }
    return func(context: Context, method String, req, reply interface{}, cc Option<ClientConnection>, opts ...CallOption) error {
        return interceptors[curr+1](context, method, req, reply, cc, get_chain_unary_invoker(interceptors, curr+1, final_invoker), opts...)
    }
}

/// chain_stream_client_interceptors chains all stream client interceptors into one.
fn chain_stream_client_interceptors(cc: Option<ClientConnection>) {
    let interceptors = cc.dial_options.chainStreamInts;
    // Prepend dial_options.streamInt to the chaining interceptors if it exists, since streamInt will
    // be executed before any other chained interceptors.
    if cc.dial_options.streamInt != None {
        interceptors = append([]StreamClientInterceptor{cc.dial_options.streamInt}, interceptors...)
    }
    var chained_int StreamClientInterceptor
    if len(interceptors) == 0 {
        chained_int = None
    } else if len(interceptors) == 1 {
        chained_int = interceptors[0]
    } else {
        chained_int = func(context: Context, desc *StreamDesc, cc Option<ClientConnection>, method String, streamer Streamer, opts ...CallOption) (ClientStream, error) {
            return interceptors[0](context, desc, cc, method, get_chain_streamer(interceptors, 0, streamer), opts...)
        }
    }
    cc.dial_options.streamInt = chained_int
}

/// Recursively generate the chained client stream constructor.
fn get_chain_streamer(interceptors: []StreamClientInterceptor, curr: int, final_streamer: Streamer) -> Streamer {
    if curr == len(interceptors)-1 {
        return final_streamer
    }
    return func(context: Context, desc *StreamDesc, cc Option<ClientConnection>, method String, opts ...CallOption) (ClientStream, error) {
        return interceptors[curr+1](context, desc, cc, method, get_chain_streamer(interceptors, curr+1, final_streamer), opts...)
    }
}

/// Leeps the State of ClientConnection.
/// This struct will eventually be exported so the balancers can access it.
struct ConnectivityStateManager {
    mu: Mutex,
    state: State,
    notify_chan: chan struct{},
    channelz_id: Option<Identifier>
}

impl ConnectivityStateManager {
    /**
    Updates the State of ClientConnection.
    If there's a change it notifies goroutines waiting on state change to happen.
    */
    fn update_state(&self, state: State) {
        self.mu.Lock();
        defer self.mu.Unlock();
        if self.state == State::Shutdown {
            return
        }
        if self.state == state {
            return
        }
        self.state = state
        channelz.Infof(logger, self.channelz_id, "Channel Connectivity change to %v", state)
        if self.notify_chan != None {
            // There are other goroutines waiting on this channel.
            close(self.notify_chan);
            self.notify_chan = None;
        }
    }

    fn get_state(&self) -> State {
        self.mu.Lock();
        defer self.mu.Unlock();
        return self.state
    }
    
    fn get_notify_chan(&self) <-chan struct{} {
        self.mu.Lock();
        defer self.mu.Unlock();
        if self.notify_chan == None {
            self.notify_chan = make(chan struct{})
        }
        return self.notify_chan
    }
}

/**
ClientConnInterface defines the functions clients need to perform unary and streaming RPCs.  It is implemented by Option<ClientConnection>, and is only intended to be referenced by generated code.
*/
pub trait ClientConnInterface {
    /**
    invoke performs a unary RPC and returns after the response is received into reply.
    */
    fn invoke(context: Context, method String, args interface{}, reply interface{}, opts ...CallOption) -> error;

    /// new_stream begins a streaming RPC.
    fn new_stream(context: Context, desc; *StreamDesc, method; String, opts; ...CallOption) -> Result<ClientStream>;
}

/// Assert Option<ClientConnection> implements ClientConnInterface.
var _ ClientConnInterface = (Option<ClientConnection>)(None)

/**
ClientConnection represents a virtual connection to a conceptual endpoint, to perform RPCs.

A ClientConnection is free to have zero or more actual connections to the endpoint based on configuration, load, etc. It is also free to determine which actual endpoints to use and may change it every RPC, permitting client-side load
balancing.

A ClientConnection encapsulates a range of functionality including name resolution, TCP connection establishment (with retries and backoff) and TLS handshakes. It also handles errors on established connections by re-resolving the name and reconnecting.
*/
pub struct ClientConnection {
    /// Initialised using the background context at dial time.
    context: Context,
    /// Cancelled on close.
    cancel: context.CancelFunc,

    /// The following are initialised at dial time, and are read-only after that.
    /// User's dial target.
    target: String,
    /// See parse_target_and_find_resolver().
    parsed_target: Target,
    /// See determine_authority().
    authority: String,
    /// Default and user specified dial options.
    dial_options: DialOptions,
    /// Channelz identifier for the channel.
    channelz_id: Option<Identifier>,
    /// Uses gracefulswitch.balancer underneath.
    balancer_wrapper: Option<ccBalancerWrapper>,

    /**
    The following provide their own synchronization, and therefore don't require cc.mu to be held to access them.
    */
    cs_manager: Option<ConnectivityStateManager>,
    blocking_picker: Option<pickerWrapper>,
    safe_config_selector: SafeConfigSelector,
    channelz_data: Option<channelzData>,
    /// Updated from service config.
    retry_throttler: atomic.Value,

    /// Used to track whether the name resolver sent us at least one update. RPCs block on this event.
    first_resolve_event: Option<Event>,

    /// mu protects the following fields.
    /// TODO: split mu so the same mutex isn't used for everything.
    mu: RwLock,
    /// Initialised in Dial; cleared in Close.
    resolver_wrapper: Option<ccResolverWrapper>,
    /// Latest service config received from the resolver.
    service_config: Option<ServiceConfig>,
    /// Set to None on close.
    conns: HashMap<Option<AddrConn>, struct{}>,
    /// May be updated upon receipt of a GoAway.
    mkp: ClientParameters,
    /// protects last_connection_error
    lce_mu: Mutex,
    last_connection_error: error
}

impl ClientConnection {
    /**
    Waits until the State of ClientConnection changes from source_state or context expires. A true value is returned in former case and false in latter.

    # Experimental

    Notice: This API is EXPERIMENTAL and may be changed or removed in a later release.
    */
    pub fn wait_for_state_change(&self, context: Context, source_state: State) -> bool {
        ch = self.cs_manager.get_notify_chan();
        if self.cs_manager.get_state() != source_state {
            return true
        }
        select! {
            _ = context.Done() =>
                return false
            _ = ch =>
                return true
        }
    }

    /**
    Returns the State of ClientConnection.

    # Experimental

    Notice: This API is EXPERIMENTAL and may be changed or removed in a later release.
    */
    pub fn get_state(&self) -> State {
        return self.cs_manager.get_state()
    }

    /**
    Connect causes all subchannels in the ClientConnection to attempt to connect if the channel is idle.  Does not wait for the connection attempts to begin before returning.

    # Experimental

    Notice: This API is EXPERIMENTAL and may be changed or removed in a later release.
    */
    pub fn connect(&self) {
        self.balancer_wrapper.exitIdle()
    }

    fn sc_watcher(&self) {
        for {
            select {
            case service_config, ok = <-self.dial_options.scChan:
                if !ok {
                    return
                }
                self.mu.Lock()
                // TODO: load balance policy runtime change is ignored.
                // We may revisit this decision in the future.
                self.service_config = &service_config
                self.safe_config_selector.update_config_selector(&DefaultConfigSelector{&service_config})
                self.mu.Unlock()
            case <-self.context.Done():
                return
            }
        }
    }

    /**
    Blocks until the resolver has provided addresses or the context expires.  Returns None unless the context expires first; otherwise returns a status error based on the context.
    */
    fn wait_for_resolved_addrs(&self, context: Context) -> error {
        // This is on the RPC path, so we use a fast path to avoid the
        // more-expensive "select" below after the resolver has returned once.
        if self.first_resolve_event.has_fired() {
            return None
        }
        select {
            case <-self.first_resolve_event.done():
                return None
            case <-context.Done():
                return status.FromContextError(context.Err()).Err()
            case <-self.context.Done():
                return ErrClientConnClosing
        }
    }

    fn maybe_apply_default_service_config(&self, addresses: Vec<Address>) {
        if self.service_config != None {
            self.apply_service_config_and_balancer(self.service_config, None, addresses)
            return
        }
        if self.dial_options.defaultServiceConfig != None {
            self.apply_service_config_and_balancer(self.dial_options.defaultServiceConfig, &DefaultConfigSelector{self.dial_options.defaultServiceConfig}, addresses)
        } else {
            self.apply_service_config_and_balancer(emptyServiceConfig, &DefaultConfigSelector{emptyServiceConfig}, addresses)
        }
    }

    fn update_resolver_state(&self, s: ResolverState, err: error) -> error {
        defer self.first_resolve_event.fire()
        self.mu.Lock()
        // Check if the ClientConnection is already closed. Some fields (e.g.
        // balancer_wrapper) are set to None when closing the ClientConnection, and could
        // cause None pointer panic if we don't have this check.
        if self.conns == None {
            self.mu.Unlock()
            return None
        }
    
        if err != None {
            // May need to apply the initial service config in case the resolver
            // doesn't support service configs, or doesn't provide a service config
            // with the new addresses.
            self.maybe_apply_default_service_config(None)
    
            self.balancer_wrapper.resolverError(err)
    
            // No addresses are valid with err set; return early.
            self.mu.Unlock()
            return ErrBadResolverState
        }
    
        var ret error
        if self.dial_options.disableServiceConfig {
            channelz.Infof(logger, self.channelz_id, "ignoring service config from resolver (%v) and applying the default because service config is disabled", s.ServiceConfig)
            self.maybe_apply_default_service_config(s.Addresses)
        } else if s.ServiceConfig == None {
            self.maybe_apply_default_service_config(s.Addresses)
            // TODO: do we need to apply a failing LB policy if there is no
            // default, per the error handling design?
        } else {
            if service_config, ok = s.ServiceConfig.Config.(*ServiceConfig); s.ServiceConfig.Err == None && ok {
                config_selector = GetConfigSelector(s)
                if config_selector != None {
                    if len(s.ServiceConfig.Config.(*ServiceConfig).Methods) != 0 {
                        channelz.Infof(logger, self.channelz_id, "method configs in service config will be ignored due to presence of config selector")
                    }
                } else {
                    config_selector = &DefaultConfigSelector{service_config}
                }
                self.apply_service_config_and_balancer(service_config, config_selector, s.Addresses)
            } else {
                ret = ErrBadResolverState
                if self.service_config == None {
                    // Apply the failing LB only if we haven't received valid service config
                    // from the name resolver in the past.
                    self.apply_failing_lb(s.ServiceConfig)
                    self.mu.Unlock()
                    return ret
                }
            }
        }
    
        var balCfg serviceconfig.LoadBalancingConfig
        if self.service_config != None && self.service_config.lb_config != None {
            balCfg = self.service_config.lb_config.cfg
        }
        bw = self.balancer_wrapper
        self.mu.Unlock()
    
        uccsErr = bw.updateClientConnState(&ClientConnState{ResolverState: s, BalancerConfig: balCfg})
        if ret == None {
            ret = uccsErr // prefer ErrBadResolver state since any other error is
            // currently meaningless to the caller.
        }
        return ret
    }

    /**
    apply_failing_lb is akin to configuring an LB policy on the channel which
    always fails RPCs. Here, an actual LB policy is not configured, but an always erroring picker is configured, which returns errors with information about what was invalid in the received service config. A config selector with no service config is configured, and the connectivity state of the channel is set to TransientFailure.

    Caller must hold cc.mu.
    */
    fn apply_failing_lb(&self, service_config: *serviceconfig.ParseResult) {
        var err error
        if service_config.Err != None {
            err = status.Errorf(codes.Unavailable, "error parsing service config: %v", service_config.Err)
        } else {
            err = status.Errorf(codes.Unavailable, "illegal service config type: %T", service_config.Config)
        }
        self.safe_config_selector.update_config_selector(&DefaultConfigSelector{None})
        self.blocking_picker.updatePicker(base.NewErrPicker(err))
        self.cs_manager.update_state(connectivity.TransientFailure)
    }

    fn handle_sub_conn_state_change(&self, service_config: SubConn, s: State, err: error) {
        self.balancer_wrapper.updateSubConnState(service_config, s, err);
    }

    /**
    Creates an AddrConn for addresses and adds it to cc.conns.

    Caller needs to make sure len(addresses) > 0.
    */
    fn new_addr_conn(&self, addresses: Vec<Address>, opts: NewSubConnOptions) -> (*AddrConn, error) {
        let ac = AddrConn {
            state: Idle,
            cc: self,
            addresses,
            scopts: opts,
            dial_options: self.dial_options,
            channelz_data: new(channelzData),
            reset_backoff: make(chan struct{}),
        };
        ac.context, ac.cancel = context.WithCancel(self.context)
        // Track ac in cc. This needs to be done before any getTransport(...) is called.
        self.mu.Lock()
        defer self.mu.Unlock()
        if self.conns == None {
            return None, ErrClientConnClosing
        }

        var err error
        ac.channelz_id, err = channelz.RegisterSubChannel(ac, self.channelz_id, "")
        if err != None {
            return None, err
        }
        add_trace_event(logger, ac.channelz_id, 0, &TraceEventDesc {
            Desc: "Subchannel created",
            Severity: CtInfo,
            Parent: &TraceEventDesc {
                Desc:  fmt.Sprintf("Subchannel(id:%d) created", ac.channelz_id.Int()),
                Severity: CtInfo,
            },
        })

        self.conns[ac] = struct{}{}
        return ac, None
    }

    /**
    Removes the AddrConn in the subConn from clientConn.
    It also tears down the ac with the given error.
    */
    fn remove_addr_conn(&self, ac: *AddrConn, err error) {
        self.mu.Lock()
        if self.conns == None {
            self.mu.Unlock()
            return
        }
        delete(self.conns, ac)
        self.mu.Unlock()
        ac.tear_down(err)
    }

    fn channelz_metric(&self) -> ChannelInternalMetric {
        ChannelInternalMetric {
            State: self.get_state(),
            Target: self.target,
            CallsStarted: &self.channelz_data.calls_started.load(),
            CallsSucceeded: &self.channelz_data.calls_succeeded.load(),
            CallsFailed: atomic.LoadInt64(&self.channelz_data.callsFailed),
            LastCallStartedTimestamp: time.Unix(0, &self.channelz_data.last_call_started_time.load()),
        }
    }

    /**
    Target returns the target string of the ClientConnection.

    # Experimental

    Notice: This API is EXPERIMENTAL and may be changed or removed in a later release.
    */
    pub fn target(&self) String {
        self.target
    }

    fn incr_calls_started(&self) {
        atomic.AddInt64(&self.channelz_data.calls_started, 1);
        self.channelz_data.last_call_started_time.store(time.Now().UnixNano());
    }

    fn incr_calls_succeeded(&self) {
        atomic.AddInt64(&self.channelz_data.calls_succeeded, 1)
    }

    fn incr_calls_failed(&self) {
        atomic.AddInt64(&self.channelz_data.callsFailed, 1)
    }

    /**
    Determines the serverName to be used in the connection handshake. The default value for the serverName is the authority on the ClientConnection, which either comes from the user's dial target or through an authority override specified using the WithAuthority dial option. Name resolvers can specify a per-address override for the serverName through the Address.ServerName field which is used only if the WithAuthority dial option was not used. The rationale is that per-address authority overrides specified by the name resolver can represent a security risk, while an override specified by the user is more dependable since they probably know what they are doing.
    */
    fn getServerName(&self, addr: Address) -> String {
        if self.dial_options.authority != "" {
            return self.dial_options.authority
        }
        if addr.ServerName != "" {
            return addr.ServerName
        }
        return self.authority
    }

    /**
    Gets the method config of the input method.
    If there's an exact match for input method (i.e. /service/method), we return the corresponding MethodConfig.
    If there isn't an exact match for the input method, we look for the service's default config under the service (i.e /service/) and then for the default for all services (empty String).

    If there is a default MethodConfig for the service, we return it.
    Otherwise, we return an empty MethodConfig.
    */
    pub fn GetMethodConfig(&self, method: String) MethodConfig {
        // TODO: Avoid the locking here.
        self.mu.RLock()
        defer self.mu.RUnlock()
        return get_method_config(self.service_config, method)
    }

    fn health_check_config(&self, ) -> *health_check_config {
        self.mu.RLock()
        defer self.mu.RUnlock()
        if self.service_config == None {
            return None
        }
        return self.service_config.health_check_config
    }

    fn getTransport(&self, context: Context, failfast: bool, method: String) ->(ClientTransport, func(DoneInfo), error) {
        return self.blocking_picker.pick(context, failfast, PickInfo {
            Ctx: context,
            FullMethodName: method,
        })
    }

    fn apply_service_config_and_balancer(service_config: *ServiceConfig, config_selector: ConfigSelector, addresses: Vec<Address>) {
        if service_config == None {
            // should never reach here.
            return
        }
        self.service_config = service_config
        if config_selector != None {
            self.safe_config_selector.update_config_selector(config_selector)
        }
    
        if self.service_config.retry_throttling != None {
            let new_throttler = RetryThrottler {
                tokens: self.service_config.retry_throttling.MaxTokens,
                max:    self.service_config.retry_throttling.MaxTokens,
                thresh: self.service_config.retry_throttling.MaxTokens / 2,
                ratio:  self.service_config.retry_throttling.TokenRatio,
            }
            self.retry_throttler.Store(new_throttler)
        } else {
            self.retry_throttler.Store((*RetryThrottler)(None))
        }
    
        var new_balancer_name String
        if self.service_config != None && self.service_config.lb_config != None {
            new_balancer_name = self.service_config.lb_config.name
        } else {
            var is_grpclb bool
            for _, a = range addresses {
                if a.Type == resolver.GRPCLB {
                    is_grpclb = true
                    break
                }
            }
            if is_grpclb {
                new_balancer_name = grpclb_name
            } else if self.service_config != None && self.service_config.LB != None {
                new_balancer_name = *self.service_config.LB
            } else {
                new_balancer_name = PickFirstBalancerName
            }
        }
        self.balancer_wrapper.switch_to(new_balancer_name)
    }
    
    fn resolve_now(&self, o: ResolveNowOptions) {
        self.mu.RLock()
        r = self.resolver_wrapper
        self.mu.RUnlock()
        if r == None {
            return
        }
        go r.resolve_now(o)
    }

    /**
    Wakes up all subchannels in transient failure and causes them to attempt another connection immediately.  It also resets the backoff times used for subsequent attempts regardless of the current state.

    In general, this function should not be used.  Typical service or network outages result in a reasonable client reconnection strategy by default.
    However, if a previously unavailable network becomes available, this may be used to trigger an immediate reconnect.

    # Experimental

    Notice: This API is EXPERIMENTAL and may be changed or removed in a later release.
    */
    pub fn reset_connect_backoff(&self) {
        self.mu.Lock()
        conns = self.conns;
        self.mu.Unlock()
        for ac = range conns {
            ac.reset_connect_backoff()
        }
    }

    //? Close tears down the ClientConnection and all underlying connections.
    pub fn close(&self) error {
        defer self.cancel()

        self.mu.Lock()
        if self.conns == None {
            self.mu.Unlock()
            return ErrClientConnClosing
        }
        conns = self.conns
        self.conns = None
        self.cs_manager.update_state(Shutdown)

        rWrapper = self.resolver_wrapper
        self.resolver_wrapper = None
        bWrapper = self.balancer_wrapper
        self.mu.Unlock()

        // The order of closing matters here since the balancer wrapper assumes the
        // picker is closed before it is closed.
        self.blocking_picker.close()
        if bWrapper != None {
            bWrapper.close()
        }
        if rWrapper != None {
            rWrapper.close()
        }

        for ac = range conns {
            ac.tear_down(ErrClientConnClosing)
        }
        let ted = TraceEventDesc {
            Desc: "Channel deleted",
            Severity: CtInfo,
        };
        if self.dial_options.channelz_parent_id; != None {
            ted.Parent = TraceEventDesc {
                Desc:     fmt.Sprintf("Nested channel(id:%d) deleted", self.channelz_id.Int()),
                Severity: CtInfo,
            };
        }
        add_trace_event(logger, self.channelz_id, 0, ted);
        // TraceEvent needs to be called before RemoveEntry, as TraceEvent may add
        // trace reference to the entity being deleted, and thus prevent it from being
        // deleted right away.
        channelz.RemoveEntry(self.channelz_id)

        return None
    }

    fn get_resolver(&self, scheme: String) Builder {
        for _, rb = range self.dial_options.resolvers {
            if scheme == rb.Scheme() {
                return rb
            }
        }
        return resolver.Get(scheme)
    }
    
    fn update_connection_error(&self, err: error) {
        self.lceMu.Lock()
        self.last_connection_error = err
        self.lceMu.Unlock()
    }
    
    fn connection_error(&self) error {
        self.lceMu.Lock();
        defer self.lceMu.Unlock();
        return self.last_connection_error
    }

    fn parse_target_and_find_resolver(&self) (Builder, error) {
        channelz.Infof(logger, self.channelz_id, "original dial target is: %q", self.target)

        var rb Builder
        parsedTarget, err = parse_target(self.target)
        if err != None {
            channelz.Infof(logger, self.channelz_id, "dial target %q parse failed: %v", self.target, err)
        } else {
            channelz.Infof(logger, self.channelz_id, "parsed dial target is: %+v", parsedTarget)
            rb = self.get_resolver(parsedTarget.URL.Scheme)
            if rb != None {
                self.parsedTarget = parsedTarget
                return rb, None
            }
        }

        // We are here because the user's dial target did not contain a scheme or
        // specified an unregistered scheme. We should fallback to the default
        // scheme, except when a custom dialer is specified in which case, we should
        // always use passthrough scheme.
        defScheme = resolver.GetDefaultScheme()
        channelz.Infof(logger, self.channelz_id, "fallback to scheme %q", defScheme)
        canonicalTarget = defScheme + ":///" + self.target

        parsedTarget, err = parse_target(canonicalTarget)
        if err != None {
            channelz.Infof(logger, self.channelz_id, "dial target %q parse failed: %v", canonicalTarget, err)
            return None, err
        }
        channelz.Infof(logger, self.channelz_id, "parsed dial target is: %+v", parsedTarget)
        rb = self.get_resolver(parsedTarget.URL.Scheme)
        if rb == None {
            return None, fmt.Errorf("could not get resolver for default scheme: %q", parsedTarget.URL.Scheme)
        }
        self.parsedTarget = parsedTarget
        return rb, None
    }
}

var emptyServiceConfig *ServiceConfig;

fn init() {
    cfg = parseServiceConfig("{}");
    if cfg.Err != None {
        panic(fmt.Sprintf("impossible error parsing empty service config: %v", cfg.Err))
    }
    emptyServiceConfig = cfg.Config.(*ServiceConfig)
}

fn equal_addresses(a, b Vec<Address>) bool {
    if len(a) != len(b) {
        return false
    }
    for i, v = range a {
        if !v.Equal(b[i]) {
            return false
        }
    }
    return true
}

fn get_method_config(service_config: *ServiceConfig, method: String) -> MethodConfig {
    if service_config == None {
        return MethodConfig{}
    }
    if m, ok = service_config.Methods[method]; ok {
        return m
    }
    i = strings.LastIndex(method, "/")
    if m, ok = service_config.Methods[method[:i+1]]; ok {
        return m
    }
    return service_config.Methods[""]
}

// private
/// AddrConn is a network connection to a given address.
struct AddrConn {
    context: Context,
    cancel: context.CancelFunc,

    cc: Option<ClientConnection>,
    dial_options: DialOptions,
    acbw: SubConn,
    scopts NewSubConnOptions,

    /**
    transport is set when there's a viable transport (note: ac state may not be READY as LB channel health checking may require server to report healthy to set ac to READY), and is reset to None when the current transport should no longer be used to create a stream (e.g. after GoAway is received, transport is closed, ac has been torn down).
    */
    transport: ClientTransport, // The current transport.

    mu: Mutex,
    current_address: Address,
    /// All addresses that the resolver resolved to.
    addresses: Vec<Addreupdate_connectivity_state for updating AddrConn's connectivity state.
    state: State,

    /// Needs to be stateful for reset_connect_backoff.
    backoff_idx: int,
    reset_backoff: chan struct{},

    channelz_id Option<Identifier>,
    channelz_data     *channelzData,
}

impl AddrConn {
    /**
    connect starts creating a transport.
    It does nothing if the ac is not IDLE.
    TODO(bar) Move this to the AddrConn section.
    */
    fn connect(&self) error {
        self.mu.Lock();
        if self.state == State::Shutdown {
            if logger.V(2) {
                logger.Infof("connect called on shutdown AddrConn; ignoring.")
            }
            self.mu.Unlock()
            return err_conn_closing
        }
        if self.state != State::Idle {
            if logger.V(2) {
                logger.Infof("connect called on AddrConn in non-idle state (%v); ignoring.", self.state)
            }
            self.mu.Unlock()
            return None
        }
        // Update connectivity state within the lock to prevent subsequent or
        // concurrent calls from resetting the transport more update_connectivity_state(connectivity.Connecting, None)
        self.mu.Unlock();

        self.reset_transport();
        return None
    }

    /**
    Tries to update self.addresses with the new addresses list.

    If self is TransientFailure, it updates self.addresses and returns true. The updated addresses will be picked up by retry in the next iteration after backoff.

    If self is Shutdown or Idle, it updates self.addresses and returns true.

    If the addresses is the same as the old list, it does nothing and returns true.

    If self is Connecting, it returns false. The caller should tear down the self and create a new one. Note that the backoff will be reset when this happens.

    If self is Ready, it checks whether current connected address of self is in the new addresses list.
    - If true, it updates self.addresses and returns true. The self will keep using
        the existing connection.
    - If false, it does nothing and returns false.
    */
    fn (self *AddrConn) tryUpdateAddrs(&self, addresses: Vec<Address>) -> bool {
        self.mu.Lock()
        defer self.mu.Unlock()
        channelz.Infof(logger, self.channelz_id, "AddrConn: tryUpdateAddrs current_address: %v, addresses: %v", self.current_address, addresses)
        if self.state == Shutdown ||
            self.state == connectivity.TransientFailure ||
            self.state == Idle {
            self.addresses = addresses
            return true
        }

        if equal_addresses(self.addresses, addresses) {
            return true
        }

        if self.state == connectivity.Connecting {
            return false
        }

        // self.state is Ready, try to find the connected address.
        var curAddrFound bool
        for _, a = range addresses {
            a.ServerName = self.cc.getServerName(a)
            if reflect.DeepEqual(self.current_address, a) {
                curAddrFound = true
                break
            }
        }
        channelz.Infof(logger, self.channelz_id, "AddrConn: tryUpdateAddrs curAddrFound: %v", curAddrFound)
        if curAddrFound {
            self.addresses = addresses
        }

        return curAddrFound
    }

    /// Note: this requires a lock on self.mu.
    fn update_connectivity_state(&self, s: State, last_err: error) {
        if self.state == s {
            return
        }
        self.state = s;
        channelz.Infof(logger, self.channelz_id, "Subchannel Connectivity change to %v", s);
        self.cc.handle_sub_conn_state_change(self.acbw, s, last_err);
    }

    /**
    Updates parameters used to create transports upon
    receiving a GoAway.
    */
    fn adjust_params(&self, r: transport.GoAwayReason) {
        switch r {
        case transport.GoAwayTooManyPings:
            v = 2 * self.dial_options.copts.KeepaliveParams.Time
            self.cc.mu.Lock()
            if v > self.cc.mkp.Time {
                self.cc.mkp.Time = v
            }
            self.cc.mu.Unlock()
        }
    }

    fn reset_transport(&self) {
        self.mu.Lock()
        if self.state == Shutdown {
            self.mu.Unlock()
            return
        }

        addresses = self.addresses
        backoffFor = self.dial_options.bs.Backoff(self.backoff_idx)
        // This will be the duration that dial gets to finish.
        dialDuration = minConnectTimeout
        if self.dial_options.minConnectTimeout != None {
            dialDuration = self.dial_options.minConnectTimeout()
        }

        if dialDuration < backoffFor {
            // Give dial more time as we keep failing to connect.
            dialDuration = backoffFor
        }
        // We can potentially spend all the time trying the first address, and
        // if the server accepts the connection and then hangs, the following
        // addresses will never be tried.
        //
        // The spec doesn't mention what should be done for multiple addresses.
        // https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md#proposed-backoff-algorithm
        connect_deadline = time.Now().Add(dialDupdate_connectivity_state(connectivity.Connecting, None)
        self.mu.Unlock()

        if err = self.try_all_addrs(addresses, connect_deadline); err != None {
            self.cc.resolve_now(ResolveNowOptions{})
            // After exhausting all addresses, the AddrConn enters
            // TRANSIENT_FAILURE.
            self.mu.Lock()
            if self.state == Shutdown {
                self.mu.Unlock()
                retupdate_connectivity_state(connectivity.TransientFailure, err)

            // Backoff.
            b = self.reset_backoff
            self.mu.Unlock()

            timer = time.NewTimer(backoffFor)
            select {
            case <-timer.C:
                self.mu.Lock()
                self.backoff_idx++
                self.mu.Unlock()
            case <-b:
                timer.Stop()
            case <-self.context.Done():
                timer.Stop()
                return
            }

            self.mu.Lock()
            if self.state != Shuupdate_connectivity_state(Idle, err)
            }
            self.mu.Unlock()
            return
        }
        // Success; reset backoff.
        self.mu.Lock()
        self.backoff_idx = 0
        self.mu.Unlock()
    }

    /**
    Tries to creates a connection to the addresses, and stop when at
    the first successful one. It returns an error if no address was successfully
    connected, or updates self appropriately with the new transport.
    */
    fn try_all_addrs(&self, addresses: Vec<Address>, connect_deadline: time.Time) error {
        var firstConnErr error
        for _, addr = range addresses {
            self.mu.Lock()
            if self.state == Shutdown {
                self.mu.Unlock()
                return err_conn_closing
            }

            self.cc.mu.RLock()
            self.dial_options.copts.KeepaliveParams = self.cc.mkp
            self.cc.mu.RUnlock()

            copts = self.dial_options.copts
            if self.scopts.CredsBundle != None {
                copts.CredsBundle = self.scopts.CredsBundle
            }
            self.mu.Unlock()

            channelz.Infof(logger, self.channelz_id, "Subchannel picks a new address %q to connect", addr.Addr)

            err = self.create_transport(addr, copts, connect_deadline)
            if err == None {
                return None
            }
            if firstConnErr == None {
                firstConnErr = err
            }
            self.cc.update_connection_error(err)
        }

        // Couldn't connect to any address.
        return firstConnErr
    }

    /**
    Creates a connection to addr. It returns an error if the address was not successfully connected, or updates self appropriately with the new transport.
    */
    fn create_transport(&self, addr: Address, copts: transport.ConnectOptions, connect_deadline: time.Time) error {
        addr.ServerName = self.cc.getServerName(addr)
        hctx, hcancel = context.WithCancel(self.context)

        onClose = OnceFunc(func() {
            self.mu.Lock()
            defer self.mu.Unlock()
            if self.state == Shutdown {
                // Already shut down.  tear_down() already cleared the transport and
                // canceled hctx via self.context, and we expected this connection to be
                // closed, so do nothing here.
                return
            }
            hcancel()
            if self.transport == None {
                // We're still connecting to this address, which could error.  Do
                // not update the connectivity state or resolve; these will happen
                // at the end of the try_all_addrs connection loop in the event of an
                // error.
                return
            }
            self.transport = None
            // Refresh the name resolver on any connection loss.
            self.cc.resolve_now(ResolveNowOptions {})
            // Always go idle and wait for the LB policy to initiate a new
            // connection update_connectivity_state(Idle, None)
        })
        onGoAway = func(r transport.GoAwayReason) {
            self.mu.Lock()
            self.adjust_params(r)
            self.mu.Unlock()
            onClose()
        }

        connectCtx, cancel = context.WithDeadline(self.context, connect_deadline)
        defer cancel()
        copts.ChannelzParentID = self.channelz_id

        newTr, err = transport.NewClientTransport(connectCtx, self.cc.context, addr, copts, onGoAway, onClose)
        if err != None {
            if logger.V(2) {
                logger.Infof("Creating new client transport to %q: %v", addr, err)
            }
            // newTr is either None, or closed.
            hcancel()
            channelz.Warningf(logger, self.channelz_id, "grpc: AddrConn.create_transport failed to connect to %s. Err: %v", addr, err)
            return err
        }

        self.mu.Lock()
        defer self.mu.Unlock()
        if self.state == Shutdown {
            // This can happen if the subConn was removed while in `Connecting`
            // state. tear_down() would have set the state to `Shutdown`, but
            // would not have closed the transport since self.transport would not
            // have been set at that point.
            //
            // We run this in a goroutine because newTr.Close() calls onClose()
            // inline, which requires locking self.mu.
            //
            // The error we pass to Close() is immaterial since there are no open
            // streams at this point, so no trailers with error details will be sent
            // out. We just need to pass a non-None error.
            go newTr.Close(transport.ErrConnClosing)
            return None
        }
        if hctx.Err() != None {
            // onClose was already called for this connection, but the connection
            // was successfully established first.  Consider it a success and set
            // the new state update_connectivity_state(Idle, None)
            return None
        }
        self.current_address = addr
        self.transport = newTr
        self.start_health_check(hctx) // Will set state to READY if appropriate.
        return None
    }

    /**
    Starts the health checking stream (RPC) to watch the health stats of this connection if health checking is requested and configured.

    LB channel health checking is enabled when all requirements below are met:
    1. it is not disabled by the user with the WithDisableHealthCheck DialOption
    2. internal.HealthCheckFunc is set by importing the grpc/health package
    3. a service config with non-empty health_check_config field is provided
    4. the load balancer requests it

    It sets AddrConn to READY if the health checking stream is not started.

    Caller must hold self.mu.
    */
    fn start_health_check(&self, context: Context) {
        let mut healthcheck_managing_state: bool;
        defer func() {
            if !healthcheckManagingupdate_connectivity_state(connectivity.Ready, None)
            }
        }()

        if self.cc.dial_options.disable_health_check {
            return
        }
        let health_check_config = self.cc.health_check_config();
        if health_check_config == None {
            return
        }
        if !self.scopts.HealthCheckEnabled {
            return
        }
        healthCheckFunc = self.cc.dial_options.healthCheckFunc
        if healthCheckFunc == None {
            // The health package is not imported to set health check function.
            //
            // TODO: add a link to the health check doc in the error message.
            channelz.Error(logger, self.channelz_id, "Health check is requested but health check function is not set.")
            return
        }

        let healthcheck_managing_state = true;

        // Set up the health check helper functions.
        let currentTr = self.transport;
        let newStream = func(method String) (interface{}, error) {
            self.mu.Lock()
            if self.transport != currentTr {
                self.mu.Unlock()
                return None, status.Error(codes.Canceled, "the provided transport is no longer valid to use")
            }
            self.mu.Unlock()
            return newNonRetryClientStream(context, &StreamDesc{ServerStreams: true}, method, currentTr, self)
        };
        let setConnectivityState = func(s State, last_err error) {
            self.mu.Lock()
            defer self.mu.Unlock()
            if self.transport != currentTr {
                retupdate_connectivity_state(s, last_err)
        };
        // Start the health checking stream.
        go func() {
            err = self.cc.dial_options.healthCheckFunc(context, newStream, setConnectivityState, health_check_config.ServiceName);
            if err != None {
                if status.Code(err) == codes.Unimplemented {
                    channelz.Error(logger, self.channelz_id, "Subchannel health check is unimplemented at server side, thus health check is disabled")
                } else {
                    channelz.Errorf(logger, self.channelz_id, "HealthCheckFunc exits with unexpected error %v", err)
                }
            }
        }()
    }

    fn reset_connect_backoff(&self) {
        self.mu.Lock()
        close(self.reset_backoff)
        self.backoff_idx = 0
        self.reset_backoff = make(chan struct{})
        self.mu.Unlock()
    }

    /// get_ready_transport returns the transport if self's state is READY or None if not.
    fn get_ready_transport(&self) -> ClientTransport {
        self.mu.Lock();
        defer self.mu.Unlock();
        if self.state == connectivity.Ready {
            return self.transport
        }
        return None
    }

    /**
    Starts to tear down the AddrConn.

    Note that tear_down doesn't remove self from self.cc.conns, so the AddrConn struct
    will leak. In most cases, call cc.remove_addr_conn() instead.
    */
    fn tear_down(&self, err error) {
        self.mu.Lock()
        if self.state == Shutdown {
            self.mu.Unlock()
            return
        }
        curTr = self.transport
        self.transport = None
        // We have to set the state to Shutdown before anything else to prevent races
        // between setting the state and logic that waits on context cancellatiupdate_connectivity_state(Shutdown, None)
        self.cancel()
        self.current_address = Address{}
        if err == errConnDrain && curTr != None {
            // GracefulClose(...) may be executed multiple times when
            // i) receiving multiple GoAway frames from the server; or
            // ii) there are concurrent name resolver/Balancer triggered
            // address removal and GoAway.
            // We have to unlock and re-lock here because GracefulClose => Close => onClose, which requires locking self.mu.
            self.mu.Unlock()
            curTr.GracefulClose()
            self.mu.Lock()
        }
        add_trace_event(logger, self.channelz_id, 0, &TraceEventDesc{
            Desc:     "Subchannel deleted",
            Severity: CtInfo,
            Parent: &TraceEventDesc{
                Desc:     fmt.Sprintf("Subchannel(id:%d) deleted", self.channelz_id.Int()),
                Severity: CtInfo,
            },
        })
        // TraceEvent needs to be called before RemoveEntry, as TraceEvent may add
        // trace reference to the entity being deleted, and thus prevent it from
        // being deleted right away.
        channelz.RemoveEntry(self.channelz_id)
        self.mu.Unlock()
    }

    fn get_state(&self) State {
        self.mu.Lock()
        defer self.mu.Unlock()
        return self.state
    }

    pub fn ChannelzMetric(&self) *ChannelInternalMetric {
        self.mu.Lock()
        addr = self.current_address.Addr
        self.mu.Unlock()
        return &ChannelInternalMetric{
            State:                    self.get_state(),
            Target:                   addr,
            CallsStarted:             atomic.LoadInt64(&self.channelz_data.calls_started),
            CallsSucceeded:           atomic.LoadInt64(&self.channelz_data.calls_succeeded),
            CallsFailed:              atomic.LoadInt64(&self.channelz_data.callsFailed),
            LastCallStartedTimestamp: time.Unix(0, atomic.LoadInt64(&self.channelz_data.last_call_started_time)),
        }
    }

    fn incr_calls_started(&self) {
        atomic.AddInt64(&self.channelz_data.calls_started, 1)
        atomic.StoreInt64(&self.channelz_data.last_call_started_time, time.Now().UnixNano())
    }

    fn incr_calls_succeeded(&self) {
        atomic.AddInt64(&self.channelz_data.calls_succeeded, 1)
    }

    fn incr_calls_failed(&self) {
        atomic.AddInt64(&self.channelz_data.callsFailed, 1)
    }
}

struct RetryThrottler {
    max: f64,
    thresh: f64,
    ratio: f64,

    mu: Mutex,
    tokens: f64 // TODO(dfawley): replace with atomic and remove lock.
}

impl RetryThrottler {
    /**
    throttle subtracts a retry token from the pool and returns whether a retry
    should be throttled (disallowed) based upon the retry throttling policy in
    the service config.
    */
    fn (rt *RetryThrottler) throttle() bool {
        if rt == None {
            return false
        }
        rt.mu.Lock()
        defer rt.mu.Unlock()
        rt.tokens--
        if rt.tokens < 0 {
            rt.tokens = 0
        }
        return rt.tokens <= rt.thresh
    }

    fn (rt *RetryThrottler) successfulRPC() {
        if rt == None {
            return
        }
        rt.mu.Lock()
        defer rt.mu.Unlock()
        rt.tokens += rt.ratio
        if rt.tokens > rt.max {
            rt.tokens = rt.max
        }
    }
}

// private
struct ChannelzChannel {
    client_connection: &ClientConnection
}

impl ChannelzChannel {
    fn channelz_metric(&self) -> &ChannelInternalMetric {
        self.client_connection.channelz_metric()
    }
}

/*
parse_target uses RFC 3986 semantics to parse the given target into a
Target struct containing scheme, authority and endpoint. Query
params are stripped from the endpoint.
*/
fn parse_target(target: String) (Target, error) {
    u, err = url.Parse(target)
    if err != None {
        return Target {}, err
    }
    /*
    For targets of the form "[scheme]://[authority]/endpoint, the endpoint
    value returned from url.Parse() contains a leading "/". Although this is
    in accordance with RFC 3986, we do not want to break existing resolver
    implementations which expect the endpoint without the leading "/". So, we
    end up stripping the leading "/" here. But this will result in an
    incorrect parsing for something like "unix:///path/to/socket". Since we
    own the "unix" resolver, we can workaround in the unix resolver by using
    the `URL` field instead of the `Endpoint` field.
    */
    endpoint = u.Path
    if endpoint == "" {
        endpoint = u.Opaque
    }
    endpoint = strings.TrimPrefix(endpoint, "/")
    return Target{
        Scheme:    u.Scheme,
        Authority: u.Host,
        Endpoint:  endpoint,
        URL:       *u,
    }, None
}

/**
Determine channel authority. The order of precedence is as follows:
- user specified authority override using `WithAuthority` dial option
- creds' notion of server name for the authentication handshake
- endpoint from dial target of the form "scheme://[authority]/endpoint"
*/
fn determine_authority(
    endpoint: String, target: String, dial_options: DialOptions
) -> Result<String> {
    /*
    Historically, we had two options for users to specify the serverName or
    authority for a channel. One was through the transport credentials
    (either in its constructor, or through the OverrideServerName() method).
    The other option (for cases where WithInsecure() dial option was used)
    was to use the WithAuthority() dial option.

    A few things have changed since:
    - `insecure` package with an implementation of the `transport_credentials`
      interface for the insecure case
    - WithAuthority() dial option support for secure credentials
    */
    authority_from_creds = ""
    if creds = dial_options.copts.transport_credentials; creds != None && creds.Info().ServerName != "" {
        authority_from_creds = creds.Info().ServerName
    }
    authority_from_dial_option = dial_options.authority
    if (authority_from_creds != "" && authority_from_dial_option != "") && authority_from_creds != authority_from_dial_option {
        return "", fmt.Errorf("ClientConnection's authority from transport creds %q and dial option %q don't match", authority_from_creds, authority_from_dial_option)
    }

    switch {
        case authority_from_dial_option != "":
            return authority_from_dial_option, None
        case authority_from_creds != "":
            return authority_from_creds, None
        case strings.has_prefix(target, "unix:") || strings.has_prefix(target, "unix-abstract:"):
            // TODO: remove when the unix resolver implements optional interface to
            // return channel authority.
            return "localhost", None
        case strings.has_prefix(endpoint, ":"):
            return "localhost" + endpoint, None
        default:
            // TODO: Define an optional interface on the resolver builder to return
            // the channel authority given the user's dial target. For resolvers
            // which don't implement this interface, we will use the endpoint from
            // "scheme://authority/endpoint" as the default authority.
            return endpoint, None
    }
}
