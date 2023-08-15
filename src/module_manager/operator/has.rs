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
