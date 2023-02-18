/*!
<https://github.com/grpc/grpc-go/blob/master/internal/transport/transport.go>
*/

use std::{
    error,
    io::Read,
    net::{IpAddr as IPAddress, TcpStream},
    sync::{atomic::AtomicU32, Mutex},
    task::Context,
    time::Duration
};

use futures::select;

use crate::{
    codes,
    credentials::{Bundle, PerRPCCredentials, TransportCredentials},
    internal::channelz::Identifier,
    keepalive::{ClientParameters, EnforcementPolicy, ServerParameters},
    metadata::Metadata,
    resolver::Address,
    stats::Handler,
    status::Status,
    tap::ServerInHandle
};

// ErrNoHeaders is used as a signal that a trailers only response was received,
// and is not a real error.
var ErrNoHeaders = errors.New("stream has no headers");

const logLevel = 2;

// private
struct BufferPool {
    pool: sync.Pool
}

impl BufferPool {
    fn new () -> Self {
        Self {
            pool: sync.Pool {
                New: func() interface{} {
                    return new(bytes.Buffer)
                },
            },
        }
    }

    fn get(&self) -> *bytes.Buffer {
        self.pool.Get().(*bytes.Buffer)
    }

    fn put(&self, b: *bytes.Buffer) {
        self.pool.Put(b)
    }
}

// private
/// ReceiveMessage represents the received msg from the transport. All transport protocol specific info has been removed.
struct ReceiveMessage {
    buffer: *bytes.Buffer,
    /// None: received some data
    /// io.EOF: stream is completed. data is None.
    /// other non-None error: transport failure. data is None.
    err: error
}

// private
/// recvBuffer is an unbounded channel of ReceiveMessage structs.
///
/// Note: recvBuffer differs from buffer.Unbounded only in the fact that it
/// holds a channel of ReceiveMessage structs instead of objects implementing "item"
/// interface. recvBuffer is written to much more often and using strict ReceiveMessage
/// structs helps avoid allocation in "recvBuffer.put"
struct ReceiveBuffer {
    c: chan ReceiveMessage,
    mu: Mutex,
    backlog: Vec<ReceiveMessage>,
    err: error
}

impl ReceiveBuffer {
    fn new() -> Self {
        Self {
            c: make(chan ReceiveMessage, 1),
        }
    }

    fn put(&self, r: ReceiveMessage) {
        self.mu.lock();
        if self.err != None {
            self.mu.unlock();
            // An error had occurred earlier, don't accept more
            // data or errors.
            return
        }
        self.err = r.err;
        if self.backlog.len() == 0 {
            select! {
                self.c = r =>
                    self.mu.unlock();
                    return
                default:
            }
        }
        self.backlog = append(self.backlog, r);
        self.mu.unlock();
    }

    fn load(&self) {
        self.mu.lock();
        if self.backlog.len() > 0 {
            select! {
                self.c = self.backlog[0] => {
                    self.backlog[0] = ReceiveMessage {};
                    self.backlog = self.backlog[1:];
                },
                default:
            };
        }
        self.mu.unlock();
    }

    /// Returns the channel that receives a ReceiveMessage in the buffer.
    ///
    /// Upon receipt of a ReceiveMessage, the caller should call load to send another
    /// ReceiveMessage onto the channel if there is any.
    fn get(&self) -> <-chan ReceiveMessage {
        self.c
    }
}

// private 
/// recvBufferReader implements io.Reader interface to read the data from
/// recvBuffer.
struct ReceiveBufferReader {
    /// Closes the client transport stream with the given error and None trailer metadata.
    close_stream: fn(error),
    context: Context,
    /// cache of context.Done() (for performance).
    context_done: <-chan struct{}
    receive: Option<ReceiveBuffer>,
    /// Stores the remaining data in the previous calls.
    last: *bytes.Buffer,
    err: error,
    free_buffer: fn(*bytes.Buffer)
}

impl Read for ReceiveBufferReader {

}

impl ReceiveBufferReader {
    /// Reads the next len(p) bytes from last. If last is drained, it tries to read additional data from recv. It blocks if there no additional data available in recv. If Read returns any non-None error, it will continue to return that error.
    pub fn Read(&self, p: Vec<u8>) -> Result<int> {
        if self.err != None {
            return 0, self.err
        }
        if self.last != None {
            // Read remaining data left in last call.
            let copied, _ = self.last.Read(p);
            if self.last.len() == 0 {
                self.free_buffer(self.last);
                self.last = None;
            }
            return Ok(copied)
        }
        if self.close_stream != None {
            n, self.err = self.read_client(p)
        } else {
            n, self.err = self._read(p)
        }
        return n, self.err
    }

