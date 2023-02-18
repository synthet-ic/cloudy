/*!
<https://github.com/grpc/grpc-go/blob/master/internal/channelz/types.go>
*/

use std::{
    collections::HashMap,
    net,
    sync::atomic,
    time
};

use crate::{
    connectivity,
    credentials
};

// private
/// entry represents a node in the channelz database.
trait Entry {
    /// Adds a child e, whose channelz id is id to child list
    fn add_child(id: i64, entry: Self);

    /// Deletes a child with channelz id to be id from child list
    fn delete_child(id: i64);

    /// Tries to delete self from channelz database. However, if child list is not empty, then deletion from the database is on hold until the last child is deleted from database.
    fn trigger_delete();

    /// Check whether trigger_delete() has been called before, and whether child list is now empty. If both conditions are met, then delete self from database.
    fn delete_self_if_ready();

    /// Returns parent ID of the entry. 0 value parent ID means no parent.
    fn get_parent_id() -> i64;
}

// private
/// DummyEntry is a fake entry to handle entry not found case.
struct DummyEntry  {
    id_not_found: i64
}

impl DummyEntry {
    fn add_child(&self, id: i64, entry: Entry) {
        /*
        Note: It is possible for a normal program to reach here under race condition.
        For example, there could be a race between ClientConn.Close() info being propagated to addrConn and http2Client. ClientConn.Close() cancel the context and result in http2Client to error. The error info is then caught by transport monitor and before addrConn.tearDown() is called in side ClientConn.Close(). Therefore, the addrConn will create a new transport. And when registering the new transport in channelz, its parent addrConn could have already been torn down and deleted from channelz tracking, and thus reach the code here.
        */
        logger.Infof("attempt to add child of type %T with id %d to a parent (id=%d) that doesn't currently exist", e, id, self.id_not_found)
    }

    fn delete_child(&self, id: i64) {
        // It is possible for a normal program to reach here under race condition.
        // Refer to the example described in add_child().
        logger.Infof("attempt to delete child with id %d from a parent (id=%d) that doesn't currently exist", id, self.id_not_found)
    }

    fn trigger_delete(&self, ) {
        logger.Warningf("attempt to delete an entry (id=%d) that doesn't currently exist", self.id_not_found)
    }

    fn delete_self_if_ready(&self) {
        // code should not reach here. delete_self_if_ready is always called on an existing entry.
    }

    fn get_parent_id(&self) -> i64 {
        return 0
    }
}

/**
ChannelMetric defines the info channelz provides for a specific Channel, which includes ChannelInternalMetric and channelz-specific data, such as channelz id,
child list, etc.
*/
pub struct ChannelMetric {
    /// ID is the channelz id of this channel.
    pub ID: i64,
    /// RefName is the human readable reference string of this channel.
    pub RefName: String,
    /// ChannelData contains channel internal metric reported by the channel through
    /// ChannelzMetric().
    pub ChannelData: Option<ChannelInternalMetric>,
    /// NestedChans tracks the nested channel type children of this channel in the format of
    /// a map from nested channel channelz id to corresponding reference string.
    pub NestedChans: HashMap<i64, String>,
    /// SubChans tracks the subchannel type children of this channel in the format of a
    /// map from subchannel channelz id to corresponding reference string.
    pub SubChans: HashMap<i64, String>,
    /// Sockets tracks the socket type children of this channel in the format of a map
    /// from socket channelz id to corresponding reference string.
    /// Note current grpc implementation doesn't allow channel having sockets directly,
    /// therefore, this is field is unused.
    pub Sockets: HashMap<i64, String>,
    /// Trace contains the most recent traced events.
    pub Trace: Option<ChannelTrace>
}

