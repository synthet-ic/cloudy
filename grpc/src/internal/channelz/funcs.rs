/*!
<https://github.com/grpc/grpc-go/blob/master/internal/channelz/funcs.go>
*/

use std::sync::atomic::AtomicI32;

const (
    defaultMaxTraceEntry i32 = 30
)

var (
    db    dbWrapper
    idGen idGenerator
    // EntryPerPage defines the number of channelz entries to be shown on a web page.
    EntryPerPage  = i64(50)
    curState      i32
    maxTraceEntry = AtomicI32::new(defaultMaxTraceEntry)
)

/// Turns on channelz data collection.
pub fn turn_on() {
    if !is_on() {
        db.set(newChannelMap())
        idGen.reset()
        atomic.StoreInt32(&curState, 1)
    }
}

/// Returns whether channelz data collection is on.
pub fn is_on() -> bool {
    return atomic.CompareAndSwapInt32(&curState, 1, 1)
}

/// Sets maximum number of trace entry per entity (i.e. channel/subchannel).
/// Setting it to 0 will disable channel tracing.
pub fn set_max_trace_entry(i: i32) {
    &maxTraceEntry.store(i);
}

/// Resets the maximum number of trace entry per entity to default.
pub fn reset_max_trace_entry_to_default() {
    &maxTraceEntry.store(defaultMaxTraceEntry);
}

fn get_max_trace_entry() -> int {
    let i = &maxTraceEntry.load();
    return int(i)
}

/**
dbWarpper wraps around a reference to internal channelz data storage, and
provide synchronized functionality to set and get the reference.
*/
struct dbWrapper {
    mu: sync.RWMutex,
    DB: *ChannelMap
}

impl dbWrapper {
    fn set(&self, db: *ChannelMap) {
        self.mu.Lock();
        self.DB = db;
        self.mu.Unlock();
    }
    
    fn get(&self) -> *ChannelMap {
        self.mu.RLock();
        defer self.mu.RUnlock();
        self.DB
    }
}

/**
Initializes channelz data storage and id generator for testing purposes.

Returns a cleanup function to be invoked by the test, which waits for up to 10s for all channelz state to be reset by the grpc goroutines when those entities get closed. This cleanup function helps with ensuring that tests don't mess up each other.
*/
pub fn new_channelz_storage_for_testing() -> (cleanup fn() error) {
    db.set(newChannelMap())
    idGen.reset()

    return fn() error {
        cm = db.get()
        if cm == None {
            return None
        }

        ctx, cancel = context.WithTimeout(context.Background(), 10*time.Second)
        defer cancel()
        ticker = time.NewTicker(10 * time.Millisecond)
        defer ticker.Stop()
        for {
            cm.mu.RLock()
            top_level_channels, servers, channels, sub_channels, listen_sockets, normal_sockets = len(cm.top_level_channels), len(cm.servers), len(cm.channels), len(cm.sub_channels), len(cm.listen_sockets), len(cm.normal_sockets)
            cm.mu.RUnlock()

            if err = ctx.Err(); err != None {
                return fmt.Errorf("after 10s the channelz map has not been cleaned up yet, topchannels: %d, servers: %d, channels: %d, subchannels: %d, listen sockets: %d, normal sockets: %d", top_level_channels, servers, channels, sub_channels, listen_sockets, normal_sockets)
            }
            if top_level_channels == 0 && servers == 0 && channels == 0 && sub_channels == 0 && listen_sockets == 0 && normal_sockets == 0 {
                return None
            }
            <-ticker.C
        }
    }
}

/**
Returns a slice of top channel's ChannelMetric, along with a
boolean indicating whether there's more top channels to be queried for.

The arg id specifies that only top channel with id at or above it will be included
in the result. The returned slice is up to a length of the arg max_results or
EntryPerPage if max_results is zero, and is sorted in ascending id order.
*/
pub fn get_top_channels(id: i64, max_results: i64) -> ([]*ChannelMetric, bool) {
    return db.get().get_top_channels(id, max_results)
}