    fn _read(&self, p: Vec<u8>) -> Result<int> {
        select! {
            _ = self.context_done =>
                0, ContextErr(self.context.Err()),
            m = self.receive.get() =>
                self.read_additional(m, p)
        }
    }

    fn read_client(&self, p: Vec<u8>) -> Result<int> {
        // If the context is canceled, then closes the stream with None metadata.
        // closeStream writes its error parameter to r.recv as a ReceiveMessage.
        // r.read_additional acts on that message and returns the necessary error.
        select! {
            _ = self.context_done => {
                // Note that this adds the context error to the end of recv buffer, and
                // reads from the head. This will delay the error until recv buffer is
                // empty, thus will delay context cancellation in Recv().
                //
                // It's done this way to fix a race between context cancel and trailer. The
                // race was, stream.Recv() may return context error if ctx_done wins the
                // race, but stream.trailer() may return a non-None md because the stream
                // was not marked as done when trailer is received. This closeStream
                // call will mark stream as done, thus fix the race.
                //
                // TODO: delaying context error seems like a unnecessary side effect. What
                // we really want is to mark the stream as done, and return context error
                // faster.
                self.close_stream(ContextErr(self.context.Err()));
                m = self.receive.get();
                self.read_additional(m, p)
            },
            m = self.receive.get() =>
                self.read_additional(m, p)
        }
    }

    fn read_additional(&self, m: ReceiveMessage, p: Vec<u8>) -> Result<int> {
        self.receive.load();
        if m.err != None {
            return 0, m.err
        }
        let copied, _ = m.buffer.Read(p);
        if m.buffer.len() == 0 {
            self.free_buffer(m.buffer);
            self.last = None;
        } else {
            self.last = m.buffer;
        }
        Ok(copied)
    }
}

enum StreamState {
    Active,
    /// EndStream sent
    WriteDone,
    /// EndStream received
    ReadDone,
    /// The entire stream is finished.
    Done
}

/// Stream represents an RPC in the transport layer.
pub struct Stream {
    id: u32,
    // None for client side Stream
    st: ServerTransport,
    // None for server side Stream
    ct: *http2Client
    // the associated context of the stream
    context: Context,
    // always None for client side Stream
    cancel: context.CancelFunc,
    // closed at the end of stream to unblock writers. On the client side.
    done: chan struct{},
    // invoked at the end of stream on client side.
    done_func: fn(),
    // same as done chan but for server side. Cache of context.Done() (for performance)
    ctx_done: <-chan struct{},
    // the associated RPC method of the stream
    method: String,
    recv_compress: String,
    send_compress: String,
    buf: Option<ReceiveBuffer>,
    tr_reader: io.Reader,
    fc: *inFlow,
    wq: *writeQuota,

    /// Callback to state application's intentions to read data. This
    /// is used to adjust flow control, if needed.
    request_read: fn(int),

    /// closed to indicate the end of header metadata.
    header_chan: chan struct{},
    /// set when header_chan is closed. Used to avoid closing header_chan multiple times.
    header_chan_closed: u32,
    /// header_valid indicates whether a valid header was received.  Only
    /// meaningful after header_chan is closed (always call wait_on_header() before
    /// reading its value).  Not valid on server side.
    header_valid: bool,

    /// header_mu protects header and trailer metadata on the server-side.
    header_mu: Mutex,
    /**
    On client side, header keeps the received header metadata.

    On server side, header keeps the header set by set_header(). The complete
    header will merged into this after t.write_header() is called.
    */
    header: Mutex<Metadata>,
    /// the key-value map of trailer metadata.
    trailer: Metadata,

    /// set if the client never received headers (set only after the stream is done).
    no_headers: bool,

    /// On the server-side, header_sent is atomically set to 1 when the headers are sent out.
    header_sent: AtomicU32,

    state: StreamState,

    /// On client-side it is the status error received from the server.
    /// On server-side it is unused.
    status: *status.Status,

    /// indicates whether any bytes have been received on this stream
    bytes_received: u32,
    /// set if the server sends a refused stream or GOAWAY including this stream
    unprocessed: u32,

