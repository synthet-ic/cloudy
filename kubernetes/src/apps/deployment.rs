/*!
- Concepts <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/>
- Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/deployment-v1/>
```toml
*/
#![doc = include_str!("../../examples/controllers/nginx-deployment.toml")]
/*!
```
*/

use kfl::Decode;

use crate::{
    core::pod_template::PodTemplateSpec,
    meta::{
        condition::Condition,
        label_selector::Selector,
        metadata::Metadata
    },
    IntOrString
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/deployment-v1/#Deployment>
#[derive(Debug, Decode)]
pub struct Deployment {
    metadata: Option<Metadata>,
    spec: Option<Spec>,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/deployment-v1/#DeploymentSpec>
///
/// Spec is the specification of the desired behaviour of the Deployment.
#[derive(Debug, Decode)]
pub struct Spec {
    /// Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment. It must match the pod template's labels.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#selector>
    ///
    /// `selector` is a required field that specifies a [label selector](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/) for the Pods targeted by this Deployment.
    ///
    /// `selector` must match [`template.metadata.labels`][crate::meta::metadata::Metadata::labels], or it will be rejected by the API.
    ///
    /// In API version `apps/v1`, `selector` and `.metadata.labels` do not default to [`template.metadata.labels`][crate::meta::metadata::Metadata::labels] if not set. So they must be set explicitly. Also note that `selector` is immutable after creation of the Deployment in `apps/v1`.
    ///
    /// A Deployment may terminate Pods whose labels match the selector if their template is different from [`template`][Self::template] or if the total number of such Pods exceeds [`replicas`][Self::replicas]. It brings up new Pods with [`template`][Self::template] if the number of Pods is less than the desired number.
    ///
    /// > **Note**: You should not create other Pods whose labels match this selector, either directly, by creating another Deployment, or by creating another controller such as a ReplicaSet or a ReplicationController. If you do so, the first Deployment thinks that it created these other Pods. Kubernetes does not stop you from doing this.
    ///
    /// If you have multiple controllers that have overlapping selectors, the controllers will fight with each other and won't behave correctly.
    selector: Selector,

    /// `template` describes the pods that will be created.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#pod-template>
    ///
    /// The `template` and [`selector`][Self::selector] are the only required fields of the [`spec`][Deployment::spec].
    ///
    /// The `template` is a [Pod template](https://kubernetes.io/docs/concepts/workloads/pods/#pod-templates). It has exactly the same schema as a [Pod](https://kubernetes.io/docs/concepts/workloads/pods/), except it is nested and does not have an `api-version` or `kind`.
    ///
    /// In addition to required fields for a Pod, a Pod template in a Deployment must specify appropriate labels and an appropriate restart policy. For labels, make sure not to overlap with other controllers. See [`selector`][Self::selector].
    ///
    /// Only a [`template.spec.restart_policy`][crate::core::pod::PodSpec::restart_policy] equal to `Always` is allowed, which is the default if not specified.
    template: PodTemplateSpec,

    /// Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to `1`.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#replicas>
    ///
    /// `replicas` is an optional field that specifies the number of desired Pods. It defaults to `1`.
    ///
    /// Should you manually scale a Deployment, example via `kubectl scale deployment deployment --replicas=X`, and then you update that Deployment based on a manifest (for example: by running `kubectl apply -f deployment.yaml`), then applying that manifest overwrites the manual scaling that you previously did.
    ///
    /// If a [HorizontalPodAutoscaler](https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/) (or any similar API for horizontal scaling) is managing scaling for a Deployment, don't set [`replicas`][Self::replicas].
    ///
    /// Instead, allow the Kubernetes control plane to manage the `replicas` field automatically.
    replicas: Option<u16>,

    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to `0` (pod will be considered available as soon as it is ready).
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#min-ready-seconds>
    ///
    /// `min_ready_seconds` is an optional field that specifies the minimum number of seconds for which a newly created Pod should be ready without any of its containers crashing, for it to be considered available. This defaults to `0` (the Pod will be considered available as soon as it is ready). To learn more about when a Pod is considered ready, see [Container Probes](https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes).
    min_ready_seconds: Option<u16>,

    /// The deployment strategy to use to replace existing pods with new ones.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#strategy>
    ///
    /// `strategy` specifies the strategy used to replace old Pods by new ones. `strategy.type` can be `Recreate` or `RollingUpdate`. `RollingUpdate` is the default value.
    ///
    /// - All existing Pods are killed before new ones are created when `strategy.type` = `Recreate`.
    ///
    /// > **Note**: This will only guarantee Pod termination previous to creation for upgrades. If you upgrade a Deployment, all Pods of the old revision will be terminated immediately. Successful removal is awaited before any Pod of the new revision is created. If you manually delete a Pod, the lifecycle is controlled by the ReplicaSet and the replacement will be created immediately (even if the old Pod is still in a Terminating state). If you need an 'at most' guarantee for your Pods, you should consider using a [StatefulSet](https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/).
    ///
    /// - The Deployment updates Pods in a rolling update fashion when `strategy.type` = `RollingUpdate`. You can specify `max_unavailable` and `max_surge` to control the rolling update process.
    strategy: Option<DeploymentStrategy>,

    /// The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to `10`.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#revision-history-limit>
    ///
    /// A Deployment's revision history is stored in the ReplicaSets it controls.
    ///
    /// `revision_history_limit` is an optional field that specifies the number of old ReplicaSets to retain to allow rollback. These old ReplicaSets consume resources in `etcd` and crowd the output of **kubectl get replicasets**. The configuration of each Deployment revision is stored in its ReplicaSets; therefore, once an old ReplicaSet is deleted, you lose the ability to rollback to that revision of Deployment. By default, `10` old ReplicaSets will be kept, however its ideal value depends on the frequency and stability of new Deployments.
    ///
    /// More specifically, setting this field to `0` means that all old ReplicaSets with 0 replicas will be cleaned up. In this case, a new Deployment rollout cannot be undone, since its revision history is cleaned up.
    revision_history_limit: Option<u16>,

    /// The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a `ProgressDeadlineExceeded` reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. Defaults to `600`.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#progress-deadline-seconds>
    ///
    /// `progress_deadline_seconds` is an optional field that specifies the number of seconds you want to wait for your Deployment to progress before the system reports back that the Deployment has [failed progressing](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#failed-deployment) - surfaced as a condition with `type`: `Progressing`, `status`: `False`. and `reason`: `ProgressDeadlineExceeded` in the status of the resource. The Deployment controller will keep retrying the Deployment. This defaults to 600. In the future, once automatic rollback will be implemented, the Deployment controller will roll back a Deployment as soon as it observes such a condition.
    ///
    /// If specified, this field needs to be greater than [`min_ready_seconds`][Self::min_ready_seconds].
    progress_deadline_seconds: Option<u16>,
    /// Indicates that the deployment is paused.
    ///
    /// # Concepts
    ///
    /// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#paused>
    ///
    /// `paused` is an optional boolean field for pausing and resuming a Deployment. The only difference between a paused Deployment and one that is not paused, is that any changes into the PodTemplateSpec of the paused Deployment will not trigger new rollouts as long as it is paused. A Deployment is not paused by default when it is created.
    paused: Option<bool>
}

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Debug, Decode)]
pub enum DeploymentStrategy {
    Recreate,
    // #[default]
    RollingUpdate(RollingUpdateDeployment)
}