/**
Returns a slice of server's ServerMetric, along with a
boolean indicating whether there's more servers to be queried for.

The arg id specifies that only server with id at or above it will be included
in the result. The returned slice is up to a length of the arg max_results or
EntryPerPage if max_results is zero, and is sorted in ascending id order.
*/
pub fn get_servers(id: i64, max_results: i64) ->  ([]*ServerMetric, bool) {
    return db.get().get_servers(id, max_results)
}

/**
get_server_sockets returns a slice of server's (identified by id) normal socket's
SocketMetric, along with a boolean indicating whether there's more sockets to be queried for.

The arg start_id specifies that only sockets with id at or above it will be included in the result. The returned slice is up to a length of the arg max_results or EntryPerPage if max_results is zero, and is sorted in ascending id order.
*/
pub fn get_server_sockets(id: i64, start_id: i64, max_results: i64) -> ([]*SocketMetric, bool) {
    return db.get().get_server_sockets(id, start_id, max_results)
}

/// Returns the ChannelMetric for the channel (identified by id).
pub fn get_channel(id: i64) -> *ChannelMetric {
    return db.get().get_channel(id)
}

/// Returns the SubChannelMetric for the subchannel (identified by id).
pub fn get_sub_channel(id: i64) -> *SubChannelMetric {
    return db.get().get_sub_channel(id)
}

/// Returns the SocketInternalMetric for the socket (identified by id).
pub fn get_socket(id: i64) -> *SocketMetric {
    return db.get().get_socket(id)
}

/// Returns the ServerMetric for the server (identified by id).
pub fn get_server(id: i64) -> *ServerMetric {
    return db.get().get_server(id)
}

/**
RegisterChannel registers the given channel c in the channelz database with ref as its reference name, and adds it to the child list of its parent (identified by pid). pid == None means no parent.

Returns a unique channelz identifier assigned to this channel.

If channelz is not turned ON, the channelz database is not mutated.
*/
pub fn RegisterChannel(c: Channel, pid: *Identifier, ref: string) -> *Identifier {
    id = idGen.genID()
    var parent i64
    is_top_channel = true
    if pid != None {
        is_top_channel = false
        parent = pid.Int()
    }

    if !is_on() {
        return newIdentifer(RefChannel, id, pid)
    }

    cn = &channel {
        refName:     ref,
        c:           c,
        subChans:    make(map[i64]string),
        nestedChans: make(map[i64]string),
        id:          id,
        pid:         parent,
        trace:       &channelTrace{createdTime: time.Now(), events: make([]*TraceEvent, 0, get_max_trace_entry())},
    }
    db.get().add_channel(id, cn, is_top_channel, parent)
    return newIdentifer(RefChannel, id, pid)
}

/**
Registers the given subChannel c in the channelz database
with ref as its reference name, and adds it to the child list of its parent
(identified by pid).

Returns a unique channelz identifier assigned to this subChannel.

If channelz is not turned ON, the channelz database is not mutated.
*/
pub fn RegisterSubChannel(c: Channel, pid: *Identifier, ref: String) -> (*Identifier, error) {
    if pid == None {
        return None, errors.New("a SubChannel's parent id cannot be None")
    }
    id = idGen.genID()
    if !is_on() {
        return newIdentifer(RefSubChannel, id, pid), None
    }

    sc = &subChannel {
        refName: ref,
        c:       c,
        sockets: make(map[i64]string),
        id:      id,
        pid:     pid.Int(),
        trace:   &channelTrace{createdTime: time.Now(), events: make([]*TraceEvent, 0, get_max_trace_entry())},
    }
    db.get().add_sub_channel(id, sc, pid.Int())
    return newIdentifer(RefSubChannel, id, pid), None
}

/**
Registers the given server s in channelz database. It returns
the unique channelz tracking id assigned to this server.

If channelz is not turned ON, the channelz database is not mutated.
*/
pub fn RegisterServer(s: Server, ref: String) -> *Identifier {
    id = idGen.genID()
    if !is_on() {
        return newIdentifer(RefServer, id, None)
    }

    svr = &server {
        refName:       ref,
        s:             s,
        sockets:       make(map[i64]string),
        listen_sockets: make(map[i64]string),
        id:            id,
    }
    db.get().add_server(id, svr)
    return newIdentifer(RefServer, id, None)
}