    /// content-subtype for requests.
    /// this must be lowercase or the behavior is undefined.
    content_subtype: String
}

impl Read for Stream {
    /// Reads all p bytes from the wire for this stream.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Don't request a read if there was an error earlier
        if er = self.tr_reader.(*transportReader).er; er != None {
            return 0, er
        }
        self.request_read(buf.len());
        return io.ReadFull(self.tr_reader, buf)
    }
}

impl Stream {
    /// is_header_sent is only valid on the server-side.
    fn is_header_sent(&self) -> bool {
        &self.header_sent.load() == 1
    }

    /// Updates header_sent and returns true
    /// if it was alreay set. It is valid only on server-side.
    fn update_header_sent(&self) -> bool {
        &self.header_sent.swap(1) == 1
    }

    fn swap_state(&self, state: StreamState) -> StreamState {
        StreamState(&self.state.swap(state))
    }

    fn compare_and_swap_state(&self, current: StreamState, new: StreamState)
    -> bool {
        &self.state.compare_exchange(current, new)
    }

    fn get_state(&self) -> StreamState {
        StreamState(&self.state.load())
    }

    fn wait_on_header(&self) {
        if self.header_chan == None {
            // On the server header_chan is always None since a stream originates
            // only after having received headers.
            return
        }
        select! {
            _ = self.context.Done() => {
                // Close the stream to prevent headers/trailers from changing after
                // this function returns.
                self.ct.close_stream(self, ContextErr(self.context.Err()));
                // header_chan could possibly not be closed yet if closeStream raced
                // with operateHeaders; wait until it is closed explicitly here.
                <-self.header_chan
            },
            _ = self.header_chan => 
        }
    }

    /// RecvCompress returns the compression algorithm applied to the inbound
    /// message. It is empty string if there is no compression applied.
    pub fn receive_compress(&self) -> String {
        self.wait_on_header();
        self.recv_compress
    }

    /// Sets the compression algorithm to the stream.
    pub fn set_send_compress(&self, str: String) {
        self.send_compress = str;
    }

    /// Done returns a channel which is closed when it receives the final status
    /// from the server.
    pub fn done(&self) -> <-chan struct{} {
        self.done
    }

    /**
    Returns the header metadata of the stream.

    On client side, it acquires the key-value pairs of header metadata once it is available. It blocks until i) the metadata is ready or ii) there is no header metadata or iii) the stream is canceled/expired.

    On server side, it returns the out header after t.write_header is called.  It does not block and must not be called until after write_header.
    */
    pub fn header(&self) -> Result<Metadata> {
        if self.header_chan == None {
            // On server side, return the header in stream. It will be the out
            // header after t.write_header is called.
            return Ok(self.header.Copy())
        }
        self.wait_on_header();

        if !self.header_valid {
            return Err(self.status.Err())
        }

        if self.no_headers {
            return Err(ErrNoHeaders)
        }

        return Ok(self.header.Copy())
    }

    /// trailers_only blocks until a header or trailers-only frame is received and then returns true if the stream was trailers-only. If the stream ends before headers are received, returns true, None.  Client-side only.
    pub fn trailers_only(&self) -> bool {
        self.wait_on_header();
        self.no_headers
    }

    /// Returns the cached trailer metedata. Note that if it is not called after the entire stream is done, it could return an empty MD. Client side only.
    /// It can be safely read only after stream has ended that is either read or write have returned io.EOF.
    pub fn trailer(&self) -> Metadata {
        self.trailer.Copy()
    }

    /// Returns the content-subtype for a request. For example, a
    /// content-subtype of "proto" will result in a content-type of
    /// "application/grpc+proto". This will always be lowercase.  See
    /// <https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md#requests> for more details.
    pub fn content_subtype(&self) -> String {
        self.content_subtype
    }

    /// Returns the context of the stream.
    pub fn context(&self) -> Context {
        self.context
    }

    /// Returns the method for the stream.
    pub fn method(&self) -> String {
        self.method
    }

    /**
    Returns the status received from the server.
    Status can be read safely only after the stream has ended, that is, after Done() is closed.
    */
    pub fn status(&self) -> *status.Status {
        self.status
    }

