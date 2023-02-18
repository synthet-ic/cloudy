/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/overload/v3/overload.proto>
*/

type Any = String;
type Struct = String;
type ConfigType = String;

use std::time::Duration;

use crate::types::percent::Percent;

/**
The Overload Manager provides an extensible framework to protect Envoy instances from overload of various resources (memory, cpu, file descriptors, etc).
It monitors a configurable set of resources and notifies registered listeners when triggers related to those resources fire.
*/
pub struct ResourceMonitor {
    /**
    The name of the resource monitor to instantiate. Must match a registered resource monitor type.
    See the :ref:`extensions listed in typed_config below <extension_category_envoy.resource_monitors>` for the default list of available resource monitor.

    [(validate.rules).string = {min_len: 1}];
    */
    name: String,

    /**
    Configuration for the resource monitor being instantiated.
    [#extension-category: envoy.resource_monitors]
    */
    config_type: ConfigType
}

pub struct ThresholdTrigger {
    /**
    If the resource pressure is greater than or equal to this value, the trigger will enter saturation.

    [(validate.rules).f64 = {lte: 1.0 gte: 0.0}]
    */
    value: f64
}

pub struct ScaledTrigger {
    /**
    If the resource pressure is greater than this value, the trigger will be in the :ref:`scaling <arch_overview_overload_manager-triggers-state>` state with value `(pressure - scaling_threshold) / (saturation_threshold - scaling_threshold)`.

    [(validate.rules).f64 = {lte: 1.0 gte: 0.0}];
    */
    scaling_threshold: f64,

    /// If the resource pressure is greater than this value, the trigger will enter saturation.
    // [(validate.rules).f64 = {lte: 1.0 gte: 0.0}];
    saturation_threshold: f64
}

pub struct Trigger {
    /// The name of the resource this is a trigger for.
    // [!is_empty()]
    name: String,

    trigger_oneof: TriggerOneof
}


pub enum TriggerOneof {
    // option (validate.required) = true;

    Threshold(ThresholdTrigger),

    Scaled(ScaledTrigger),
}

/**
Typed configuration for the "envoy.overload_actions.reduce_timeouts" action. See
:ref:`the docs <config_overload_manager_reducing_timeouts>` for an example of how to configure the action with different timeouts and minimum values.
*/
pub struct ScaleTimersOverloadActionConfig {
    /// A set of timer scaling rules to be applied.
    /// [!timer_scale_factors.is_empty()]
    timer_scale_factors: Vec<ScaleTimer>
}

pub enum TimerType {
    /// Unsupported value; users must explicitly specify the timer they want scaled.
    Unspecified,

    /**
    Adjusts the idle timer for downstream HTTP connections that takes effect when there are no active streams.
    This affects the value of [`HTTPProtocolOptions::idle_timeout`][crate::config::core::protocol::HTTPProtocolOptions::idle_timeout].
    */
    HTTPDownstreamConnectionIdle,

    /**
    Adjusts the idle timer for HTTP streams initiated by downstream clients.
    This affects the value of [`RouteAction::idle_timeout`][crate::config::route::route_components::RouteAction::idle_timeout] and
    [`HTTPConnectionManager::stream_idle_timeout`][crate::extensions::filters::network::http_connection_manager::HTTPConnectionManager::stream_idle_timeout].
    */
    HTTPDownstreamStreamIdle,

    /**
    Adjusts the timer for how long downstream clients have to finish transport-level negotiations before the connection is closed.
    This affects the value of
    [`FilterChain::transport_socket_connect_timeout`][crate::config::listener::listener_components::FilterChain::transport_socket_connect_timeout].
    */
    TransportSocketConnect,
}

pub struct ScaleTimer {
    /// The type of timer this minimum applies to.
    // [(validate.rules).enum = {defined_only: true not_in: 0}];
    timer: TimerType,

    overload_adjust: OverloadAdjust
}

pub enum OverloadAdjust {
    // option (validate.required) = true;

    /// Sets the minimum duration as an absolute value.
    MinTimeout(Duration),

    /// Sets the minimum duration as a percentage of the maximum value.
    MinScale(Percent),
}

pub struct OverloadAction {
    /**
    The name of the overload action. This is just a well-known string that listeners can use for registering callbacks. Custom overload actions should be named using reverse
    DNS to ensure uniqueness.

    [(validate.rules).string = {min_len: 1}];
    */
    name: String,

    /**
    A set of triggers for this action. The state of the action is the maximum state of all triggers, which can be scaling between 0 and 1 or saturated. Listeners are notified when the overload action changes state.

    [(validate.rules).repeated = {min_items: 1}];
    */
    triggers: Vec<Trigger>,

    /// Configuration for the action being instantiated.
    typed_config: Any,
}

/**
Configuration for which accounts the WatermarkBuffer Factories should track.
*/
pub struct BufferFactoryConfig {
    /**
    The minimum power of two at which Envoy starts tracking an account.

    Envoy has 8 power of two buckets starting with the provided exponent below.
    Concretely the 1st bucket contains accounts for streams that use
    [2^minimum_account_to_track_power_of_two,
    2^(minimum_account_to_track_power_of_two + 1)) bytes.
    With the 8th bucket tracking accounts
    >= 128 * 2^minimum_account_to_track_power_of_two.

    The maximum value is 56, since we're using uint64_t for bytes counting,
    and that's the last value that would use the 8 buckets. In practice,
    we don't expect the proxy to be holding 2^56 bytes.

    If omitted, Envoy should not do any tracking.

    [(validate.rules).u32 = {lte: 56 gte: 10}];
    */
    minimum_account_to_track_power_of_two: u32
}

pub struct OverloadManager {
    /// The interval for refreshing resource usage.
    refresh_interval: Duration,

    /// The set of resources to monitor.
    /// [!resource_monitors.is_empty()]
    resource_monitors: Vec<ResourceMonitor>,

    /// The set of overload actions.
    actions: Vec<OverloadAction>,

    /// Configuration for buffer factory.
    buffer_factory_config: BufferFactoryConfig,
}
