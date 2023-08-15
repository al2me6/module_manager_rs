use ksp_cfg_formatter::parser::HasPredicate;

use crate::config_node::ConfigNode;
use crate::node_patch::NodePatch;

pub fn is_satisfied(node: &ConfigNode, patch: &NodePatch) -> bool {
    if let Some(target_name) = patch.target_name {
        match target_name {
            "*" => {}
            target_names => {
                // TODO: OR is supported at the top-level only.
                if !target_names.split('|').any(|name| name == node.ident) {
                    return false;
                }
            }
        }
    }
    patch
        .has
        .iter()
        .all(|predicate| evaluate_predicate(node, predicate))
}

pub fn evaluate_predicate(node: &ConfigNode, predicate: &HasPredicate) -> bool {
    true
}
