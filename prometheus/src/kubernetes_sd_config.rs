/*!
<https://prometheus.io/docs/prometheus/latest/configuration/configuration/#kubernetes_sd_config>
*/

use serde::{Serialize, Deserialize};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct KubernetesSDConfig {
    
}
