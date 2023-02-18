/*!
<https://github.com/etcd-io/etcd/blob/main/client/v3/txn.go>
*/

use std::{
    error::Error,
    sync::Mutex,
    task::Context
};

use super::kv::KV;

/*
Txn is the interface that wraps mini-transactions.

```
Txn(context.TODO()).r#if(
    Compare(Value(k1), ">", v1),
    Compare(Version(k1), "=", 2)
).r#then(
    OpPut(k2, v2), OpPut(k3, v3)
).r#else(
    OpPut(k4, v4), OpPut(k5, v5)
).commit()
```
*/
pub struct Txn {
    kv: Option<KV>,
    context: Context,
    mu: Mutex,
    cif: bool,
    cthen: bool,
    celse: bool,

    is_write: bool,

    cmps: Vec<pb::Compare>,

    sus: Vec<pb::RequestOp>,
    fas: Vec<pb::RequestOp>,

    call_opts: Vec<grpc::CallOption>
}

impl Txn {
    /**
    If takes a list of comparison. If all comparisons passed in succeed,
    the operations passed into `then()` will be executed. Or the operations
    passed into `else()` will be executed.
    */
    fn r#if(&self, cs: Cmp) -> Txn {

    }

    /**
    Then takes a list of operations. The Ops list will be executed, if the comparisons passed in `if()` succeed.
    */
    fn then(&self, ops: Op) -> Txn {

    }

    /**
    Else takes a list of operations. The Ops list will be executed, if the
    comparisons passed in If() fail.
    */
    fn r#else(&self, ops: Op) -> Txn {

    }

    /// Commit tries to commit the transaction.
    fn commit(&self) -> Result<TxnResponse, dyn Error> {

    }
}
