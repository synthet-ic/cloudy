/*!
<https://github.com/cncf/xds/blob/main/xds/core/v3/extension.proto>
*/

use std::collections::HashMap;

/**
Additional parameters that can be used to select resource variants. These include any global context parameters, per-resource type client feature capabilities and per-resource type functional attributes. All per-resource type attributes will be `xds.resource.` prefixed and some of these are documented below:

`xds.resource.listening_address`: The value is "IP:port" (e.g. "10.1.1.3:8080") which is the listening address of a Listener. Used in a Listener resource query.
*/
pub struct ContextParams {
    params: HashMap<String, String>
}
