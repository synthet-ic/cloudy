/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/cluster/v3/filter.proto>
*/

type Any = String;

pub struct Filter {
    /// The name of the filter configuration.
    // [!is_empty()]
    name: String,

    /**
    Filter specific configuration which depends on the filter being instantiated. See the supported filters for further documentation.
    Note that Envoy's :ref:`downstream network filters <config_network_filters>` are not valid upstream filters.
    */
    typed_config: Any,
}
