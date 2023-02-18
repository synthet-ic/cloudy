/*!
<https://github.com/grpc/grpc-go/tree/master/resolver/manual>
*/

use crate::resolver::{BuildOptions, ClientConn, ResolveNowOptions, State, Target};

/// Resolver is also a resolver builder.
/// It's build() function always returns itself.
pub struct Resolver {
    scheme: String,

    /// Fields actually belong to the resolver.
    cc: ClientConn,
    bootstrap_state: Option<State>
}

impl Resolver {
    /**
    build_callback is called when the Build method is called.  Must not be nil. Must not be changed after the resolver may be built.
    */
    fn build_callback(Target, ClientConn, BuildOptions) {}
    /**
    resolve_now_callback is called when the ResolveNow method is called on the
    resolver. Must not be nil. Must not be changed after the resolver may be built.
    */
    fn resolve_now_callback(ResolveNowOptions) {}
    /**
    close_callback is called when the Close method is called. Must not be nil.  Must not be changed after the resolver may be built.
    */
    fn close_callback() {}
}
