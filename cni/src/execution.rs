/// <https://www.cni.dev/docs/spec/#attachment-parameters>
pub struct AttachmentParameters {
    /// Container ID: A unique plaintext identifier for a container, allocated by the runtime. Must not be empty. Must start with an alphanumeric character, optionally followed by any combination of one or more alphanumeric characters, underscore (_), dot (.) or hyphen (-). During execution, always set as the `CNI_CONTAINERID` parameter.
    container_id: String,
    /// Namespace: A reference to the container’s 'isolation domain'. If using network namespaces, then a path to the network namespace (e.g. `/run/netns/[nsname]`). During execution, always set as the `CNI_NETNS` parameter.
    namespace: String,
    /// Container interface name: Name of the interface to create inside the container. During execution, always set as the `CNI_IFNAME` parameter.
    container_interface_name: String,
    /// Generic Arguments: Extra arguments, in the form of key-value string pairs, that are relevant to a specific attachment. During execution, always set as the `CNI_ARGS` parameter.
    generic_arguments: String,
    /// Capability Arguments: These are also key-value pairs. The key is a string, whereas the value is any JSON-serializable type. The keys and values are defined by [convention](https://www.cni.dev/docs/conventions/).
    capability_arguments: String
}
