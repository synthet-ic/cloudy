/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/trace/v3/http_tracer.proto>
*/

type Any = String;

/**
The tracing configuration specifies settings for an HTTP tracer provider used by Envoy.

Envoy may support other tracers in the future, but right now the HTTP tracer is the only one supported.

> attention: Use of this message type has been deprecated in favor of direct use of [`Tracing.HTTP`][crate::config::trace::Tracing.HTTP].
*/
pub struct Tracing {
    /// Provides configuration for the HTTP tracer.
    http: HTTP
}

/**
Configuration for an HTTP tracer provider used by Envoy.

The configuration is defined by the
[`HTTPConnectionManager.Tracing`][crate::extensions::filters.network.http_connection_manager::HTTPConnectionManager.Tracing]
[`provider`][crate::extensions::filters.network.http_connection_manager::HTTPConnectionManager.Tracing.provider]
field.
*/
pub struct HTTP {
    /**
    The name of the HTTP trace driver to instantiate. The name must match a supported HTTP trace driver.
    See the :ref:`extensions listed in typed_config below <extension_category_envoy.tracers>` for the default list of the HTTP trace driver.

    [(validate.rules).string = {min_len: 1}];
    */
    name: String,

    /**
    Trace driver specific configuration which must be set according to the driver being instantiated.
    [#extension-category: envoy.tracers]
    */
    config_type: ConfigType
}

pub enum ConfigType {
    TypedConfig(Any)
}