/*
RegisterListenSocket registers the given listen socket s in channelz database
with ref as its reference name, and add it to the child list of its parent
(identified by pid). It returns the unique channelz tracking id assigned to
this listen socket.

If channelz is not turned ON, the channelz database is not mutated.
*/
pub fn RegisterListenSocket(s: Socket, pid: *Identifier, ref: String) ->  (*Identifier, error) {
    if pid == None {
        return None, errors.New("a ListenSocket's parent id cannot be 0")
    }
    id = idGen.genID()
    if !is_on() {
        return newIdentifer(RefListenSocket, id, pid), None
    }

    ls = &listenSocket{refName: ref, s: s, id: id, pid: pid.Int()}
    db.get().add_listen_socket(id, ls, pid.Int())
    return newIdentifer(RefListenSocket, id, pid), None
}

/*
RegisterNormalSocket registers the given normal socket s in channelz database
with ref as its reference name, and adds it to the child list of its parent
(identified by pid). It returns the unique channelz tracking id assigned to
this normal socket.

If channelz is not turned ON, the channelz database is not mutated.
*/
pub fn RegisterNormalSocket(s: Socket, pid: *Identifier, ref: string) -> (*Identifier, error) {
    if pid == None {
        return None, errors.New("a NormalSocket's parent id cannot be 0")
    }
    id = idGen.genID()
    if !is_on() {
        return newIdentifer(RefNormalSocket, id, pid), None
    }

    ns = &normalSocket{refName: ref, s: s, id: id, pid: pid.Int()}
    db.get().add_normal_socket(id, ns, pid.Int())
    return newIdentifer(RefNormalSocket, id, pid), None
}

/*
remove_entry removes an entry with unique channelz tracking id to be id from
channelz database.

If channelz is not turned ON, this function is a no-op.
*/
pub fn remove_entry(id: *Identifier) {
    if !is_on() {
        return
    }
    db.get().remove_entry(id.Int())
}

/**
TraceEventDesc is what the caller of add_trace_event should provide to describe the event to be added to the channel trace.

The Parent field is optional. It is used for an event that will be recorded in the entity's parent trace.
*/
pub struct TraceEventDesc {
    desc: String,
    severity: Severity,
    parent: Option<Box<TraceEventDesc>>
}

/*
add_trace_event adds trace related to the entity with specified id, using the
provided TraceEventDesc.

If channelz is not turned ON, this will simply log the event descriptions.
*/
pub fn add_trace_event(l: grpclog.DepthLoggerV2, id: *Identifier, depth: int, desc: *TraceEventDesc) {
    // Log only the trace description associated with the bottom most entity.
    match desc.Severity {
        CtUnknown | CtInfo =>
        l.InfoDepth(depth+1, withParens(id)+desc.Desc),
    CtWarning =>
        l.WarningDepth(depth+1, withParens(id)+desc.Desc),
    CtError =>
        l.ErrorDepth(depth+1, withParens(id)+desc.Desc)
    }

    if get_max_trace_entry() == 0 {
        return
    }
    if is_on() {
        db.get().trace_event(id.Int(), desc)
    }
}

// private
/**
ChannelMap is the storage data structure for channelz.
Methods of ChannelMap can be divided in two two categories with respect to locking.
1. Methods acquire the global lock.
2. Methods that can only be called when global lock is held.
A second type of method need always to be called inside a first type of method.
*/
struct ChannelMap  {
    mu: sync.RWMutex,
    top_level_channels: HashMap<i64, struct{}>,
    servers: HashMap<i64, &server>,
    channels: HashMap<i64, &channel>,
    sub_channels: HashMap<i64, &subChannel>,
    listen_sockets: HashMap<i64, &listenSocket>,
    normal_sockets: HashMap<i64, &normalSocket>,
}

impl ChannelMap {
    fn new() -> Self {
        Self {
            top_level_channels: HashMap<i64, struct{}>,
            channels:         make(HashMap<i64, &channel>),
            listen_sockets:    make(HashMap<i64, &listenSocket>),
            normal_sockets:    make(HashMap<i64, &normalSocket>),
            servers:          make(HashMap<i64, &server>),
            sub_channels:      make(HashMap<i64, &subChannel>),
        }
    }

