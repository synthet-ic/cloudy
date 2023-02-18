pub mod gateway;
pub mod gateway_class;
pub mod http_route;

// use kfl::Decode;

pub use gateway::Gateway;
pub use gateway_class::GatewayClass;
pub use http_route::HttpRoute;

// #[derive(Debug, Decode)]
// pub enum NetworkGateway {
//     Gateway(Gateway),
//     GatewayClass(GatewayClass),
//     HttpRoute(HttpRoute)
// }