/**
SubChannelMetric defines the info channelz provides for a specific SubChannel,
which includes ChannelInternalMetric and channelz-specific data, such as
channelz id, child list, etc.
*/
pub struct SubChannelMetric {
    /// ID is the channelz id of this subchannel.
    pub ID: i64,
    /// RefName is the human readable reference string of this subchannel.
    pub RefName: String,
    /// ChannelData contains subchannel internal metric reported by the subchannel
    // through ChannelzMetric().
    pub ChannelData: *ChannelInternalMetric,
    // NestedChans tracks the nested channel type children of this subchannel in/ the format of
    /// a map from nested channel channelz id to corresponding reference string.
    /// Note current grpc implementation doesn't allow subchannel to have nested channels
    /// as children, therefore, this field is unused.
    pub NestedChans: HashMap<i64, String>,
    /// SubChans tracks the subchannel type children of this subchannel in the format of a
    /// map from subchannel channelz id to corresponding reference string.
    /// Note current grpc implementation doesn't allow subchannel to have subchannels
    /// as children, therefore, this field is unused.
    pub SubChans: HashMap<i64, String>,
    /// Sockets tracks the socket type children of this subchannel in the format of a map
    /// from socket channelz id to corresponding reference string.
    pub Sockets: HashMap<i64, String>,
    /// Trace contains the most recent traced events.
    pub Trace: *ChannelTrace
}

/**
ChannelInternalMetric defines the struct that the implementor of Channel interface should return from ChannelzMetric().
*/
pub struct ChannelInternalMetric {
    // current connectivity state of the channel.
    pub State: connectivity.State,
    // The target this channel originally tried to connect to.  May be absent
    pub Target: String,
    // The number of calls started on the channel.
    pub CallsStarted: i64,
    // The number of calls that have completed with an OK status.
    pub CallsSucceeded: i64,
    // The number of calls that have a completed with a non-OK status.
    pub CallsFailed: i64,
    // The last time a call was started on the channel.
    pub LastCallStartedTimestamp: time.Time,
}

/// ChannelTrace stores traced events on a channel/subchannel and related info.
pub struct ChannelTrace {
    /// EventNum is the number of events that ever got traced (i.e. including those that have been deleted)
    pub EventNum: i64,
    /// CreationTime is the creation time of the trace.
    pub CreationTime: time.Time,
    /// Events stores the most recent trace events (up to $maxTraceEntry, newer event will overwrite the
    /// oldest one)
    pub Events: []*TraceEvent,
}

/// TraceEvent represent a single trace event
pub struct TraceEvent {
    // Desc is a simple description of the trace event.
    Desc: String,
    // Severity states the severity of this trace event.
    Severity: Severity,
    // Timestamp is the event time.
    Timestamp: time.Time,
    // RefID is the id of the entity that gets referenced in the event. RefID is 0 if no other entity is
    // involved in this event.
    // e.g. SubChannel (id: 4[]) Created. --> RefID = 4, RefName = "" (inside [])
    RefID: i64,
    // RefName is the reference name for the entity that gets referenced in the event.
    RefName: String,
    // RefType indicates the referenced entity type, i.e Channel or SubChannel.
    RefType: RefChannelType,
}

/**
Channel is the interface that should be satisfied in order to be tracked by
channelz as Channel or SubChannel.
*/
pub trait Channel {
    fn ChannelzMetric() -> *ChannelInternalMetric
}

// private
struct DummyChannel;

pub fn (d *DummyChannel) ChannelzMetric() *ChannelInternalMetric {
    return &ChannelInternalMetric{}
}

// private
struct Channel {
    refName     String,
    c           Channel,
    closeCalled bool,
    nestedChans HashMap<i64, String>,
    subChans    HashMap<i64, String>,
    id          i64,
    pid         i64,
    cm          *channelMap,
    trace       *ChannelTrace,
    // traceRefCount is the number of trace events that reference this channel.
    // Non-zero traceRefCount means the trace of this channel cannot be deleted.
    traceRefCount i32,
}

fn (c *channel) add_child(id: i64, entry: Entry) {
    switch v := e.(type) {
    case *SubChannel:
        c.subChans[id] = v.refName
    case *channel:
        c.nestedChans[id] = v.refName
    default:
        logger.Errorf("cannot add a child (id = %d) of type %T to a channel", id, e)
    }
}

fn (c *channel) delete_child(id: i64) {
    delete(c.subChans, id)
    delete(c.nestedChans, id)
    c.delete_self_if_ready()
}

