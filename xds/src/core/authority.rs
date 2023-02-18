/*!
<https://github.com/cncf/xds/blob/main/xds/core/v3/authority.proto>
*/

/// xDS authority information.
pub struct Authority {
    // [!is_empty()]
    name: String,

    // .. space reserved for additional authority addressing information, e.g. for resource signing, items such as CA trust chain, cert pinning may be added.
}
