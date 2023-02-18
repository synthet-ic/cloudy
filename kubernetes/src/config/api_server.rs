pub mod admission_configuration;
pub mod encryption;

// use kfl::Decode;

pub use admission_configuration::AdmissionConfiguration;
pub use encryption::EncryptionConfiguration;

// #[derive(Debug, Decode)]
// pub enum ApiServer {
//     AdmissionConfiguration(AdmissionConfiguration),
//     EncryptionConfiguration(EncryptionConfiguration)
// }
