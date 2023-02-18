//! <https://github.com/kubernetes/kubernetes/tree/master/staging/src/k8s.io/cri-api>

pub mod runtime;

use std::time::Duration;

use runtime::{
    AttachRequest,
    AttachResponse,
    CheckpointContainerRequest,
    Container,
    ContainerConfig,
    ContainerEventResponse,
    ContainerFilter,
    ContainerResources,
    ContainerStatusResponse,
    ExecRequest,
    ExecResponse,
    PodSandboxConfig
};

/// Contains methods to manipulate containers managed by a container runtime. The methods are thread-safe.
pub trait ContainerManager {
    /// Creates a new container in specified PodSandbox.
    fn create_container(
        &self,
        pod_sandbox_id: String,
        config: &ContainerConfig,
        sandbox_config: &PodSandboxConfig
    ) -> Result<String>;

    /// Starts the container.
    fn start_container(&self, container_id: String) -> Result<()>;

    /// Stops a running container with a grace period (i.e., timeout).
    fn stop_container(&self, container_id: String, timeout: i64) -> Result<()>;

    /// Removes the container.
    fn remove_container(&self, container_id: String) -> Result<()>;

    /// Lists all containers by filters.
    fn list_containers(
        &self,
        filter: &ContainerFilter
    ) -> Result<Vec<&Container>>;

    /// Returns the status of the container.
    fn container_status(
        &self, container_id: String, verbose: bool
    ) -> Result<&ContainerStatusResponse>;

    /// Updates ContainerConfig of the container synchronously.
    /// If runtime fails to transactionally update the requested resources, an error is returned.
    fn update_container_resources(
        &self,
        container_id: String,
        resources: &ContainerResources
    ) -> Result<()>;

    /// Executes a command in the container, and returns the stdout output.
    /// If command exits with a non-zero exit code, an error is returned.
    fn exec_sync(
        &self,
        container_id: String,
        cmd: Vec<String>,
        timeout: Duration
    ) -> (stdout Vec<u8>, stderr Vec<u8>, err error);
    
    /// Prepares a streaming endpoint to execute a command in the container, and returns the address.
    fn exec(&self, req: &ExecRequest) -> Result<&ExecResponse>;

    /// Attach prepares a streaming endpoint to attach to a running container, and returns the address.
    fn attach(
        &self,
        req: &AttachRequest
    ) -> Result<&AttachResponse>;

    /// Asks runtime to reopen the stdout/stderr log file
    /// for the container. If it returns error, new container log file MUST NOT
    /// be created.
    fn reopen_container_log(&self, container_id: String) -> error;

    /// Checkpoints a container.
    fn checkpoint_container(
        &self,
        options: &CheckpointContainerRequest
    ) -> Result<()>;

    /// Gets container events from the CRI runtime.
    fn get_container_events(
        &self,
        container_events_ch: chan &ContainerEventResponse
    ) -> Result<()>;
}
