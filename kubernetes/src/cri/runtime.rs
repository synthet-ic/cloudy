/// Runtime service defines the public APIs for remote container runtimes
pub trait RuntimeService {
    /// Version returns the runtime name, runtime version, and runtime API version.
    fn version(request: VersionRequest) -> VersionResponse;

    /// Creates and starts a pod-level sandbox. Runtimes must ensure
    /// the sandbox is in the ready state on success.
    fn run_pod_sandbox(
        request: RunPodSandboxRequest
    ) -> (RunPodSandboxResponse);

    /// Stops any running process that is part of the sandbox and
    /// reclaims network resources (e.g., IP addresses) allocated to the sandbox.
    /// If there are any running containers in the sandbox, they must be forcibly
    /// terminated.
    /// This call is idempotent, and must not return an error if all relevant
    /// resources have already been reclaimed. kubelet will call StopPodSandbox
    /// at least once before calling RemovePodSandbox. It will also attempt to
    /// reclaim resources eagerly, as soon as a sandbox is not needed. Hence,
    /// multiple StopPodSandbox calls are expected.
    fn stop_pod_sandbox(
        request: StopPodSandboxRequest
    ) -> StopPodSandboxResponse;
    
    /// Removes the sandbox. If there are any running containers
    /// in the sandbox, they must be forcibly terminated and removed.
    /// This call is idempotent, and must not return an error if the sandbox has
    /// already been removed.
    fn remove_pod_sandbox(
        request: RemovePodSandboxRequest
    ) -> RemovePodSandboxResponse;

    /// Returns the status of the PodSandbox. If the PodSandbox is not present, returns an error.
    fn pod_sandbox_status(
        request: PodSandboxStatusRequest
    ) -> (PodSandboxStatusResponse);
    /// Returns a list of PodSandboxes.
    fn list_pod_sandbox(
        request: ListPodSandboxRequest
    ) -> (ListPodSandboxResponse);

    /// Creates a new container in specified PodSandbox
    fn create_container(
        request: CreateContainerRequest
    ) -> (CreateContainerResponse);
    /// Starts the container.
    fn start_container(
        request: StartContainerRequest
    ) -> (StartContainerResponse);

    /// Stops a running container with a grace period (i.e., timeout).
    /// This call is idempotent, and must not return an error if the container has
    /// already been stopped.
    /// The runtime must forcibly kill the container after the grace period is
    /// reached.
    fn stop_container(
        request: StopContainerRequest
    ) -> (StopContainerResponse);

    /// Removes the container. If the container is running, the
    /// container must be forcibly removed.
    /// This call is idempotent, and must not return an error if the container has
    /// already been removed.
    fn remove_container(
        request: RemoveContainerRequest
    ) -> (RemoveContainerResponse);
    /// Lists all containers by filters.
    fn list_containers(
        request: ListContainersRequest
    ) -> (ListContainersResponse);

    /// Returns status of the container. If the container is not
    /// present, returns an error.
    fn container_status(
        request: ContainerStatusRequest
    ) -> (ContainerStatusResponse);

    /// Updates ContainerConfig of the container synchronously.
    /// If runtime fails to transactionally update the requested resources, an error is returned.
    fn update_container_resources(
        request: UpdateContainerResourcesRequest
    ) -> (UpdateContainerResourcesResponse);
    
    /// Asks runtime to reopen the stdout/stderr log file
    /// for the container. This is often called after the log file has been
    /// rotated. If the container is not running, container runtime can choose
    /// to either create a new log file and return nil, or return an error.
    /// Once it returns error, new container log file MUST NOT be created.
    fn reopen_container_log(request: ReopenContainerLogRequest) -> (ReopenContainerLogResponse);

    /// Runs a command in a container synchronously.
    fn exec_sync(request: ExecSyncRequest) -> (ExecSyncResponse);
    /// Prepares a streaming endpoint to execute a command in the container.
    fn exec(request: ExecRequest) -> (ExecResponse);
    /// Prepares a streaming endpoint to attach to a running container.
    fn attach(request: AttachRequest) -> (AttachResponse);
    /// Prepares a streaming endpoint to forward ports from a PodSandbox.
    fn port_forward(request: PortForwardRequest) -> (PortForwardResponse);

    /// ContainerStats returns stats of the container. If the container does not
    /// exist, the call returns an error.
    fn container_stats(
        request: ContainerStatsRequest
    ) -> (ContainerStatsResponse);
    /// Returns stats of all running containers.
    fn list_container_stats(
        request: ListContainerStatsRequest
    ) -> (ListContainerStatsResponse);

    /// Returns stats of the pod sandbox. If the pod sandbox does not
    /// exist, the call returns an error.
    fn pod_sandbox_stats(
        request: PodSandboxStatsRequest
    ) -> (PodSandboxStatsResponse);
    /// Returns stats of the pod sandboxes matching a filter.
    fn list_pod_sandbox_stats(
        request: ListPodSandboxStatsRequest
    ) -> (ListPodSandboxStatsResponse);

    /// Updates the runtime configuration based on the given request.
    fn update_runtime_config(
        request: UpdateRuntimeConfigRequest
    ) -> (UpdateRuntimeConfigResponse);

    /// Returns the status of the runtime.
    fn status(request: StatusRequest) -> (StatusResponse);

    /// Checkpoints a container
    fn checkpoint_container(
        request: CheckpointContainerRequest
    ) -> (CheckpointContainerResponse);

    /// Gets container events from the CRI runtime
    fn get_container_events(
        request: GetEventsRequest
    ) -> (stream ContainerEventResponse);
}

pub struct ContainerConfig {
    
}

pub struct PodSandboxConfig {
    
}

pub struct ContainerFilter {

}

pub struct Container {
    
}

pub struct ContainerStatusResponse {
    
}

pub struct ContainerResources {
    
}

pub struct ExecRequest {

}

pub struct ExecResponse {

}

pub struct AttachRequest {
    
}

pub struct AttachResponse {

}

pub struct CheckpointContainerRequest {

}

pub struct ContainerEventResponse {
    
}
