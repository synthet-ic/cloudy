use kfl::Decode;

#[derive(Debug, Decode)]
pub enum Protocol {
    Tcp,
    Udp,
    Sctp
}
