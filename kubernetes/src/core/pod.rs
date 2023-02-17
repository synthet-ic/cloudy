//! - Concepts <https://kubernetes.io/docs/concepts/workloads/pods/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/>

use std::{
    collections::HashMap,
    path::PathBuf
};

use kfl::{Decode, DecodeScalar};

use crate::{
    core::{
        field_selector::FieldSelector,
        local_reference::LocalReference,
        resource_field_selector::ResourceFieldSelector,
        volume::Volume
    },
    meta::{
        condition::Condition,
        label_selector::LabelSelector,
        metadata::Metadata
    },
    node_selector::{NodeSelector, NodeSelectorTerm},
    protocol::Protocol,
    quantity::Quantity,
    time::Time,
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#Pod>
#[derive(Debug, Decode)]
pub struct Pod {
    metadata: Metadata,
    spec: PodSpec,
    status: Option<PodStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#PodSpec>
#[derive(Debug, Decode)]
pub struct PodSpec {
    // Containers
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#containers>

    /// List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
    containers: Vec<Container>,
    /// List of initialisation containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its [`restart_policy`][Self::restart_policy]. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/init-containers/>
    init_containers: Vec<Container>,
    /// List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.
    ephemeral_containers: Vec<EphemeralContainer>,
    /// `image_pull_secrets` is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod>
    image_pull_secrets: Vec<LocalReference>,
    /// enable_service_links indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.
    enable_service_links: Option<bool>,
    /// Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set.
    os: Option<PodOS>,

    // Volumes

    /// List of volumes that can be mounted by containers belonging to the pod.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes/>
    volumes: Vec<Volume>,

    // Scheduling
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#scheduling>
    /// `node_selector` is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/>
    node_selector: HashMap<String, String>,
    /// `node_name` is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.
    node_name: Option<String>,
    /// If specified, the pod's scheduling constraints.
    affinity: Option<Affinity>,
    /// If specified, the pod's tolerations.
    tolerations: Vec<Toleration>,
    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
    scheduler_name: Option<String>,
    /// `runtime_class_name` refers to a [`RuntimeClass`][RuntimeClass] object in the node.k8s.io group, which should be used to run this pod. If no [`RuntimeClass`][RuntimeClass] resource matches the named class, the pod will not be run. If unset or empty, the 'legacy' [`RuntimeClass`][RuntimeClass] will be used, which is an implicit class with an empty definition that uses the default runtime handler.
    ///
    /// More info: <https://github.com/kubernetes/enhancements/tree/master/keps/sig-node/585-runtime-class>
    ///
    /// [RuntimeClass]: crate::node::runtime_class::RuntimeClass
    runtime_class_name: Option<String>,
    /// If specified, indicates the pod's priority. `"system-node-critical"` and `"system-cluster-critical"` are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    priority_class_name: Option<String>,
    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from [`priority_class_name`][Self::priority_class_name]. The higher the value, the higher the priority.
    priority: Option<i32>,
    /// `preemption_policy` is the Policy for preempting pods with lower priority. One of `Never`, `PreemptLowerPriority`. Defaults to `PreemptLowerPriority` if unset.
    preemption_policy: Option<PreemptionPolicy>,
    /// `topology_spread_constraints` describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All `topology_spread_constraints` are ANDed.
    topology_spread_constraints: Vec<TopologySpreadConstraint>,
    /// `overhead` represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be auto-populated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero.
    ///
    /// More info: <https://github.com/kubernetes/enhancements/blob/master/keps/sig-node/688-pod-overhead/README.md>
    overhead: Option<HashMap<String, Quantity>>,

    // Lifecycle
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#lifecycle>

    /// Restart policy for all containers within the pod. One of `Always`, `OnFailure`, `Never`. Default to `Always`.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy>
    #[kfl(default = RestartPolicy::Always)]
    restart_policy: RestartPolicy,
    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is empty, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    #[kfl(default = "30")]
    termination_grace_period_seconds: Option<u64>,
    /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
    active_deadline_seconds: Option<u64>,
    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to `True`.
    ///
    /// More info: <https://github.com/kubernetes/enhancements/tree/master/keps/sig-network/580-pod-readiness-gates>
    readiness_gates: Vec<PodReadinessGate>,

    // Hostname and Name Resolution
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#hostname-and-name-resolution>

    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
    hostname: Option<String>,
    /// If `true` the pod's hostname will be configured as the pod's Fqdn, rather than the leaf name (the default). In Linux containers, this means setting the Fqdn in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters to Fqdn. If a pod does not have Fqdn, this has no effect. Default to `false`.
    #[kfl(default)]
    set_hostname_as_fqdn: bool,
    /// If specified, the fully qualified Pod hostname will be `"<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>"`. If not specified, the pod will not have a domainname at all.
    subdomain: Option<String>,
    /// `host_aliases` is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified. This is only valid for non-[`host_network`][Self::host_network] pods.
    host_aliases: Vec<HostAlias>,
    /// Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on [`dns_policy`][Self::dns_policy].
    dns_config: Option<PodDNSConfig>,
    /// Set DNS policy for the pod. Defaults to `ClusterFirst`. Valid values are `ClusterFirstWithHostNet`, `ClusterFirst`, `Default` or `None`. DNS parameters given in [`dns_config`][Self::dns_config] will be merged with the policy selected with `dns_policy`. To have DNS options set along with [`host_network`][Self::host_network], you have to specify DNS policy explicitly to `ClusterFirstWithHostNet`.
    #[kfl(default = "PodDNSPolicy::ClusterFirst")]
    dns_policy: Option<PodDNSPolicy>,

    // Hosts Namespaces
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#hosts-namespaces>

    /// Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to `false`.
    #[kfl(default)]
    host_network: bool,
    /// Use the host's pid namespace. Optional: Default to `false`.
    #[kfl(default)]
    host_pid: bool,
    /// Use the host's ipc namespace. Optional: Default to `false`.
    #[kfl(default)]
    host_ipc: bool,
    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. [`host_pid`][Self::host_pid] and `share_process_namespace` cannot both be set. Optional: Default to `false`.
    #[kfl(default)]
    share_process_namespace: bool,

    // Service Account
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#service-account>

    /// Name of the ServiceAccount to use to run this pod.
    ///
    /// More info: [Configure Service Accounts for Pods](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/)
    service_account_name: Option<String>,
    /// `automount_service_account_token` indicates whether a service account token should be automatically mounted.
    automount_service_account_token: Option<bool>,

    // Security Context
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#security-context>

    /// `security_context` holds pod-level security attributes and common container settings. Optional: Defaults to empty. See type description for default values of each field.
    security_context: Option<PodSecurityContext>,

    // Alpha Level
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#alpha-level>

    /// Use the host's user namespace. Optional: Default to `true`. If set to `true` or not present, the pod will be run in the host user namespace, useful for when the pod needs a feature only available to the host user namespace, such as loading a kernel module with CAP_SYS_MODULE. When set to `false`, a new userns is created for the pod. Setting `false` is useful for mitigating container breakout vulnerabilities even allowing users to run their containers as root without actually having root privileges on the host. This field is alpha-level and is only honored by servers that enable the UserNamespacesSupport feature.
    #[kfl(default = true)]
    host_users: bool
}

/**
- Concepts <https://kubernetes.io/docs/concepts/containers/>
- Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#Container>
*/
#[derive(Debug, Decode)]
pub struct Container {
    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
    name: String,
    // Image
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#image>

    /// Container image name.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/containers/images/>
    ///
    /// This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    image: Option<String>,
    /// Image pull policy. One of `Always`, `Never`, `IfNotPresent`. Defaults to `Always` if `:latest` tag is specified, or `IfNotPresent` otherwise. Cannot be updated.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/containers/images/#updating-images>
    #[kfl(default = ImagePullPolicy::Always)]
    image_pull_policy: ImagePullPolicy,

    // Entrypoint
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#entrypoint>

    /// Entrypoint array. Not executed within a shell. The container image's `ENTRYPOINT` is used if this is not provided. Variable references `$(VAR_NAME)` are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double `$$` are reduced to a single `$`, which allows for escaping the `$(VAR_NAME)` syntax: i.e. `"$$(VAR_NAME)"` will produce the string literal `"$(VAR_NAME)"`. Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated.
    ///
    /// More info: [Define a Command and Arguments for a Container](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell)
    command: Vec<String>,
    /**
    Arguments to the entrypoint. The container image's `CMD` is used if this is not provided. Variable references `$(VAR_NAME)` are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double `$$` are reduced to a single `$`, which allows for escaping the `$(VAR_NAME)` syntax: i.e. `"$$(VAR_NAME)"` will produce the string literal `"$(VAR_NAME)"`. Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated.
    
    More info: [Define a Command and Arguments for a Container](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell)
    */
    args: Vec<String>,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    working_dir: Option<PathBuf>,

    // Ports
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#ports>

    /// List of ports to expose from the container. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default `"0.0.0.0"` address inside a container will be accessible from the network. Modifying this array with strategic merge patch may corrupt the data. For more information See <https://github.com/kubernetes/kubernetes/issues/108255>. Cannot be updated.
    ports: Vec<ContainerPort>,

    /*
    Environment Variables
    <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#environment-variables>
    */

    /// List of environment variables to set in the container. Cannot be updated.
    env: Vec<EnvVar>,
    /// List of sources to populate environment variables in the container. The keys defined within a source must be a `C_IDENTIFIER`. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    env_from: Vec<EnvFromSource>,

    // Volumes
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#volumes-1>

    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[kfl(children)]
    volume_mounts: Vec<VolumeMount>,
    /// `volume_devices` is the list of block devices to be used by the container.
    #[kfl(children)]
    volume_devices: Vec<VolumeDevice>,

    // Resources
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#resources>
    /**
    Compute Resources required by this container. Cannot be updated.
    
    More info: [Resource Management for Pods and Containers](https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/)
    */
    resources: Option<ResourceRequirements>,

    // Lifecycle
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#lifecycle-1>

    /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
    lifecycle: Option<Lifecycle>,
    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to `"/dev/termination-log"`. Cannot be updated.
    #[kfl(default = "/dev/termination-log")]
    termination_message_path: PathBuf,
    /// Indicate how the termination message should be populated. File will use the contents of [`termination_message_path`][Self::termination_message_path] to populate the container status message on both success and failure. `FallbackToLogsOnError` will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to `File`. Cannot be updated.
    #[kfl(default = TerminationMessagePolicy::File)]
    termination_message_policy: TerminationMessagePolicy,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes>
    liveness_probe: Option<Probe>,
    /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes>
    readiness_probe: Option<Probe>,
    /// StartupProbe indicates that the Pod has successfully initialised. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the [`liveness_probe`][Self::liveness_probe] failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes>
    startup_probe: Option<Probe>,

    // Security Context
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#security-context-1>
    /// `security_context` defines the security options the container should be run with. If set, the fields of [`SecurityContext`][SecurityContext] override the equivalent fields of [`PodSecurityContext`][PodSecurityContext].
    ///
    /// More info: [Configure a Security Context for a Pod or Container](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)
    security_context: Option<SecurityContext>,
    
    // Debugging
    // <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#debugging>
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is `false`.
    #[kfl(default)]
    stdin: bool,
    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is `false`.
    #[kfl(default)]
    stdin_once: bool,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is `false`.
    #[kfl(default)]
    tty: bool,
}

#[derive(Debug, DecodeScalar, Default)]
pub enum ImagePullPolicy {
    #[default]
    Always,
    Never,
    IfNotPresent
}

/// ContainerPort represents a network port in a single container.
#[derive(Debug, Decode)]
pub struct ContainerPort {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
    container_port: u16,
    /// What host IP to bind the external port to.
    #[kfl(property, default)]
    host_ip: Option<String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    #[kfl(property, default)]
    host_port: Option<u16>,
    /// If specified, this must be an `IANA_SVC_NAME` and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    #[kfl(property, default)]
    name: Option<String>,
    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to `TCP`.
    #[kfl(property, default = Protocol::Tcp)]
    protocol: Protocol
}

#[derive(Debug, Decode)]
pub struct EnvVar {
    name: String,
    value: Option<String>,
    value_from: Option<EnvVarSource>,
}

#[derive(Debug, Decode)]
pub struct EnvVarSource {
    config_map_key_ref: Option<ConfigMapKeySelector>,
    field_ref: Option<FieldSelector>,
    resource_field_ref: Option<ResourceFieldSelector>,
    secret_key_ref: Option<SecretKeySelector>
}

#[derive(Debug, Decode)]
pub struct ConfigMapKeySelector {
    key: String,
    name: Option<String>,
    optional: Option<bool>
}

#[derive(Debug, Decode)]
pub struct SecretKeySelector {
    key: String,
    name: Option<String>,
    optional: Option<bool>
}

#[derive(Debug, Decode)]
pub struct EnvFromSource {
    config_map_ref: Option<ConfigMapEnvSource>,
    prefix: Option<String>,
    secret_ref: Option<SecretEnvSource>
}

#[derive(Debug, Decode)]
pub struct ConfigMapEnvSource {
    name: Option<String>,
    optional: Option<bool>
}

#[derive(Debug, Decode)]
pub struct SecretEnvSource {
    name: Option<String>,
    optional: Option<bool>
}

#[derive(Debug, Decode)]
pub struct VolumeMount {
    mount_path: PathBuf,
    name: String,
    mount_propagation: Option<MountPropagation>,
    read_only: Option<bool>,
    sub_path: Option<String>,
    sub_path_expr: Option<String>
}

/// <https://kubernetes.io/docs/concepts/storage/volumes/#mount-propagation>
#[derive(Debug, Decode, Default)]
pub enum MountPropagation {
    #[default]
    None,
    HostToContainer,
    Bidirectional
}

#[derive(Debug, Decode)]
pub struct VolumeDevice {
    device_path: PathBuf,
    name: String
}

#[derive(Debug, Decode)]
pub struct ResourceRequirements {
    limits: Option<HashMap<String, Quantity>>,
    requests: Option<HashMap<String, Quantity>>
}

#[derive(Debug, Decode, Default)]
pub enum TerminationMessagePolicy {
    FallbackToLogsOnError,
    #[default]
    File
}

#[derive(Debug, Decode)]
pub struct Lifecycle {
    post_start: Option<LifecycleHandler>,
    pre_stop: Option<LifecycleHandler>,
}

#[derive(Debug, Decode)]
pub struct LifecycleHandler {
    exec: Option<ExecAction>,
    http_get: Option<HTTPGetAction>,
    tcp_socket: Option<TCPSocketAction>,
}

#[derive(Debug, Decode)]
pub struct ExecAction {
    command: Vec<String>
}

#[derive(Debug, Decode)]
pub struct HTTPGetAction {
    port: u16,
    host: Option<String>,
    http_headers: Vec<HTTPHeader>,
    path: Option<PathBuf>,
    scheme: Option<String>
}

#[derive(Debug, Decode)]
pub struct HTTPHeader {
    name: String,
    value: String
}

#[derive(Debug, Decode)]
pub struct TCPSocketAction {
    port: u16,
    host: Option<String>
}

#[derive(Debug, Decode)]
pub struct Probe {
    exec: Option<ExecAction>,
    http_get: Option<HTTPGetAction>,
    tcp_socket: Option<TCPSocketAction>,
    initial_delay_seconds: Option<i32>,
    termination_grace_period_seconds: Option<i64>,
    period_seconds: Option<i32>,
    timeout_seconds: Option<i32>,
    failure_threshold: Option<i32>,
    success_threshold: Option<i32>,
    grpc: Option<GRPCAction>,
}

#[derive(Debug, Decode)]
pub struct GRPCAction {
    port: u16,
    service: Option<String>
}

#[derive(Debug, Decode)]
pub struct SecurityContext {
    run_as_user: Option<i64>,
    run_as_non_root: Option<bool>,
    run_as_group: Option<i64>,
    read_only_root_filesystem: Option<bool>,
    proc_mount: Option<String>,
    privileged: Option<bool>,
    allow_privilege_escalation: Option<bool>,
    capabilities: Option<Capabilities>,
    seccomp_profile: Option<SeccompProfile>,
    se_linux_options: Option<SELinuxOptions>,
    windows_options: Option<WindowsSecurityContextOptions>
}

#[derive(Debug, Decode)]
pub struct Capabilities {
    add: Vec<String>,
    drop: Vec<String>,
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#EphemeralContainer>
#[derive(Debug, Decode)]
pub struct EphemeralContainer {
    name: String,
    target_container_name: Option<String>
}

#[derive(Debug, Decode)]
pub struct PodOS {
    /// Name is the name of the operating system. The currently supported values are `Linux` and `Windows`. Additional value may be defined in future and can be one of: <https://github.com/opencontainers/runtime-spec/blob/master/config.md#platform-specific-configuration>
    ///
    /// Clients should expect to handle additional values and treat unrecognised values in this field as os: null
    name: PodOSName
}

#[derive(Debug, Decode)]
pub enum PodOSName {
    Linux,
    Windows
}

/// Affinity is a group of affinity scheduling rules.
#[derive(Debug, Decode)]
pub struct Affinity {
    /// Describes node affinity scheduling rules for the pod.
    node_affinity: Option<NodeAffinity>,
    /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
    pod_affinity: Option<PodAffinity>,
    /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
    pod_anti_affinity: Option<PodAntiAffinity>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#NodeAffinity>
#[derive(Debug, Decode)]
pub struct NodeAffinity {
    preferred_during_scheduling_ignored_during_execution:
        Vec<PreferredSchedulingTerm>,
    required_during_scheduling_ignored_during_execution: Option<NodeSelector>
}

#[derive(Debug, Decode)]
pub struct PreferredSchedulingTerm {
    preference: NodeSelectorTerm,
    weight: i32
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#PodAffinity>
#[derive(Debug, Decode)]
pub struct PodAffinity {
    preferred_during_scheduling_ignored_during_execution:
        Vec<WeightedPodAffinityTerm>,
    required_during_scheduling_ignored_during_execution:
        Vec<PodAffinityTerm>
}

#[derive(Debug, Decode)]
pub struct WeightedPodAffinityTerm {
    pod_affinity_term: PodAffinityTerm,
    weight: i32
}

#[derive(Debug, Decode)]
pub struct PodAffinityTerm {
    topology_key: String,
    label_selector: Option<LabelSelector>,
    namespace_selector: Option<LabelSelector>,
    namespaces: Vec<String>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#PodAntiAffinity>
#[derive(Debug, Decode)]
pub struct PodAntiAffinity {
    preferred_during_scheduling_ignored_during_execution:
        Vec<WeightedPodAffinityTerm>,
    required_during_scheduling_ignored_during_execution:
        Vec<PodAffinityTerm>
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator .
#[derive(Debug, Decode)]
pub struct Toleration {
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, [`operator`][Self::operator] must be `Exists`; this combination means to match all values and all keys.
    key: Option<String>,
    /// `operator` represents a key's relationship to the value. Valid operators are `Exists` and `Equal`. Defaults to `Equal`. `Exists` is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    // #[kfl(rename(serialize = "TolerationOperator::Equal"))]
    operator: TolerationOperator,
    /// Value is the taint value the toleration matches to. If the [`operator`][Self::operator] is `Exists`, the value should be empty, otherwise just a regular string.
    value: Option<String>,
    /// `effect` indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are `NoSchedule`, `PreferNoSchedule` and `NoExecute`.
    effect: Option<TaintEffect>,
    /// `toleration_seconds` represents the period of time the toleration (which must be of effect `NoExecute`, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    toleration_seconds: Option<u64>
}

#[derive(Debug, Decode, Default)]
pub enum TolerationOperator {
    Exists,
    #[default]
    Equal
}

#[derive(Debug, Decode)]
pub enum TaintEffect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute
}

#[derive(Debug, Decode, Default)]
pub enum PreemptionPolicy {
    Never,
    #[default]
    PreemptLowerPriority
}

#[derive(Debug, Decode)]
pub struct TopologySpreadConstraint {
    max_skew: i32,
    topology_key: String,
    when_unsatisfiable: WhenUnsatisfiable,
    label_selector: Option<LabelSelector>,
    match_label_keys: Vec<String>,
    min_domains: Option<i32>,
    node_affinity_policy: Option<String>,
    node_taints_policy: Option<String>,
}

#[derive(Debug, Decode, Default)]
pub enum WhenUnsatisfiable {
    #[default]
    DoNotSchedule,
    ScheduleAnyway
}

#[derive(Debug, Decode, Default)]
pub enum RestartPolicy {
    #[default]
    Always,
    OnFailure,
    Never
}

#[derive(Debug, Decode)]
pub struct PodReadinessGate {
    condition_type: String
}

#[derive(Debug, Decode)]
pub struct HostAlias {
    hostnames: Vec<String>,
    ip: Option<String>
}

#[derive(Debug, Decode)]
pub struct PodDNSConfig {
    nameservers: Vec<String>,
    options: Vec<PodDNSConfigOption>,
    searches: Vec<String>
}

#[derive(Debug, Decode)]
pub struct PodDNSConfigOption {
    name: String,
    value: Option<String>,
}

#[derive(Debug, Decode, Default)]
pub enum PodDNSPolicy {
    ClusterFirstWithHostNet,
    #[default]
    ClusterFirst,
    Default,
    None
}

/// `PodSecurityContext` holds pod-level security attributes and common container settings. Some fields are also present in [`container.security_context`][Container::security_context]. Field values of [`container.security_context`][Container::security_context] take precedence over field values of `PodSecurityContext`.
#[derive(Debug, Decode)]
pub struct PodSecurityContext {
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext. If set in both SecurityContext and `PodSecurityContext`, the value specified in `SecurityContext` takes precedence for that container. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    run_as_user: Option<u32>,
    /// Indicates that the container must run as a non-root user. If `true`, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or `false`, no such validation will be performed. May also be set in `SecurityContext`. If set in both `SecurityContext` and `PodSecurityContext`, the value specified in `SecurityContext` takes precedence.
    run_as_non_root: Option<bool>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in `SecurityContext`. If set in both `SecurityContext` and `PodSecurityContext`, the value specified in `SecurityContext` takes precedence for that container. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    run_as_group: Option<u32>,
    /// A list of groups applied to the first process run in each container, in addition to the container's primary GID. If unspecified, no groups will be added to any container. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    supplemental_groups: Vec<i64>,
    /**
    A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:

    1. The owning GID will be the FSGroup
    2. The setgid bit is set (new files created in the volume will be owned by FSGroup)
    3. The permission bits are OR'd with rw-rw----

    If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    */
    fs_group: Option<i64>,
    /// `fs_group_change_policy` defines behaviour of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support `fs_group` based ownership (and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are `OnRootMismatch` and `Always`. If not specified, `Always` is used. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    #[kfl(default = FSGroupChangePolicy::Always)]
    fs_group_change_policy: FSGroupChangePolicy,
    /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    seccomp_profile: Option<SeccompProfile>,
    /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container. May also be set in SecurityContext. If set in both SecurityContext and `PodSecurityContext`, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    se_linux_options: Option<SELinuxOptions>,
    /// `sysctls` hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when [`spec.os.name`][PodOS::name] is `Windows`.
    sysctls: Vec<Sysctl>,
    windows_options: Option<WindowsSecurityContextOptions>
}

#[derive(Debug, Decode, Default)]
pub enum FSGroupChangePolicy {
    OnRootMismatch,
    #[default]
    Always
}

#[derive(Debug, Decode)]
pub struct SeccompProfile {
    r#type: String,
    localhost_profile: Option<String>,
}

#[derive(Debug, Decode)]
pub struct SELinuxOptions {
    level: Option<String>,
    role: Option<String>,
    r#type: Option<String>,
    user: Option<String>,
}

#[derive(Debug, Decode)]
pub struct Sysctl {
    name: String,
    value: String
}

#[derive(Debug, Decode)]
pub struct WindowsSecurityContextOptions {
    gmsa_credential_spec: Option<String>,
    gmsa_credential_spec_name: Option<String>,
    host_process: Option<bool>,
    run_as_user_name: Option<String>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/#PodStatus>
#[derive(Debug, Decode)]
pub struct PodStatus {
    nominated_node_name: String,
    host_ip: String,
    start_time: Time,
    phase: String,
    message: String,
    reason: String,
    pod_ip: String,
    pod_ips: Vec<PodIP>,
    conditions: Vec<Condition>,
    qos_class: Option<String>,
    init_container_statuses: Vec<ContainerStatus>,
    container_statuses: Vec<ContainerStatus>,
    ephemeral_container_statuses: Vec<ContainerStatus>,
}

#[derive(Debug, Decode)]
pub struct PodIP {
    ip: String
}

#[derive(Debug, Decode)]
pub struct ContainerStatus {
    name: String,
    image: String,
    image_id: String,
    container_id: Option<String>,
    state: Option<ContainerState>,
    last_state: Option<ContainerState>,
    ready: bool,
    restart_count: i32,
    started: Option<bool>
}

#[derive(Debug, Decode)]
pub struct ContainerState {
    running: Option<ContainerStateRunning>,
    terminated: Option<ContainerStateTerminated>,
    waiting: Option<ContainerStateWaiting>
}

#[derive(Debug, Decode)]
pub struct ContainerStateRunning {
    started_at: Option<Time>
}

#[derive(Debug, Decode)]
pub struct ContainerStateTerminated {
    container_id: Option<String>,
    exit_code: Option<i32>,
    started_at: Option<Time>,
    finished_at: Option<Time>,
    message: Option<String>,
    reason: Option<String>,
    signal: Option<i32>,
}

#[derive(Debug, Decode)]
pub struct ContainerStateWaiting {
    message: Option<String>,
    reason: Option<String>,
}
