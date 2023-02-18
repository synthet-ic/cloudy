/*!
<https://github.com/grpc/grpc-go/tree/master/serviceconfig>
*/

pub struct Config;

/**
`ParseResult` contains a service config or an error. Exactly one must be non-nil.
*/
pub enum ParseResult {
    Config(Config),
    Err(String)
}