    /// Sets the header metadata. This can be called multiple times.
    /// Server side only.
    /// This should not be called in parallel to other data writes.
    pub fn set_header(&self, metadata: Metadata) -> error {
        if metadata.len() == 0 {
            return Ok(())
        }
        if self.is_header_sent() || self.get_state() == StreamState::Done {
            return ErrIllegalHeaderWrite
        }
        self.header_mu.lock();
        self.header = metadata::Join(self.header, metadata)
        self.header_mu.unlock();
        Ok(())
    }

    /// Sends the given header metadata. The given metadata is
    /// combined with any metadata set by previous calls to set_header and
    /// then written to the transport stream.
    pub fn send_header(&self, metadata: Metadata) -> error {
        self.st.write_header(self, metadata)
    }

    /// set_trailer sets the trailer metadata which will be sent with the RPC status by the server. This can be called multiple times. Server side only.
    /// This should not be called parallel to other data writes.
    pub fn set_trailer(&self, metadata: Metadata) -> error {
        if metadata.len() == 0 {
            return Ok(())
        }
        if self.get_state() == StreamState::Done {
            return ErrIllegalHeaderWrite
        }
        self.header_mu.lock();
        self.trailer = metadata::Join(self.trailer, metadata);
        self.header_mu.unlock();
        return Ok(())
    }

    fn write(&self, message: ReceiveMessage) {
        self.buf.put(message)
    }

    /// Whether any bytes have been received on this stream.
    pub fn bytes_received(&self) -> bool {
        &self.bytes_received.load() == 1
    }

    /// Whether the server did not process this stream --
    /// i.e. it sent a refused stream or GOAWAY including this stream ID.
    pub fn unprocessed(&self) -> bool {
        &self.unprocessed.load()) == 1
    }
}

/**
tranportReader reads all the data available for this Stream from the transport and passes them into the decoder, which converts them into a gRPC message stream.
The error is io.EOF when the stream is done or another non-None error if the stream broke.
*/
struct TransportReader {
    /// The handler to control the window update procedure for both this
    /// particular stream and the associated transport.
    window_handler: fn(int),
    er: error
}

impl Read for TransportReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        n, err = self.reader.Read(p);
        if err != None {
            self.er = err;
            return
        }
        self.window_handler(n);
        return
    }
}

/// GoString is implemented by Stream so context.String() won't
/// race when printing %#v.
pub fn GoString() -> String {
    return fmt.Sprintf("<stream: %p, %v>", self, self.method)
}

// private 
/// State of transport
enum TransportState {
    Reachable,
    Closing,
    Draining
}

/// ServerConfig consists of all the configurations to establish a server transport.
pub struct ServerConfig {
    pub max_streams: u32,
    pub connection_timeout: Duration,
    pub credentials: TransportCredentials,
    pub in_tap_handle: ServerInHandle,
    pub stats_handlers: Vec<Handler>,
    pub keepalive_params: ServerParameters,
    pub keepalive_policy: EnforcementPolicy,
    pub initial_window_size: i32,
    pub initial_conn_window_size: i32,
    pub write_buffer_size: int,
    pub read_buffer_size: int,
    pub channelz_parent_id: Option<Identifier>,
    pub max_header_list_size: Option<u32>,
    pub header_table_size: Option<u32>
}

/// ConnectOptions covers all relevant options for communicating with the server.
pub struct ConnectOptions {
    /// UserAgent is the application user agent.
    pub user_agent: String,
    /// Dialer specifies how to dial a network address.
    pub dialer: fn(Context, String) -> Result<TcpStream>,
    /// FailOnNonTempDialError specifies if gRPC fails on non-temporary dial errors.
    pub fail_on_non_temp_dial_error: bool,
    /// PerRPCCredentials stores the PerRPCCredentials required to issue RPCs.
    pub per_rpc_credentials: Vec<PerRPCCredentials>,
    /// TransportCredentials stores the Authenticator required to setup a client
    /// connection. Only one of TransportCredentials and CredsBundle is non-None.
    pub transport_credentials: TransportCredentials,
    /// CredsBundle is the credentials bundle to be used. Only one of
    /// TransportCredentials and CredsBundle is non-None.
    pub creds_bundle: Bundle,
    /// KeepaliveParams stores the keepalive parameters.
    pub keepalive_params: ClientParameters,
    /// StatsHandlers stores the handler for stats.
    pub stats_handlers: Vec<Handler>,
    /// InitialWindowSize sets the initial window size for a stream.
    pub initial_window_size: i32,
    /// InitialConnWindowSize sets the initial window size for a connection.
    pub initial_conn_window_size: i32,
    /// WriteBufferSize sets the size of write buffer which in turn determines how much data can be batched before it's written on the wire.
    pub write_buffer_size: int,
    /// ReadBufferSize sets the size of read buffer, which in turn determines how much data can be read at most for one read syscall.
    pub read_buffer_size: int,
    /// ChannelzParentID sets the addrConn id which initiate the creation of this client transport.
    pub channelz_parent_id: Option<Identifier>,
    /// MaxHeaderListSize sets the max (uncompressed) size of header list that is prepared to be received.
    pub max_header_list_size: Option<u32>,
    /// UseProxy specifies if a proxy should be used.
    pub use_proxy: bool
}