fn (c *channel) trigger_delete() {
    c.closeCalled = true
    c.delete_self_if_ready()
}

fn (c *channel) get_parent_id() i64 {
    return c.pid
}

// deleteSelfFromTree tries to delete the channel from the channelz entry relation tree, which means
// deleting the channel reference from its parent's child list.
//
// In order for a channel to be deleted from the tree, it must meet the criteria that, removal of the
// corresponding grpc object has been invoked, and the channel does not have any children left.
//
// The returned boolean value indicates whether the channel has been successfully deleted from tree.
fn (c *channel) deleteSelfFromTree() (deleted bool) {
    if !c.closeCalled || len(c.subChans)+len(c.nestedChans) != 0 {
        return false
    }
    // not top channel
    if c.pid != 0 {
        c.cm.findEntry(c.pid).delete_child(c.id)
    }
    return true
}

// deleteSelfFromMap checks whether it is valid to delete the channel from the map, which means
// deleting the channel from channelz's tracking entirely. Users can no longer use id to query the
// channel, and its memory will be garbage collected.
//
// The trace reference count of the channel must be 0 in order to be deleted from the map. This is
// specified in the channel tracing gRFC that as long as some other trace has reference to an entity,
// the trace of the referenced entity must not be deleted. In order to release the resource allocated
// by grpc, the reference to the grpc object is reset to a dummy object.
//
// deleteSelfFromMap must be called after deleteSelfFromTree returns true.
//
// It returns a bool to indicate whether the channel can be safely deleted from map.
fn (c *channel) deleteSelfFromMap() (delete bool) {
    if c.getTraceRefCount() != 0 {
        c.c = &DummyChannel{}
        return false
    }
    return true
}

// delete_self_if_ready tries to delete the channel itself from the channelz database.
// The delete process includes two steps:
//  1. delete the channel from the entry relation tree, i.e. delete the channel reference from its
//     parent's child list.
//  2. delete the channel from the map, i.e. delete the channel entirely from channelz. Lookup by id
//     will return entry not found error.
fn (c *channel) delete_self_if_ready() {
    if !c.deleteSelfFromTree() {
        return
    }
    if !c.deleteSelfFromMap() {
        return
    }
    c.cm.deleteEntry(c.id)
    c.trace.clear()
}

fn (c *channel) getChannelTrace() *ChannelTrace {
    return c.trace
}

fn (c *channel) incrTraceRefCount() {
    atomic.AddInt32(&c.traceRefCount, 1)
}

fn (c *channel) decrTraceRefCount() {
    atomic.AddInt32(&c.traceRefCount, -1)
}

fn (c *channel) getTraceRefCount() int {
    i := atomic.LoadInt32(&c.traceRefCount)
    return int(i)
}

fn (c *channel) getRefName() String {
    return c.refName
}

// private
struct SubChannel {
    refName       String,
    c             Channel,
    closeCalled   bool,
    sockets       HashMap<i64, String>,
    id            i64,
    pid           i64,
    cm            *channelMap,
    trace         *ChannelTrace,
    traceRefCount int32,
}

fn (sc *SubChannel) add_child(id: i64, entry: Entry) {
    if v, ok := e.(*normalSocket); ok {
        sc.sockets[id] = v.refName
    } else {
        logger.Errorf("cannot add a child (id = %d) of type %T to a SubChannel", id, e)
    }
}

fn (sc *SubChannel) delete_child(id: i64) {
    delete(sc.sockets, id)
    sc.delete_self_if_ready()
}

fn (sc *SubChannel) trigger_delete() {
    sc.closeCalled = true
    sc.delete_self_if_ready()
}

fn (sc *SubChannel) get_parent_id() i64 {
    return sc.pid
}