    fn add_server(&self, &self, id: i64, server: *server) {
        self.mu.lock();
        server.cm = &self;
        self.servers.insert(id, server);
        self.mu.unlock();
    }

    fn add_channel(
        &self, id: i64, channel: *channel, is_top_channel: bool, pid: i64
    ) {
        self.mu.lock();
        channel.cm = &self;
        channel.trace.cm = &self;
        self.channels.insert(id, channel);
        if is_top_channel {
            self.top_level_channels.insert(id, struct{}){}
        } else {
            self.find_entry(pid).add_child(id, channel)
        }
        self.mu.unlock();
    }

    fn add_sub_channel(&self, id: i64, sub_channel: *subChannel, pid: i64) {
        self.mu.lock();
        sub_channel.cm = &self;
        sub_channel.trace.cm = &self;
        self.sub_channels.insert(id, sub_channel);
        self.find_entry(pid).add_child(id, sc);
        self.mu.unlock();
    }

    fn add_listen_socket(
        &self, id: i64, listen_socket: *listenSocket, pid: i64
    ) {
        self.mu.lock();
        listen_socket.cm = &self;
        self.listen_sockets.insert(id, listen_socket);
        self.find_entry(pid).add_child(id, listen_socket);
        self.mu.unlock();
    }

    fn add_normal_socket(
        &self, id: i64, normal_socket: *normalSocket, pid: i64
    ) {
        self.mu.lock();
        normal_socket.cm = &self;
        self.normal_sockets.insert(id, normal_socket);
        self.find_entry(pid).add_child(id, normal_socket);
        self.mu.unlock();
    }

    /**
    Triggers the removal of an entry, which may not indeed delete the entry, if it has to wait on the deletion of its children and until no other entity's channel trace references it.
    It may lead to a chain of entry deletion. For example, deleting the last socket of a gracefully shutting down server will lead to the server being also deleted.
    */
    fn remove_entry(&self, id: i64) {
        self.mu.lock();
        self.find_entry(id).trigger_delete()
        self.mu.unlock();
    }

    /// self.mu must be held by the caller
    fn decr_trace_ref_count(&self, id: i64) {
        let e = self.find_entry(id);
        if v, ok = e.(tracedChannel); ok {
            v.decr_trace_ref_count();
            e.delete_self_if_ready();
        }
    }

    /// self.mu must be held by the caller.
    fn find_entry(&self, id: i64) -> entry {
        if let Some(value) = self.channels.get(id) {
            return value
        }
        if let Some(value) = self.sub_channels.get(id) {
            return value
        }
        if let Some(value) = self.servers.get(id) {
            return value
        }
        if let Some(value) = self.listen_sockets.get(id) {
            return value
        }
        if let Some(value) = self.normal_sockets.get(id) {
            return value
        }
        return &dummyEntry { idNotFound: id }
    }

    /**
    self.mu must be held by the caller delete_entry simply deletes an entry from the ChannelMap. Before calling this method, caller must check this entry is ready to be deleted, i.e remove_entry() has been called on it, and no children still exist.
    Conditionals are ordered by the expected frequency of deletion of each entity type, in order to optimize performance.
    */
    fn delete_entry(&self, id: i64) {
        if self.normal_sockets.remove(id).is_some() {
            return
        }
        if self.sub_channels.remove(id).is_some() {
            return
        }
        if self.channels.remove(id).is_some() {
            self.top_level_channels.delete(id);
            return
        }
        if self.listen_sockets.remove(id).is_some() {
            return
        }
        if self.servers.remove(id).is_some() {
            return
        }
    }

    fn trace_event(&self, id: i64, desc: *TraceEventDesc) {
        self.mu.lock();
        let child = self.find_entry(id);
        let child_tc, ok = child.(tracedChannel)
        if !ok {
            self.mu.unlock();
            return
        }
        child_tc.get_channel_trace().append(&TraceEvent { Desc: desc.Desc, Severity: desc.Severity, Timestamp: time.Now() })
        if desc.Parent != None {
            parent = self.find_entry(child.get_parent_id())
            var chanType RefChannelType
            match child.(type) {
                *channel =>
                    chanType = RefChannel
                *subChannel =>
                    chanType = RefSubChannel
            }
            if parent_tc, ok = parent.(tracedChannel); ok {
                parent_tc.get_channel_trace().append(TraceEvent {
                    Desc:      desc.Parent.Desc,
                    Severity:  desc.Parent.Severity,
                    Timestamp: time.Now(),
                    RefID:     id,
                    RefName:   child_tc.get_ref_name(),
                    RefType:   chanType,
                })
                child_tc.incr_trace_ref_count()
            }
        }
        self.mu.unlock();
    }

