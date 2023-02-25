/**
<https://argoproj.github.io/argo-workflows/fields/#workflow>
*/
pub struct Workflow {
    spec: WorkflowSpec,
    status: WorkflowStatus
}

/// <https://argoproj.github.io/argo-workflows/fields/#workflowspec>
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct WorkflowSpec {
    active_deadline_seconds: i32,
    affinity: Affinity,
    archive_logs: bool,
    arguments: Arguments,
    #[serde(serialize = "artifactGC")]
    artifact_gc: ArtifactGC,
    artifact_repository_ref: ArtifactRepositoryRef,
    automountServiceAccountToken: bool,
    
}

/// <https://argoproj.github.io/argo-workflows/fields/#workflowstatus>
pub struct WorkflowStatus {
    
}
