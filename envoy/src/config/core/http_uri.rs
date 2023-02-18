/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/http_uri.proto>
*/

use std::time::Duration;

/// Envoy external URI descriptor
pub struct HTTPURI {
    /**
    The HTTP server URI. It should be a full FQDN with protocol, host and path.

    Example:

    ```yaml
    uri: https://www.googleapis.com/oauth2/v1/certs
    ```

    [(validate.rules).string = {min_len: 1}];
    */
    uri: String,

    /**
    Specify how `uri` is to be fetched. Today, this requires an explicit cluster, but in the future we may support dynamic cluster creation or inline DNS resolution. See [issue](https://github.com/envoyproxy/envoy/issues/1606).
    */
    http_upstream_type: HTTPUpstreamType,

    /**
    Sets the maximum duration in milliseconds that a response can take to arrive upon request.

    [(validate.rules).duration = {
      required: true
      gte {}
    }];
    */
    timeout: Duration
}

pub enum HTTPUpstreamType {
    // option (validate.required) = true;

    /**
    A cluster is created in the Envoy `cluster_manager` config section. This field specifies the cluster name.

    Example:

    ```yaml
    cluster: jwks_cluster
    ```

    [(validate.rules).string = {min_len: 1}];
    */
    Cluster(String)
}
