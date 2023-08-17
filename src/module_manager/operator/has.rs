use ksp_cfg_formatter::parser::HasPredicate;

use crate::config_node::ConfigNode;
use crate::node_patch::NodePatch;

pub fn is_satisfied(node: &ConfigNode, patch: &NodePatch) -> bool {
    patch
        .has
        .iter()
        .all(|predicate| evaluate_predicate(node, predicate))
}

pub fn evaluate_predicate(node: &ConfigNode, predicate: &HasPredicate) -> bool {
    true
}

pub fn name_matches(node: &ConfigNode, patch: &NodePatch) -> bool {
    // TODO: surface error for OR-matching at non-top-level.
    match (&patch.target_name, node.name_key()) {
        (Some(targets), Some(name)) => targets.iter().any(|&target| target == name),
        (Some(_), None) => false,
        (None, _) => true,
    }
}
