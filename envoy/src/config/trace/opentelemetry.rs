/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/trace/v3/opentelemetry.proto>
*/

use crate::config::core::grpc_service::GRPCService;

/**
Configuration for the OpenTelemetry tracer.
[#extension: envoy.tracers.opentelemetry]
*/
pub struct  OpenTelemetryConfig {
    /**
    The upstream gRPC cluster that will receive OTLP traces.
    Note that the tracer drops traces if the server does not read data fast enough.
    */
    // [(validate.rules).message = {required: true}];
    grpc_service: GRPCService,

    /**
    The name for the service. This will be populated in the ResourceSpan Resource attributes.
    If it is not provided, it will default to "unknown_service:envoy".
    */
    service_name: String
}
