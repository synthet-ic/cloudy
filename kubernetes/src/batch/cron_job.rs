//! - Concepts <https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/cron-job-v1/>

use kfl::{Decode, DecodeScalar};

use crate::{
    batch::job,
    core::Reference,
    meta::Metadata,
    time::Time
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/cron-job-v1/#CronJob>
/// CronJob represents the configuration of a single cron job.
#[derive(Debug, Decode)]
pub struct CronJob {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/cron-job-v1/#CronJobSpec>
///
/// Spec describes how the job execution will look like and when it will actually run.
#[derive(Debug, Decode)]
pub struct Spec {
    /// Specifies the job that will be created when executing a CronJob.
    job_template: JobTemplateSpec,
    /// The schedule in Cron format, see <https://en.wikipedia.org/wiki/Cron>.
    schedule: String,
    /// Time zone name for the given schedule, see <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>. If not specified, this will default to the time zone of the kube-controller-manager process. The set of valid time zone names and the time zone offset is loaded from the system-wide time zone database by the API server during CronJob validation and the controller manager during execution. If no system-wide time zone database can be found a bundled version of the database is used instead. If the time zone name becomes invalid during the lifetime of a CronJob or due to a change in host configuration, the controller will stop creating new new Jobs and will create a system event with the reason UnknownTimeZone. More information can be found in <https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/#time-zones> This is beta field and must be enabled via the `CronJobTimeZone` feature gate.
    time_zone: Option<String>,
    /// Specifies how to treat concurrent executions of a Job. Valid values are:
    /// - `Allow` (default): allows CronJobs to run concurrently;
    /// - `Forbid`: forbids concurrent runs, skipping next run if previous run hasn't finished yet;
    /// - `Replace`: cancels currently running job and replaces it with a new one.
    concurrency_policy: Option<ConcurrencyPolicy>,
    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason. Missed jobs executions will be counted as failed ones.
    starting_deadline_seconds: Option<u64>,
    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions. Defaults to `false`.
    suspend: Option<bool>,
    /// The number of successful finished jobs to retain. Value must be non-negative integer. Defaults to 3.
    successful_jobs_history_limit: Option<u32>,
    /// The number of failed finished jobs to retain. Value must be non-negative integer. Defaults to 1.
    failed_jobs_history_limit: Option<u32>
}

/// JobTemplateSpec describes the data a Job should have when created from a template.
#[derive(Debug, Decode)]
pub struct JobTemplateSpec {
    metadata: Option<Metadata>,
    /// Specification of the desired behaviour of the job. More info: <https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    spec: Option<job::Spec>
}

#[derive(Debug, DecodeScalar, Default)]
pub enum ConcurrencyPolicy {
    #[default]
    Allow,
    Forbid,
    Replace
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/cron-job-v1/#CronJobStatus>
///
/// ronJobStatus represents the current state of a cron job.
#[derive(Debug, Decode)]
pub struct Status {
    /// A list of pointers to currently running jobs.
    active: Vec<Reference>,
    /// Information when was the last time the job was successfully scheduled.
    last_schedule_time: Time,
    /// Information when was the last time the job successfully completed.
    last_successful_time: Time
}
