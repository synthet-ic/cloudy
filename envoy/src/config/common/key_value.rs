/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/common/key_value/v3/config.proto>
*/

use crate::config::core::extension::TypedExtensionConfig;

/// This shared configuration for Envoy key value stores.
pub struct  KeyValueStoreConfig {
    // option (xds.annotations::message_status).work_in_progress = true;

    // [#extension-category: envoy.common.key_value]
    // [(validate.rules).message = {required: true}];
    config: TypedExtensionConfig
}