/// Spec to control the desired behaviour of rolling update.
#[derive(Debug, Decode)]
pub struct RollingUpdateDeployment {
    /**
    The maximum number of pods that can be scheduled above the desired number of pods. Value can be an absolute number (ex: `5`) or a percentage of desired pods (ex: `10%`). This can not be `0` if [`max_unavailable`][Self::max_unavailable] is `0`. Absolute number is calculated from percentage by rounding up. Defaults to `25%`. Example: when this is set to `30%`, the new ReplicaSet can be scaled up immediately when the rolling update starts, such that the total number of old and new pods do not exceed 130% of desired pods. Once old pods have been killed, new ReplicaSet can be scaled up further, ensuring that total number of pods running at any time during the update is at most 130% of desired pods.
    */
    max_surge: Option<IntOrString>,
    
    /**
    The maximum number of pods that can be unavailable during the update. Value can be an absolute number (ex: `5`) or a percentage of desired pods (ex: `10%`). Absolute number is calculated from percentage by rounding down. This can not be `0` if [`max_surge`][Self::max_surge] is `0`. Defaults to `25%`. Example: when this is set to `30%`, the old ReplicaSet can be scaled down to 70% of desired pods immediately when the rolling update starts. Once new pods are ready, old ReplicaSet can be scaled down further, followed by scaling up the new ReplicaSet, ensuring that the total number of pods available at all times during the update is at least 70% of desired pods.
    */
    max_unavailable: Option<IntOrString>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/deployment-v1/#DeploymentStatus>
///
/// # Concepts
///
/// <https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#deployment-status>
///
/// A Deployment enters various states during its lifecycle. It can be progressing while rolling out a new ReplicaSet, it can be complete, or it can fail to progress.
#[derive(Debug, Decode)]
pub struct Status {
    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    replicas: Option<u16>,
    /// Total number of available pods (ready for at least [`min_ready_seconds`][Spec::min_ready_seconds]) targeted by this deployment.
    available_replicas: Option<u16>,
    /// `ready_replicas` is the number of pods targeted by this Deployment with a `Ready` Condition.
    ready_replicas: Option<u16>,
    /// Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
    unavailable_replicas: Option<u16>,
    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    updated_replicas: Option<u16>,
    /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
    collision_count: Option<u16>,
    /// Represents the latest available observations of a deployment's current state.
    conditions: Vec<Condition>,
    /// The generation observed by the deployment controller.
    observed_generation: Option<u32>,
}

pub enum DeploymentConditionType {
    Available,
    Progressing,
    ReplicaFailure
}

pub enum DeploymentConditionReason {
    MinimumReplicasAvailable,
    ReplicaSetUpdated,
    ProgressDeadlineExceeded,
    FailedCreate,
    NewReplicaSetAvailable
}
