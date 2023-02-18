/*!
<https://github.com/grpc/grpc-go/blob/master/internal/credentials/credentials.go>
*/

use std::task::Context;

/// Struct to be used as the key to store RequestInfo in a context.
struct requestInfoKey;

/// Creates a context with ri.
pub fn new_request_info_context(context: Contextt, ri: interface{}) -> Context {
	  return context::WithValue(context, requestInfoKey{}, ri)
}

/// Extracts the RequestInfo from ctx.
pub fn request_info_from_context(context: Context) -> interface{} {
	  return context.Value(requestInfoKey{})
}

/// Struct used as the key to store ClientHandshakeInfo in a context.
struct clientHandshakeInfoKey;

/// Extracts the ClientHandshakeInfo from ctx.
pub fn client_handshake_info_from_context(context: Context) -> interface{} {
	  return context.Value(clientHandshakeInfoKey{})
}

/// Creates a context with chi.
pub fn new_client_handshake_info_context(context: Context, chi: interface{}) -> Context {
	  return context::WithValue(context, clientHandshakeInfoKey{}, chi)
}
