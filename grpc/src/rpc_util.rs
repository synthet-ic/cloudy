/*!
<https://github.com/grpc/grpc-go/blob/master/rpc_util.go>
*/

use crate::credentials::PerRPCCredentials;

/// `CallInfo` contains all related configuration and information about an RPC.
pub struct CallInfo {
    compressor_type: String,
    fail_fast: bool,
    max_receive_message_size: Option<i32>,
    max_send_message_size: Option<i32>,
    credentials: PerRPCCredentials,
    content_subtype: String,
    codec: BaseCodec,
    max_retry_rpc_buffer_size: i32,
}

/**
CallOption configures a Call before it starts or extracts information from
a Call after it completes.
*/
pub trait CallOption {
    /**
    before is called before the call is sent to any server. If before returns a non-nil error, the RPC fails with that error.
    */
    fn before(&self, call_info: Option<CallInfo>) -> Result<()>;

    /**
    after is called after the call has completed. after cannot return an error, so any failures should be reported via output parameters.
    */
    fn after(&self, call_info: Option<CallInfo>, *csAttempt);
}
