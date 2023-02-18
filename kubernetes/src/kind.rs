use kfl::Decode;

#[derive(Debug, Decode)]
pub enum Kind {
    ClusterRole,
    ServiceAccount
}