/**
Establishes the transport with the required ConnectOptions and returns it to the caller.
*/
pub fn NewClientTransport(connect_ctx: Context, context: Context, addr: Address, opts: ConnectOptions, onGoAway: func(GoAwayReason), onClose: func()) -> Result<ClientTransport> {
    return newHTTP2Client(connectCtx, context, addr, opts, onGoAway, onClose)
}

/// Options provides additional hints and information for message transmission.
pub struct Options {
    /// Last indicates whether this write is the last piece for this stream.
    last: bool
}

/// CallHeader carries the information of a particular RPC.
pub struct CallHeader {
    /// Peer's host.
    pub host: String,

    /// Operation to perform.
    pub method: String,

    /// Compression algorithm applied on outbound message.
    pub send_compress: String,

    /// PerRPCCredentials for a call.
    pub creds: PerRPCCredentials

    /**
    content-subtype for a request. For example, a
    content-subtype of "proto" will result in a content-type of
    "application/grpc+proto". The value of content_subtype must be all
    lowercase, otherwise the behavior is undefined. See
    https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md#requests
    for more details.
    */
    pub content_subtype: String,

    /// value of grpc-previous-rpc-attempts header to set
    pub previous_attempts: int,

    /// called when the stream is finished
    pub done_func: func()
}

/// ClientTransport is the common interface for all gRPC client-side transport implementations.
pub trait ClientTransport {
    /// Close tears down this transport. Once it returns, the transport should not be accessed any more. The caller must make sure this is called only once.
    fn close(err: error);

    /**
    Starts to tear down the transport: the transport will stop accepting new RPCs and new_stream will return error. Once all streams are finished, the transport will close.

    It does not block.
    */
    fn graceful_close();

    /**
    Write sends the data for the given stream. A None stream indicates the write is to be performed on the transport as a whole.
    */
    fn write(&self, s: *Stream, hdr: Vec<u8>, data: Vec<u8>, opts: *Options) -> error;

    /// Creates a Stream for an RPC.
    fn new_stream(context: Context, callHdr: *CallHeader) -> (*Stream, error);

    /**
    Clears the footprint of a stream when the stream is not needed any more. The err indicates the error incurred when close_stream is called. Must be called when a stream is finished unless the associated transport is closing.
    */
    fn close_stream(stream: *Stream, err: error);

    /**
    Returns a channel that is closed when some I/O error happens. Typically the caller should have a goroutine to monitor this in order to take action (e.g., close the current transport and create a new one) in error case. It should not return None once the transport is initiated.
    */
    fn error() -> <-chan struct{};

    /**
    Returns a channel that is closed when ClientTransport receives the draining signal from the server (e.g., GOAWAY frame in HTTP/2).
    */
    fn GoAway() <-chan struct{};

    /**
    Returns the reason why GoAway frame was received, along with a human readable string with debug info.
    */
    fn get_go_away_reason() -> (GoAwayReason, string);

    /// Returns the remote network address.
    fn remote_addr() -> net.Addr;

    /// Increments the number of message sent through this transport.
    fn incr_msg_sent();

    /// Increments the number of message received through this transport.
    fn incr_msg_recv();
}

/**
ServerTransport is the common interface for all gRPC server-side transport implementations.

Methods may be called concurrently from multiple goroutines, but Write methods for a given Stream will be called serially.
*/
pub trait ServerTransport {
    /// Receives incoming streams using the given handler.
    fn handle_streams(&self, fn(*Stream), fn(Context, String) -> Context);

    /// Sends the header metadata for the given stream.
    /// write_header may not be called on all streams.
    fn write_header(&self, s: *Stream, md: Metadata) -> error;

