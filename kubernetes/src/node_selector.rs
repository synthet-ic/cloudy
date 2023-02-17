use kfl::Decode;

use crate::core::node_selector_requirement::NodeSelectorRequirement;

#[derive(Debug, Decode)]
pub struct NodeSelector {
    #[kfl(children)]
    node_selector_terms: Vec<NodeSelectorTerm>,
}

#[derive(Debug, Decode)]
pub struct NodeSelectorTerm {
    #[kfl(children)]
    match_expressions: Vec<NodeSelectorRequirement>,
    #[kfl(children)]
    match_fields: Vec<NodeSelectorRequirement>
}
