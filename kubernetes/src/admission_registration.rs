pub mod mutating_webhook_configuration;
pub mod validating_webhook_configuration;

// use kfl::Decode;

pub use mutating_webhook_configuration::MutatingWebhookConfiguration;
pub use validating_webhook_configuration::ValidatingWebhookConfiguration;

// #[derive(Debug, Decode)]
// pub enum AdmissionRegistration {
//     MutatingWebhookConfiguration(MutatingWebhookConfiguration),
//     ValidatingWebhookConfiguration(ValidatingWebhookConfiguration)
// }
