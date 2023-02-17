//! Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/event-v1/>

use kfl::Decode;

use crate::{
    core::reference::Reference,
    meta::metadata::Metadata,
    time::{MicroTime, Time}
};

#[derive(Debug, Decode)]
pub struct Event {
    metadata: Metadata,
    event_time: MicroTime,
    action: Option<String>,
    deprecated_count: Option<i32>,
    deprecated_first_timestamp: Option<Time>,
    deprecated_last_timestamp: Option<Time>,
    deprecated_source: Option<EventSource>,
    note: Option<String>,
    reason: Option<String>,
    regarding: Option<Reference>,
    related: Option<Reference>,
    reporting_controller: Option<String>,
    reporting_instance: Option<String>,
    series: Option<EventSeries>,
    r#type: Option<EventType>
}

#[derive(Debug, Decode)]
pub struct EventSource {

}

#[derive(Debug, Decode)]
pub struct EventSeries {

}

#[derive(Debug, Decode)]
pub enum EventType {
    Normal,
    Warning
}
