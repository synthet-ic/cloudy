/*!
<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/logs/v1/logs_service.proto>
*/

use crate::logs::ResourceLogs;

/// Service that can be used to push logs between one Application instrumented with OpenTelemetry and an collector, or between an collector and a central collector (in this case logs are sent/received to/from multiple Applications).
pub trait LogsService {
    /// For performance reasons, it is recommended to keep this RPC alive for the entire life of the application.
    fn export(r: ExportLogsServiceRequest) -> ExportLogsServiceResponse;
}

pub struct ExportLogsServiceRequest {
    /**
    An array of `ResourceLogs`.
    For data coming from a single resource this array will typically contain one element. Intermediary nodes (such as OpenTelemetry Collector) that receive data from multiple origins typically batch the data before forwarding further and in that case this array will contain multiple elements.
    */
    resource_logs: Vec<ResourceLogs>,
}

pub struct ExportLogsServiceResponse {
    /**
    The details of a partially successful export request.

    If the request is only partially accepted (i.e. when the server accepts only parts of the data and rejects the rest) the server MUST initialise the `partial_success` field and MUST set the `rejected_<signal>` with the number of items it rejected.

    Servers MAY also make use of the `partial_success` field to convey warnings/suggestions to senders even when the request was fully accepted.
    In such cases, the `rejected_<signal>` MUST have a value of `0` and the `error_message` MUST be non-empty.

    A `partial_success` message with an empty value (`rejected_<signal>` = `0` and `error_message` = `None`) is equivalent to it not being set/present. Senders SHOULD interpret it the same way as in the full success case.
    */
    partial_success: ExportLogsPartialSuccess,
}

pub struct ExportLogsPartialSuccess {
    /**
    The number of rejected log records.

    A `rejected_<signal>` field holding a `0` value indicates that the request was fully accepted.
    */
    rejected_log_records: i64,

    /**
    A developer-facing human-readable message in English. It should be used either to explain why the server rejected parts of the data during a partial success or to convey warnings/suggestions during a full success. The message should offer guidance on how users can address such issues.

    `error_message` is an optional field. An `error_message` with an empty value is equivalent to it not being set.
    */
    error_message: Option<String>
}
