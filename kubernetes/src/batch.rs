pub mod cron_job;
pub mod job;

use kfl::Decode;

pub use cron_job::CronJob;
pub use job::Job;

// #[derive(Debug, Decode)]
// pub enum Batch {
//     CronJob(CronJob),
//     Job(Job)
// }
