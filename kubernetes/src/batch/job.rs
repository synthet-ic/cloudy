//! - Concepts <https://kubernetes.io/docs/concepts/workloads/controllers/job/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/job-v1/>

use kfl::{Decode, DecodeScalar};

use crate::{
    core::pod_template::PodTemplateSpec,
    meta::{
        condition::Condition,
        label_selector::LabelSelector,
        metadata::Metadata
    },
    time::Time
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/job-v1/#Job>
///
/// Job represents the configuration of a single job.
#[derive(Debug, Decode)]
pub struct Job {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/job-v1/#JobSpec>
///
/// Spec describes how the job execution will look like.
#[derive(Debug, Decode)]
pub struct Spec {
    // Replicas

    /// Describes the pod that will be created when executing a job.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/job/>
    template: PodTemplateSpec,
    /// Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism), i.e. when the work left to do is less than max parallelism.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/job/>
    parallelism: Option<u32>,

    // Lifecycle

    /// Specifies the desired number of successfully finished pods the job should be run with. Setting to nil means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value. Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/job/>
    completions: Option<u32>,
    /// Specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`.
    ///
    /// - `NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other.
    ///
    /// - `Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, [`.spec.completions`][Self::completions] must be specified and [`.spec.parallelism`][Self::parallelism] must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`.
    ///
    /// More completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognise, which is possible during upgrades due to version skew, the controller skips updates for the Job.
    completion_mode: Option<CompletionMode>,
    /// Specifies the number of retries before marking this job failed. Defaults to 6
    #[kfl(default = 6)]
    backoff_limit: u32,
    /// Specifies the duration in seconds relative to the [`start_time`][Status::start_time] that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.
    active_deadline_seconds: Option<u64>,
    /// Limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, `ttl_seconds_after_finished` after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalisers) will be honoured. If this field is unset, the Job won't be automatically deleted. If this field is set to `0`, the Job becomes eligible to be deleted immediately after it finishes.
    #[kfl(default = 0)]
    ttl_seconds_after_finished: u32,
    /// Suspend specifies whether the Job controller should create Pods or not. If a Job is created with suspend set to `true`, no Pods are created by the Job controller. If a Job is suspended after creation (i.e. the flag goes from `false` to `true`), the Job controller will delete all active Pods associated with this Job. Users must design their workload to gracefully handle this. Suspending a Job will reset the [`start_time`][Status::start_time] field of the Job, effectively resetting the [`active_deadline_seconds`][Self::active_deadline_seconds] timer too. Defaults to `false`.
    suspend: Option<bool>,

    // Selector

    /// A label query over pods that should match the pod count. Normally, the system sets this field for you.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors>
    selector: Option<LabelSelector>,
    /// Controls generation of pod labels and pod selectors. Leave `manual_selector` unset unless you are certain what you are doing. When `false` or unset, the system pick labels unique to this job and appends those labels to the pod template. When `true`, the user is responsible for picking unique labels and specifying the selector. Failure to pick a unique label may cause this and other jobs not to function correctly. However, You may see `manual_selector` = `true` in jobs that were created with the old `extensions/v1beta1` API.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/job/#specifying-your-own-pod-selector>
    manual_selector: Option<bool>,

    // Alpha Level

    /// Specifies the policy of handling failed pods. In particular, it allows to specify the set of actions and conditions which need to be satisfied to take the associated action. If empty, the default behaviour applies - the counter of failed pods, represented by the jobs's [`.status.failed`][Status::failed] field, is incremented and it is checked against the [`backoff_limit`][Self::backoff_limit]. This field cannot be used in combination with [`restart_policy`][crate::core::pod::PodSpec::restart_policy] = `OnFailure`.
    ///
    /// This field is alpha-level. To use this field, you must enable the `JobPodFailurePolicy` feature gate (disabled by default).
    pod_failure_policy: Option<PodFailurePolicy>
}

#[derive(Debug, DecodeScalar)]
pub enum CompletionMode {
    NonIndexed,
    Indexed
}

/// PodFailurePolicy describes how failed pods influence the [`backoff_limit`][Spec::backoff_limit].
#[derive(Debug, Decode)]
pub struct PodFailurePolicy {
    /// A list of pod failure policy rules. The rules are evaluated in order. Once a rule matches a Pod failure, the remaining of the rules are ignored. When no rule matches the Pod failure, the default handling applies - the counter of pod failures is incremented and it is checked against the [`backoff_limit`][Spec::backoff_limit]. At most `20` elements are allowed.
    rules: Vec<Rule>
}

/// Rule describes how a pod failure is handled when the requirements are met. One of [`on_exit_codes`][Self::on_exit_codes] and [`on_pod_conditions`][Self::on_pod_conditions], but not both, can be used in each rule.
#[derive(Debug, Decode)]
pub struct Rule {
    /// Specifies the action taken on a pod failure when the requirements are satisfied.
    action: Action,
    /// Represents the requirement on the pod conditions. The requirement is represented as a list of pod condition patterns. The requirement is satisfied if at least one pattern matches an actual pod condition. At most `20` elements are allowed.
    on_pod_conditions: Vec<OnPodCondition>,
    /// Represents the requirement on the container exit codes.
    on_exit_codes: Option<OnExitCodes>,
}

#[derive(Debug, DecodeScalar)]
pub enum Action {
    /// Indicates that the pod's job is marked as Failed and all running pods are terminated.
    FailJob,
    /// Indicates that the counter towards the [`.backoff_limit`][Spec::backoff_limit] is not incremented and a replacement pod is created.
    Ignore,
    /// Indicates that the pod is handled in the default way - the counter towards the [`.backoff_limit`][Spec::backoff_limit] is incremented. Additional values are considered to be added in the future. Clients should react to an unknown action by skipping the rule.
    Count
}

/// OnPodCondition describes a pattern for matching an actual pod condition type.
#[derive(Debug, Decode)]
pub struct OnPodCondition {
    /// Specifies the required Pod condition status. To match a pod condition it is required that the specified status equals the pod condition status. Defaults to `True`.
    status: on_pod_conditions::Status,
    /// Specifies the required Pod condition type. To match a pod condition it is required that specified type equals the pod condition type.
    r#type: on_pod_conditions::Type
}

pub mod on_pod_conditions {
    use kfl::DecodeScalar;

    /// <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-conditions>
    #[derive(Debug, DecodeScalar)]
    pub enum Status {
        True,
        False,
        Unknown,
    }

    /// <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-conditions>
    #[derive(Debug, DecodeScalar)]
    pub enum Type {
        PodScheduled,
        PodHasNetwork,
        ContainersReady,
        Initialised,
        Ready
    }
}

/// OnExitCodes describes the requirement for handling a failed pod based on its container exit codes. In particular, it lookups the [`.state.terminated.exit_code`][crate::core::pod::ContainerStateTerminated::exit_code] for each app container and init container status, represented by the [`.status.container_statuses`][crate::core::pod::PodStatus::container_statuses] and [`.status.init_container_statuses`][crate::core::pod::PodStatus::init_container_statuses] fields in the Pod status, respectively. Containers completed with success (exit code 0) are excluded from the requirement check.
#[derive(Debug, Decode)]
pub struct OnExitCodes {
    /// Represents the relationship between the container exit code(s) and the specified values. Containers completed with success (exit code 0) are excluded from the requirement check. Possible values are:
    ///
    /// - `In`: the requirement is satisfied if at least one container exit code (might be multiple if there are multiple containers not restricted by the [`container_name`][Self::container_name] field) is in the set of specified values.
    ///
    /// - `NotIn`: the requirement is satisfied if at least one container exit code (might be multiple if there are multiple containers not restricted by the [`container_name`][Self::container_name] field) is not in the set of specified values. Additional values are considered to be added in the future. Clients should react to an unknown operator by assuming the requirement is not satisfied.
    operator: Operator,
    /// Specifies the set of values. Each returned container exit code (might be multiple in case of multiple containers) is checked against this set of values with respect to the operator. The list of values must be ordered and must not contain duplicates. Value `0` cannot be used for the `In` operator. At least one element is required. At most `255` elements are allowed.
    values: Vec<i32>,
    /// Restricts the check for exit codes to the container with the specified name. When `null`, the rule applies to all containers. When specified, it should match one the container or `init_container` names in the pod template.
    container_name: Option<String>
}

#[derive(Debug, Decode)]
pub enum Operator {
    In,
    NotIn
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/job-v1/#JobStatus>
///
/// Status represents the current state of a Job.
#[derive(Debug, Decode)]
pub struct Status {
    /// Represents time when the job controller started processing a job. When a Job is created in the suspended state, this field is not set until the first time it is resumed. This field is reset every time a Job is resumed from suspension. It is represented in [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339) form and is in UTC.
    start_time: Time,
    completion_time: Time,
    active: u32,
    failed: u32,
    succeeded: u32,
    completed_indexes: String,
    conditions: Vec<Condition>,
    uncounted_terminated_pods: UncountedTerminatedPods,
    /// Beta Level
    ready: u32
}

#[derive(Debug, Decode)]
pub struct UncountedTerminatedPods {
    failed: Vec<String>,
    succeeded: Vec<String>
}
