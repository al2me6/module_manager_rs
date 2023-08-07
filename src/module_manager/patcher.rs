use std::path::Path;
use std::rc::Rc;

use crate::config_node::{ConfigKey, ConfigNode};
use crate::database::Database;
use crate::node_patch::NodePatch;
use crate::operator::Op;
use crate::searcher::Searcher;
use crate::{operator, Result};

pub struct Patcher<'a, 'b> {
    file_path: Rc<Path>,
    patch: &'b NodePatch<'a>,
    database: &'b mut Database<'a>,
    parents: Vec<usize>,
}

impl<'a, 'b> Patcher<'a, 'b>
where
    'a: 'b,
{
    pub fn new(
        database: &'b mut Database<'a>,
        file_path: Rc<Path>,
        top_level_patch: &'b NodePatch<'a>,
    ) -> Self {
        Self {
            file_path,
            database,
            patch: top_level_patch,
            parents: Vec::new(),
        }
    }

    pub fn evaluate(mut self) -> Result {
        match &self.patch.operation {
            Op::Insert => {
                let mut node = ConfigNode::default();
                evaluate_node_as_pure_data(self.file_path.clone(), self.patch, &mut node)?;
                node.value.file_path = Some(self.file_path.clone());
                // TODO: insertion order.
                self.database.0.push(Some(node));
            }
            Op::Copy => {}
            Op::CopyFrom { .. } => {}
            Op::Edit => evaluate_recurse(
                self.file_path.clone(),
                self.patch,
                &mut self.database.0,
                &mut self.parents,
            )?,
            Op::EditOrCreate => {}
            Op::DefaultValue => {}
            Op::Delete => {}
            Op::Rename => {}
        }
        Ok(())
    }
}

fn evaluate_recurse<'a>(
    file_path: Rc<Path>,
    patch: &NodePatch<'a>,
    nodes: &mut Vec<Option<ConfigNode<'a>>>,
    parents: &mut Vec<usize>,
) -> Result {
    let mut searcher = Searcher::new(nodes, |node| {
        // TODO: indexing
        operator::has::is_satisfied(node, patch)
    });
    while let Some(mut target) = searcher.search()? {
        parents.push(searcher.active_index().unwrap());

        for node_patch in &patch.node_patches {
            evaluate_recurse(
                file_path.clone(),
                node_patch,
                &mut target.value.nodes,
                parents,
            )?;
        }

        for key_patch in &patch.key_patches {
            match &key_patch.operation {
                Op::Insert => {
                    target
                        .value
                        .keys
                        .push(ConfigKey::new(key_patch.ident, key_patch.value.into()));
                }
                Op::Copy => {}
                Op::CopyFrom { path, target } => {}
                Op::Edit => {}
                Op::EditOrCreate => {}
                Op::DefaultValue => {}
                Op::Delete => {}
                Op::Rename => target.name = key_patch.value,
            }
        }

        searcher.replace(target)?;
        parents.pop().unwrap();
    }
    Ok(())
}

fn evaluate_node_as_pure_data<'a>(
    path: Rc<Path>,
    patch: &NodePatch<'a>,
    node: &mut ConfigNode<'a>,
) -> Result {
    if patch.operation != Op::Insert {
        rt_error!(PatchInNonPatchNode @ path)?;
    }

    node.name = patch.ident;

    for key in &patch.key_patches {
        if key.operation != Op::Insert {
            rt_error!(PatchInNonPatchNode @ path)?;
        }
        node.value
            .keys
            .push(ConfigKey::new(key.ident, key.value.into()));
    }

    for child_node_patch in &patch.node_patches {
        let mut child_node = ConfigNode::default();
        evaluate_node_as_pure_data(path.clone(), child_node_patch, &mut child_node)?;
        node.value.nodes.push(Some(child_node));
    }

    Ok(())
}
