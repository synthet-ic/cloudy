/*!
- Homepage <https://w3c.github.io/trace-context/>
- Repository <https://github.com/w3c/trace-context>
*/

use std::collections::VecDeque;

pub struct TraceContext {
    trace_id: TraceId,
    span_id: SpanId,
    trace_flags: TraceFlags,
    is_remote: bool,
    trace_state: TraceState,
}

pub struct TraceId(u128);

pub struct SpanId(u64);

pub struct TraceFlags(u8);

pub struct TraceState(Option<VecDeque<(String, String)>>);
