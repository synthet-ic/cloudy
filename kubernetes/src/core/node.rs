//! - Conceps <https://kubernetes.io/docs/concepts/architecture/nodes/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/>

use std::collections::HashMap;

use kfl::Decode;

use crate::{
    core::pod::TaintEffect,
    meta::{condition::Condition, metadata::Metadata},
    quantity::Quantity,
    time::Time,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#Node>
#[derive(Debug, Decode)]
pub struct Node {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>,
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#NodeSpec>
#[derive(Debug, Decode)]
pub struct Spec {
    /// Represents the pod IP range assigned to the node.
    pod_cidr: Option<String>,
    /// Represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the `pod_cidr` field. It may contain at most 1 value for each of Ipv4 and Ipv6.
    pod_cidrs: Vec<String>,
    /// ID of the node assigned by the cloud provider in the format: `<provider-name>://<provider-specific-node-id>`.
    provider_id: Option<String>,
    /// If specified, the node's taints.
    taints: Vec<Taint>,
    /// Controls node schedulability of new pods. By default, node is schedulable.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/architecture/nodes/#manual-node-administration>
    unschedulable: Option<bool>
}

/// The node this Taint is attached to has the 'effect' on any pod that does not tolerate the Taint.
#[derive(Debug, Decode)]
pub struct Taint {
    /// Effect of the taint on pods that do not tolerate the taint. Valid effects are `NoSchedule`, `PreferNoSchedule` and `NoExecute`.
    effect: TaintEffect,
    /// The taint key to be applied to a node.
    key: String,
    /// Represents the time at which the taint was added. It is only written for `NoExecute` taints.
    time_added: Option<Time>,
    /// The taint value corresponding to the taint key.
    value: Option<String>
}

/// NodeStatus is information about the current status of a node.
/// 
/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/node-v1/#NodeStatus>
#[derive(Debug, Decode)]
pub struct Status {
    /// List of addresses reachable to the node. Queried from cloud provider, if available.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/architecture/nodes/#addresses>
    ///
    /// Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See <https://github.com/kubernetes/kubernetes/pull/79391> for an example.
    addresses: Vec<NodeAddress>,
    /// Represents the resources of a node that are available for scheduling. Defaults to [`capacity`][Self::capacity].
    allocatable: HashMap<String, Quantity>,
    /// Represents the total resources of a node.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#capacity>
    capacity: HashMap<String, Quantity>,
    /// An array of current observed node conditions.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/architecture/nodes/#condition>
    conditions: Vec<Condition<NodeConditionType>>,
    /// Status of the config assigned to the node via the dynamic Kubelet config feature.
    config: Option<NodeConfigStatus>,
    /// Endpoints of daemons running on the Node.
    daemon_endpoints: DaemonEndpoints,
    /// List of container images on this node.
    images: Vec<ContainerImage>,
    /// Set of ids/uuids to uniquely identify the node.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/architecture/nodes/#info>
    node_info: Option<NodeSystemInfo>
}

#[derive(Debug, Decode)]
pub struct NodeAddress {
    address: String,
    r#type: NodeAddressType
}

#[derive(Debug, Decode)]
pub enum NodeAddressType {
    Hostname,
    ExternalIP,
    InternalIP
}

/// <https://kubernetes.io/docs/concepts/architecture/nodes/#condition>
#[derive(Debug, DecodeScalar)]
pub enum NodeConditionType {
    /// `True` if the node is healthy and ready to accept pods, `False` if the node is not healthy and is not accepting pods, and `Unknown` if the node controller has not heard from the node in the last node-monitor-grace-period (default is 40 seconds).
    Ready,
    /// `True` if pressure exists on the disc size—that is, if the disc capacity is low; otherwise `False`.
    DiscPressure,
    /// `True` if pressure exists on the node memory—that is, if the node memory is low; otherwise `False`.
    MemoryPressure,
    /// `True` if pressure exists on the processes—that is, if there are too many processes on the node; otherwise `False`.
    PIDPressure,
    /// `True` if the network for the node is not correctly configured, otherwise `False`.
    NetworkUnavailable
}

#[derive(Debug, Decode)]
pub struct NodeConfigStatus {
    error: Option<String>,
}

/// DaemonEndpoints lists ports opened by daemons running on the Node.
#[derive(Debug, Decode)]
pub struct DaemonEndpoints {
    /// Endpoint on which Kubelet is listening.
    kubelet_endpoint: Option<DaemonEndpoint>
}

/// DaemonEndpoint contains information about a single Daemon endpoint.
#[derive(Debug, Decode)]
pub struct DaemonEndpoint {
    /// Port number of the given endpoint.
    port: u16
}

#[derive(Debug, Decode)]
pub struct ContainerImage {
    /// Names by which this image is known. e.g. `[ "kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7" ]`
    names: Vec<String>,
    /// The size of the image in bytes.
    size_bytes: Option<u64>
}

/// NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
#[derive(Debug, Decode)]
pub struct NodeSystemInfo {
    /// The Architecture reported by the node.
    architecture: String,
    /// Boot ID reported by the node.
    boot_id: String,
    /// ContainerRuntime Version reported by the node through runtime remote API (e.g. `containerd://1.4.2`).
    container_runtime_version: String,
    /// Kernel Version reported by the node from **uname -r** (e.g. `5.15.49-linuxkit`).
    kernel_version: String,
    /// KubeProxy Version reported by the node.
    kube_proxy_version: String,
    /// Kubelet Version reported by the node.
    kubelet_version: String,
    /// MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: <http://man7.org/linux/man-pages/man5/machine-id.5.html>
    machine_id: String,
    /// The Operating System reported by the node
    operating_system: String,
    /// OS Image reported by the node from `/etc/os-release` (e.g. `Alpine Linux edge`).
    os_image: String,
    /// SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts <https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid>
    system_uuid: String
}