// deleteSelfFromTree tries to delete the subchannel from the channelz entry relation tree, which
// means deleting the subchannel reference from its parent's child list.
//
// In order for a subchannel to be deleted from the tree, it must meet the criteria that, removal of
// the corresponding grpc object has been invoked, and the subchannel does not have any children left.
//
// The returned boolean value indicates whether the channel has been successfully deleted from tree.
fn (sc *SubChannel) deleteSelfFromTree() (deleted bool) {
    if !sc.closeCalled || len(sc.sockets) != 0 {
        return false
    }
    sc.cm.findEntry(sc.pid).delete_child(sc.id)
    return true
}

// deleteSelfFromMap checks whether it is valid to delete the subchannel from the map, which means
// deleting the subchannel from channelz's tracking entirely. Users can no longer use id to query
// the subchannel, and its memory will be garbage collected.
//
// The trace reference count of the subchannel must be 0 in order to be deleted from the map. This is
// specified in the channel tracing gRFC that as long as some other trace has reference to an entity,
// the trace of the referenced entity must not be deleted. In order to release the resource allocated
// by grpc, the reference to the grpc object is reset to a dummy object.
//
// deleteSelfFromMap must be called after deleteSelfFromTree returns true.
//
// It returns a bool to indicate whether the channel can be safely deleted from map.
fn (sc *SubChannel) deleteSelfFromMap() (delete bool) {
    if sc.getTraceRefCount() != 0 {
        // free the grpc struct (i.e. addrConn)
        sc.c = &DummyChannel{}
        return false
    }
    return true
}

// delete_self_if_ready tries to delete the subchannel itself from the channelz database.
// The delete process includes two steps:
//  1. delete the subchannel from the entry relation tree, i.e. delete the subchannel reference from
//     its parent's child list.
//  2. delete the subchannel from the map, i.e. delete the subchannel entirely from channelz. Lookup
//     by id will return entry not found error.
fn (sc *SubChannel) delete_self_if_ready() {
    if !sc.deleteSelfFromTree() {
        return
    }
    if !sc.deleteSelfFromMap() {
        return
    }
    sc.cm.deleteEntry(sc.id)
    sc.trace.clear()
}

fn (sc *SubChannel) getChannelTrace() *ChannelTrace {
    return sc.trace
}

fn (sc *SubChannel) incrTraceRefCount() {
    atomic.AddInt32(&sc.traceRefCount, 1)
}

fn (sc *SubChannel) decrTraceRefCount() {
    atomic.AddInt32(&sc.traceRefCount, -1)
}

fn (sc *SubChannel) getTraceRefCount() int {
    i := atomic.LoadInt32(&sc.traceRefCount)
    return int(i)
}

fn (sc *SubChannel) getRefName() String {
    return sc.refName
}

// SocketMetric defines the info channelz provides for a specific Socket, which
// includes SocketInternalMetric and channelz-specific data, such as channelz id, etc.
type SocketMetric struct {
    // ID is the channelz id of this socket.
    ID i64
    // RefName is the human readable reference string of this socket.
    RefName String
    // SocketData contains socket internal metric reported by the socket through
    // ChannelzMetric().
    SocketData *SocketInternalMetric
}

// SocketInternalMetric defines the struct that the implementor of Socket interface
// should return from ChannelzMetric().
type SocketInternalMetric struct {
    // The number of streams that have been started.
    StreamsStarted i64
    // The number of streams that have ended successfully:
    // On client side, receiving frame with eos bit set.
    // On server side, sending frame with eos bit set.
    StreamsSucceeded i64
    // The number of streams that have ended unsuccessfully:
    // On client side, termination without receiving frame with eos bit set.
    // On server side, termination without sending frame with eos bit set.
    StreamsFailed i64
    // The number of messages successfully sent on this socket.
    MessagesSent     i64
    MessagesReceived i64
    // The number of keep alives sent.  This is typically implemented with HTTP/2
    // ping messages.
    KeepAlivesSent i64
    // The last time a stream was created by this endpoint.  Usually unset for
    // servers.
    LastLocalStreamCreatedTimestamp time.Time
    // The last time a stream was created by the remote endpoint.  Usually unset
    // for clients.
    LastRemoteStreamCreatedTimestamp time.Time
    // The last time a message was sent by this endpoint.
    LastMessageSentTimestamp time.Time
    // The last time a message was received by this endpoint.
    LastMessageReceivedTimestamp time.Time
    // The amount of window, granted to the local endpoint by the remote endpoint.
    // This may be slightly out of date due to network latency.  This does NOT
    // include stream level or TCP level flow control info.
    LocalFlowControlWindow i64
    // The amount of window, granted to the remote endpoint by the local endpoint.
    // This may be slightly out of date due to network latency.  This does NOT
    // include stream level or TCP level flow control info.
    RemoteFlowControlWindow i64
    // The locally bound address.
    LocalAddr net.Addr
    // The remote bound address.  May be absent.
    RemoteAddr net.Addr
    // Optional, represents the name of the remote endpoint, if different than
    // the original target name.
    RemoteName    String
    SocketOptions *SocketOptionData
    Security      credentials.ChannelzSecurityValue
}

