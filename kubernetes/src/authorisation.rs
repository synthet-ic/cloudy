pub mod rbac;

use kfl::Decode;

pub use rbac::Rbac;

// #[derive(Debug, Decode)]
// pub enum Authorisation {
//     #[kfl(rename = "")]
//     Rbac(Rbac)
// }
