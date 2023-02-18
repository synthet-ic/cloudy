/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/extensions/common/matching/v3/extension_matcher.proto>
*/

use xds::types::matcher::matcher::Matcher;

use crate::config::core::extension::TypedExtensionConfig;

/**
Wrapper around an existing extension that provides an associated matcher. This allows decorating an existing extension with a matcher, which can be used to match against relevant protocol data.
*/
pub struct  ExtensionWithMatcher {
    // option (xds.annotations.v3.message_status).work_in_progress = true;

    /// The associated matcher.
    matcher: Matcher,
  
    /// The underlying extension config.
    // [(validate.rules).message = {required: true}];
    extension_config: TypedExtensionConfig
        
}
