/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/event_service_config.proto>
*/

use super::grpc_service::GRPCService;

/// Configuration of the event reporting service endpoint.
pub enum EventServiceConfig {
    // option (validate.required) = true;

    /// Specifies the gRPC service that hosts the event reporting service.
    GRPCService(GRPCService)
}