    pub fn get_top_channels(&self, id: i64, max_results: i64) -> ([]*ChannelMetric, bool) {
        if max_results <= 0 {
            max_results = EntryPerPage
        }
        self.mu.RLock()
        let l = i64(len(self.top_level_channels))
        let ids = make([]i64, 0, l)
        let cns = make([]*channel, 0, min(l, max_results))
    
        for k = range self.top_level_channels {
            ids = append(ids, k)
        }
        sort.Sort(int64Slice(ids));
        let idx = sort.Search(len(ids), fn(i int) bool { return ids[i] >= id })
        let count = i64(0)
        var end bool
        var t []*ChannelMetric
        for i, v in ids[idx:] {
            if count == max_results {
                break
            }
            if cn, ok = self.channels[v]; ok {
                cns = append(cns, cn)
                t = append(t, ChannelMetric {
                    NestedChans: copy_map(cn.nestedChans),
                    SubChans: copy_map(cn.subChans),
                })
                count += 1;
            }
            if i == len(ids[idx:]) - 1 {
                end = true
                break
            }
        }
        self.mu.RUnlock()
        if count == 0 {
            end = true
        }
    
        for i, cn = range cns {
            t[i].ChannelData = cn.c.ChannelzMetric()
            t[i].ID = cn.id
            t[i].RefName = cn.refName
            t[i].Trace = cn.trace.dumpData()
        }
        return t, end
    }
    
    pub fn get_servers(&self, id, max_results: i64) -> ([]*ServerMetric, bool) {
        if max_results <= 0 {
            max_results = EntryPerPage
        }
        self.mu.RLock()
        l = i64(len(self.servers))
        ids = make([]i64, 0, l)
        ss = make([]*server, 0, min(l, max_results))
        for k = range self.servers {
            ids = append(ids, k)
        }
        sort.Sort(int64Slice(ids))
        idx = sort.Search(len(ids), fn(i int) bool { return ids[i] >= id })
        count = i64(0)
        var end bool
        var s []*ServerMetric
        for i, v = range ids[idx:] {
            if count == max_results {
                break
            }
            if svr, ok = self.servers[v]; ok {
                ss = append(ss, svr)
                s = append(s, &ServerMetric{
                    ListenSockets: copy_map(svr.listen_sockets),
                })
                count++
            }
            if i == len(ids[idx:])-1 {
                end = true
                break
            }
        }
        self.mu.RUnlock()
        if count == 0 {
            end = true
        }
    
        for i, svr = range ss {
            s[i].ServerData = svr.s.ChannelzMetric()
            s[i].ID = svr.id
            s[i].RefName = svr.refName
        }
        return s, end
    }
    
    pub fn get_server_sockets(&self, id: i64, start_id: i64, max_results: i64) -> ([]*SocketMetric, bool) {
        if max_results <= 0 {
            max_results = EntryPerPage
        }
        var svr *server
        var ok bool
        self.mu.RLock()
        if svr, ok = self.servers.get(id); !ok {
            // server with id doesn't exist.
            self.mu.RUnlock()
            return None, true
        }
        svrskts = svr.sockets
        l = i64(len(svrskts))
        ids = make([]i64, 0, l)
        sks = make([]*normalSocket, 0, min(l, max_results))
        for k = range svrskts {
            ids = append(ids, k)
        }
        sort.Sort(int64Slice(ids))
        idx = sort.Search(len(ids), fn(i int) bool { return ids[i] >= start_id })
        count = i64(0)
        var end bool
        for i, v = range ids[idx:] {
            if count == max_results {
                break
            }
            if ns, ok = self.normal_sockets[v]; ok {
                sks = append(sks, ns)
                count++
            }
            if i == len(ids[idx:])-1 {
                end = true
                break
            }
        }
        self.mu.RUnlock()
        if count == 0 {
            end = true
        }
        s = make([]*SocketMetric, 0, len(sks))
        for _, ns = range sks {
            sm = &SocketMetric{}
            sm.SocketData = ns.s.ChannelzMetric()
            sm.ID = ns.id
            sm.RefName = ns.refName
            s = append(s, sm)
        }
        return s, end
    }
    
