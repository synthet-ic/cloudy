pub struct CIDRPolicy {
    /// List of CIDR egress rules
    egress: Vec<PolicyRule>,

    /// List of CIDR ingress rules
    ingress: Vec<PolicyRule>
}