// Socket is the interface that should be satisfied in order to be tracked by
// channelz as Socket.
type Socket interface {
    ChannelzMetric() *SocketInternalMetric
}

type listenSocket struct {
    refName String
    s       Socket
    id      i64
    pid     i64
    cm      *channelMap
}

fn (ls *listenSocket) add_child(id: i64, entry: Entry) {
    logger.Errorf("cannot add a child (id = %d) of type %T to a listen socket", id, e)
}

fn (ls *listenSocket) delete_child(id: i64) {
    logger.Errorf("cannot delete a child (id = %d) from a listen socket", id)
}

fn (ls *listenSocket) trigger_delete() {
    ls.cm.deleteEntry(ls.id)
    ls.cm.findEntry(ls.pid).delete_child(ls.id)
}

fn (ls *listenSocket) delete_self_if_ready() {
    logger.Errorf("cannot call delete_self_if_ready on a listen socket")
}

fn (ls *listenSocket) get_parent_id() i64 {
    return ls.pid
}

type normalSocket struct {
    refName String
    s       Socket
    id      i64
    pid     i64
    cm      *channelMap
}

fn (ns *normalSocket) add_child(id: i64, entry: Entry) {
    logger.Errorf("cannot add a child (id = %d) of type %T to a normal socket", id, e)
}

fn (ns *normalSocket) delete_child(id: i64) {
    logger.Errorf("cannot delete a child (id = %d) from a normal socket", id)
}

fn (ns *normalSocket) trigger_delete() {
    ns.cm.deleteEntry(ns.id)
    ns.cm.findEntry(ns.pid).delete_child(ns.id)
}

fn (ns *normalSocket) delete_self_if_ready() {
    logger.Errorf("cannot call delete_self_if_ready on a normal socket")
}

fn (ns *normalSocket) get_parent_id() i64 {
    return ns.pid
}

// ServerMetric defines the info channelz provides for a specific Server, which
// includes ServerInternalMetric and channelz-specific data, such as channelz id,
// child list, etc.
type ServerMetric struct {
    // ID is the channelz id of this server.
    ID i64
    // RefName is the human readable reference string of this server.
    RefName String
    // ServerData contains server internal metric reported by the server through
    // ChannelzMetric().
    ServerData *ServerInternalMetric
    // ListenSockets tracks the listener socket type children of this server in the
    // format of a map from socket channelz id to corresponding reference string.
    ListenSockets HashMap<i64, String>
}

// ServerInternalMetric defines the struct that the implementor of Server interface
// should return from ChannelzMetric().
type ServerInternalMetric struct {
    // The number of incoming calls started on the server.
    CallsStarted i64
    // The number of incoming calls that have completed with an OK status.
    CallsSucceeded i64
    // The number of incoming calls that have a completed with a non-OK status.
    CallsFailed i64
    // The last time a call was started on the server.
    LastCallStartedTimestamp time.Time
}

// Server is the interface to be satisfied in order to be tracked by channelz as
// Server.
type Server interface {
    ChannelzMetric() *ServerInternalMetric
}

type server struct {
    refName       String
    s             Server
    closeCalled   bool
    sockets       HashMap<i64, String>
    listenSockets HashMap<i64, String>
    id            i64
    cm            *channelMap
}