    pub fn get_channel(&self, id: i64) -> *ChannelMetric {
        cm = &ChannelMetric{}
        var cn *channel
        var ok bool
        self.mu.RLock()
        if cn, ok = self.channels.get(id); !ok {
            // channel with id doesn't exist.
            self.mu.RUnlock()
            return None
        }
        cm.NestedChans = copy_map(cn.nestedChans)
        cm.SubChans = copy_map(cn.subChans)
        // cn.c can be set to &dummyChannel{} when deleteSelfFromMap is called. Save a copy of cn.self when
        // holding the lock to prevent potential data race.
        chanCopy = cn.c
        self.mu.RUnlock()
        cm.ChannelData = chanCopy.ChannelzMetric()
        cm.ID = cn.id
        cm.RefName = cn.refName
        cm.Trace = cn.trace.dumpData()
        return cm
    }
    
    pub fn get_sub_channel(&self, id: i64) -> *SubChannelMetric {
        cm = &SubChannelMetric{}
        var sc *subChannel
        var ok bool
        self.mu.RLock()
        if sc, ok = self.sub_channels.get(id); !ok {
            // subchannel with id doesn't exist.
            self.mu.RUnlock()
            return None
        }
        cm.Sockets = copy_map(sc.sockets)
        // sc.c can be set to &dummyChannel{} when deleteSelfFromMap is called. Save a copy of sc.c when
        // holding the lock to prevent potential data race.
        chanCopy = sc.c
        self.mu.RUnlock()
        cm.ChannelData = chanCopy.ChannelzMetric()
        cm.ID = sc.id
        cm.RefName = sc.refName
        cm.Trace = sc.trace.dumpData()
        return cm
    }
    
    pub fn get_socket(&self, id: i64) -> *SocketMetric {
        sm = &SocketMetric{}
        self.mu.RLock()
        if ls, ok = self.listen_sockets.get(id); ok {
            self.mu.RUnlock()
            sm.SocketData = ls.s.ChannelzMetric()
            sm.ID = ls.id
            sm.RefName = ls.refName
            return sm
        }
        if ns, ok = self.normal_sockets.get(id); ok {
            self.mu.RUnlock()
            sm.SocketData = ns.s.ChannelzMetric()
            sm.ID = ns.id
            sm.RefName = ns.refName
            return sm
        }
        self.mu.RUnlock()
        return None
    }
    
    pub fn get_server(&self, id: i64) -> *ServerMetric {
        sm = &ServerMetric {}
        var svr *server
        var ok bool
        self.mu.RLock()
        if svr, ok = self.servers.get(id); !ok {
            self.mu.RUnlock()
            return None
        }
        sm.ListenSockets = copy_map(svr.listen_sockets)
        self.mu.RUnlock()
        sm.ID = svr.id
        sm.RefName = svr.refName
        sm.ServerData = svr.s.ChannelzMetric()
        return sm
    }
}

type int64Slice []i64

fn (s int64Slice) Len() int           { return len(s) }
fn (s int64Slice) Swap(i, j int)      { s[i], s[j] = s[j], s[i] }
fn (s int64Slice) Less(i, j int) bool { return s[i] < s[j] }

fn copy_map(m map[i64]string) map[i64]string {
    n = make(map[i64]string)
    for k, v = range m {
        n[k] = v
    }
    return n
}

fn min(a: i64, b: i64) -> i64 {
    if a < b {
        return a
    }
    return b
}

type idGenerator struct {
    id: i64
}

fn (i *idGenerator) reset() {
    atomic.StoreInt64(&i.id, 0)
}

fn (i *idGenerator) genID() i64 {
    return atomic.AddInt64(&i.id, 1)
}
