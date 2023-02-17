pub mod client;
pub mod token_request;
pub mod token_review;

use kfl::Decode;

pub use client::Client;
pub use token_request::TokenRequest;
pub use token_review::TokenReview;

// #[derive(Debug, Decode)]
// pub enum Authentication {
//     Client(Client),
//     TokenRequest(TokenRequest),
//     TokenReview(TokenReview)
// }