fn (s *server) add_child(id: i64, entry: Entry) {
    switch v := e.(type) {
    case *normalSocket:
        s.sockets[id] = v.refName
    case *listenSocket:
        s.listenSockets[id] = v.refName
    default:
        logger.Errorf("cannot add a child (id = %d) of type %T to a server", id, e)
    }
}

fn (s *server) delete_child(id: i64) {
    delete(s.sockets, id)
    delete(s.listenSockets, id)
    s.delete_self_if_ready()
}

fn (s *server) trigger_delete() {
    s.closeCalled = true
    s.delete_self_if_ready()
}

fn (s *server) delete_self_if_ready() {
    if !s.closeCalled || len(s.sockets)+len(s.listenSockets) != 0 {
        return
    }
    s.cm.deleteEntry(s.id)
}

fn (s *server) get_parent_id() i64 {
    return 0
}

// private
trait TracedChannel {
    fn getChannelTrace() *ChannelTrace
    fn incrTraceRefCount()
    fn decrTraceRefCount()
    fn getRefName() String
}

// private
struct ChannelTrace {
    cm          *channelMap
    createdTime time.Time
    eventCount  i64
    mu          sync.Mutex
    events      []*TraceEvent
}

fn (c *ChannelTrace) append(e *TraceEvent) {
    c.mu.Lock()
    if len(c.events) == getMaxTraceEntry() {
        del := c.events[0]
        c.events = c.events[1:]
        if del.RefID != 0 {
            // start recursive cleanup in a goroutine to not block the call originated from grpc.
            go fn() {
                // need to acquire c.cm.mu lock to call the unlocked attemptCleanup fn.
                c.cm.mu.Lock()
                c.cm.decrTraceRefCount(del.RefID)
                c.cm.mu.Unlock()
            }()
        }
    }
    e.Timestamp = time.Now()
    c.events = append(c.events, e)
    c.eventCount++
    c.mu.Unlock()
}

fn (c *ChannelTrace) clear() {
    c.mu.Lock()
    for _, e := range c.events {
        if e.RefID != 0 {
            // caller should have already held the c.cm.mu lock.
            c.cm.decrTraceRefCount(e.RefID)
        }
    }
    c.mu.Unlock()
}

/**
Severity is the severity level of a trace event.
The canonical enumeration of all valid values is here:
<https://github.com/grpc/grpc-proto/blob/9b13d199cc0d4703c7ea26c9c330ba695866eb23/grpc/channelz/v1/channelz.proto#L126>.
*/
pub enum Severity {
    /// Unknown severity of a trace event.
    CtUnknown,
    /// Info level severity of a trace event.
    CtInfo,
    /// Warning level severity of a trace event.
    CtWarning,
    /// Error level severity of a trace event.
    CtError,
}

// RefChannelType is the type of the entity being referenced in a trace event.
pub enum RefChannelType {
    // RefUnknown indicates an unknown entity type, the zero value for this type.
    RefUnknown,
    /// RefChannel indicates the referenced entity is a Channel.
    RefChannel,
    /// RefSubChannel indicates the referenced entity is a SubChannel.
    RefSubChannel,
    /// RefServer indicates the referenced entity is a Server.
    RefServer,
    /// RefListenSocket indicates the referenced entity is a ListenSocket.
    RefListenSocket,
    /// RefNormalSocket indicates the referenced entity is a NormalSocket.
    RefNormalSocket
}

var refChannelTypeToString = map[RefChannelType]String{
    RefUnknown:      "Unknown",
    RefChannel:      "Channel",
    RefSubChannel:   "SubChannel",
    RefServer:       "Server",
    RefListenSocket: "ListenSocket",
    RefNormalSocket: "NormalSocket",
}

fn (r RefChannelType) String() String {
    return refChannelTypeToString[r]
}

fn (c *ChannelTrace) dumpData() *ChannelTrace {
    c.mu.Lock()
    ct := &ChannelTrace{EventNum: c.eventCount, CreationTime: c.createdTime}
    ct.Events = c.events[:len(c.events)]
    c.mu.Unlock()
    return ct
}