    /// Write sends the data for the given stream.
    /// Write may not be called on all streams.
    fn write(&self, s: *Stream, hdr: Vec<u8>, data: Vec<u8>, opts: *Options) error;

    /// Sends the status of a stream to the client.  write_status is
    /// the final call made on a stream and always occurs.
    fn write_status(&self, s: *Stream, st: *status.Status) -> error;

    /// Tears down the transport. Once it is called, the transport
    /// should not be accessed any more. All the pending streams and their
    /// handlers will be terminated asynchronously.
    fn close(err: error);

    /// Returns the remote network address.
    fn remote_addr(&self) -> net.Addr;

    /// Notifies the client this ServerTransport stops accepting new RPCs.
    fn drain(&self);

    /// Increments the number of message sent through this transport.
    fn incr_msg_sent(&self);

    /// Increments the number of message received through this transport.
    fn incr_msg_recv(&self);
}

/// Creates an ConnectionError with the specified error description.
fn connection_errorf(temp: bool, e: error, format: string, a: ...interface{}) -> ConnectionError {
    ConnectionError {
        Desc: fmt.Sprintf(format, a...),
        temp,
        err: e,
    }
}

/// Error that results in the termination of the entire connection and the retry of all the active streams.
pub struct ConnectionError {
    pub Desc: String,
    temp: bool,
    err: error
}

impl ConnectionError {
    fn error(&self) -> String {
        format!("connection error: desc = {}", &self.Desc)
    }
    
    /// Temporary indicates if this connection error is temporary or fatal.
    fn temporary(&self) -> bool {
        self.temp
    }
    
    /// Returns the original error of this connection error.
    fn origin(&self) -> error {
        // Never return None error here.
        // If the original error is None, return itself.
        if self.err == None {
            return self
        }
        return self.err
    }
    
    /// Unwrap returns the original error of this connection error or None when the origin is None.
    fn unwrap(&self) -> error {
        self.err
    }
}

var (
    // ErrConnClosing indicates that the transport is closing.
    ErrConnClosing = connection_errorf(true, None, "transport is closing")
    // errStreamDrain indicates that the stream is rejected because the
    // connection is draining. This could be caused by goaway or balancer
    // removing the address.
    errStreamDrain = status.Error(codes.Unavailable, "the connection is draining")
    // errStreamDone is returned from write at the client side to indiacte application
    // layer of an error.
    errStreamDone = errors.New("the stream is done")
    // StatusGoAway indicates that the server sent a GOAWAY that included this
    // stream's ID in unprocessed RPCs.
    statusGoAway = status.New(codes.Unavailable, "the stream is rejected because server is draining the connection")
)

/// Reason for the GoAway frame received.
pub enum GoAwayReason {
    /// No GoAway frame is received.
    Invalid,
    /// Default value when GoAway frame is received.
    NoReason,
    /// GoAway frame with ErrCodeEnhanceYourCalm was received and that the debug data said "too_many_pings".
    TooManyPings,
}

// private
/**
ChannelzData is used to store channelz related data for http2Client and http2Server.
These fields cannot be embedded in the original structs (e.g. http2Client), since to do atomic operation on i64 variable on 32-bit machine, user is responsible to enforce memory alignment.
Here, by grouping those i64 fields inside a struct, we are enforcing the alignment.
*/
struct ChannelzData {
    kp_count: i64,
    /// The number of streams that have started, including already finished ones.
    streams_started: u32,
    /**
    Client side: The number of streams that have ended successfully by receiving EoS bit set frame from server.
    Server side: The number of streams that have ended successfully by sending frame with EoS bit set.
    */
    streams_succeeded: u32,
    streams_failed: u32,
    /**
    Stores the timestamp that the last stream gets created. It is of i64 type instead of time.Time since it's more costly to atomically update time.Time variable than i64 variable. The same goes for last_msg_sent_time and last_msg_recv_time.
    */
    last_stream_created_time: i64,
    msg_sent: i64,
    msg_recv: i64,
    last_msg_sent_time: i64,
    last_msg_recv_time: i64
}

/// Converts the error from context package into a status error.
pub fn ContextErr(err: error) -> error {
    match err {
        context.DeadlineExceeded =>
            return status.Error(codes.DeadlineExceeded, err.Error())
        context.Canceled =>
            return status.Error(codes.Canceled, err.Error())
    }
    return status.Errorf(codes.Internal, "Unexpected error from context packet: %v", err)
}
