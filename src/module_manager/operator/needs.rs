use std::collections::HashSet;

use ksp_cfg_formatter::parser::{self, OrClause};

use crate::node_patch::NodePatch;

/// # Returns:
/// Whether this node should be **kept**.
pub fn prune_node_recurse(node: &mut NodePatch, declared_passes: &HashSet<String>) -> bool {
    if !is_satisfied(&node.needs, declared_passes) {
        return false;
    }
    node.node_patches
        .retain_mut(|child| prune_node_recurse(child, declared_passes));
    node.key_patches
        .retain(|child| is_satisfied(&child.needs, declared_passes));
    true
}

pub fn is_satisfied(needs: &[OrClause], declared_passes: &HashSet<String>) -> bool {
    needs.iter().all(|or| {
        or.mod_clauses
            .iter()
            .any(|need| evaluate_mod_need(need, declared_passes))
    })
}

pub fn evaluate_mod_need(need: &parser::ModClause, declared_passes: &HashSet<String>) -> bool {
    let exists = if need.name.contains('/') {
        todo!("subfolder :NEEDS are not yet implemented")
    } else {
        declared_passes.contains(need.name)
    };
    need.negated ^ exists
}
