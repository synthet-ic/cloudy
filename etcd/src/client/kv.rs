/*!
<https://github.com/etcd-io/etcd/blob/main/client/v3/kv.go>
*/

use std::{
    convert::From,
    task::Context
};

use grpc::rpc_util::CallOption;

use crate::{
    // api::server,
    client::{
        client::Client,
        transaction::Txn
    }
};

pub struct KV {
    remote: pb::KVClient,
    call_opts: Vec<CallOption>
}

impl KV {
    fn new(client: Option<Client>) -> Self {
        let call_opts = match client {
            Some(client) => client.call_opts,
            None => Vec::new()
        };
        Self { remote: RetryKVClient(c), call_opts }
    }

    fn fromKVClient(remote: pb::KVClient, client: Option<Client>) -> Self {
        let call_opts = match client {
            Some(client) => client.call_opts,
            None => Vec::new()
        };
        Self { remote, call_opts }
    }
    
    /**
    Puts a key-value pair into etcd.
    Note that key,value can be plain bytes array and string is an immutable representation of that bytes array.
    To get a string of bytes, do string([]byte{0x10, 0x20}).
    */
    fn put(&self, context: Context, key: String, value: String, opts: OpOption) -> Result<PutResponse> {
        match self.do(context, OpPut(key, value, opts)) {
            Ok(r) => r.put,
            Err(err) => toErr(context, err)
        }
    }

    /**
    Get retrieves keys.
    By default, Get will return the value for "key", if any.
    When passed WithRange(end), Get will return the keys in the range [key, end).
    When passed WithFromKey(), Get returns keys greater than or equal to key.
    When passed WithRev(rev) with rev > 0, Get retrieves keys at the given revision; if the required revision is compacted, the request will fail with ErrCompacted.
    When passed WithLimit(limit), the number of returned keys is bounded by limit.
    When passed WithSort(), the keys will be sorted.
    */
    fn get(&self, context: Context, key: String, opts: OpOption) -> Result<GetResponse> {
        match self.do(context, OpGet(key, opts)) {
            Ok(r) => r.get,
            Err(err) => toErr(context, err)
        }
    }

    /// Deletes a key, or optionally using WithRange(end), [key, end).
    fn delete(&self, context: Context, key: String, opts: OpOption) -> Result<DeleteResponse> {
        match self.do(context, OpDelete(key, opts)) {
            Ok(r) => r.del,
            Err(err) => toErr(context, err)
        }
    }

    /// Compacts etcd KV history before the given rev.
    fn compact(&self, context: Context, rev: i64, opts: CompactOption) -> Result<CompactResponse> {
        match self.remote.compact(context, OpCompact(rev, opts).to_request(), self.call_opts) {
            Ok(r) => Ok(CompactResponse(resp)),
            Err(err) => toErr(context, err)
        }
    }

    /**
    Applies a single Op on KV without a transaction.
    `do` is useful when creating arbitrary operations to be issued at a later time; the user can range over the operations, calling `do` to execute them. `get`/`put`/`delete`, on the other hand, are best suited for when the operation should be issued at the time of declaration.
    */
    fn do(&self, context: Context, op: Op) -> Result<OpResponse>;

    /// Creates a transaction.
    fn txn(&self, context: Context) -> Txn;
}
