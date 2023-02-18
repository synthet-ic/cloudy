/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/extension.proto>
*/

type Any = String;

/**
Message type for extension configuration.
[#next-major-version: revisit all existing typed_config that doesn't use this wrapper.].
*/
pub struct TypedExtensionConfig {
    /// The name of an extension. This is not used to select the extension, instead it serves the role of an opaque identifier.
    // [!is_empty()];
    name: String,

    /**
    The typed config for the extension. The type URL will be used to identify the extension. In the case that the type URL is `xds.type::TypedStruct` (or, for historical reasons, `udpa.type.v1.TypedStruct`), the inner type URL of `TypedStruct` will be utilized. See the :ref:`extension configuration overview <config_overview_extension_configuration>` for further details.
    */
    // [(validate.rules).any = {required: true}];
    typed_config: Any
}
