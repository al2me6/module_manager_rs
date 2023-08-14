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
    parents: Vec<ConfigNode<'a>>,
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
                let mut node = evaluate_node_as_pure_data(self.file_path.clone(), self.patch)?;
                node.value.file_path = Some(self.file_path.clone());
                // TODO: insertion order.
                self.database.0.push(Some(node));
            }
            Op::Rename => {
                rt_error!(CannotRenameNode @ self.file_path)?;
            }
            Op::CopyFrom { .. } => {}
            Op::Copy | Op::Edit | Op::Delete | Op::EditOrCreate | Op::DefaultValue => {
                let mut searcher = make_searcher(self.patch);
                while let Some(mut target) = searcher.search(&mut self.database.0)? {
                    match &self.patch.operation {
                        Op::Copy => {
                            let copy = target.clone();
                            searcher.replace(&mut self.database.0, target)?;
                            searcher.push(&mut self.database.0, copy)?;
                            // TODO: run inside of copy.
                        }
                        Op::Edit => {
                            target = self.evaluate_recurse(self.patch, target)?;
                            searcher.replace(&mut self.database.0, target)?;
                        }
                        Op::EditOrCreate => {}
                        Op::DefaultValue => {}
                        Op::Delete => {
                            searcher.delete_active(&mut self.database.0)?;
                        }
                        Op::Insert | Op::Rename | Op::CopyFrom { .. } => unreachable!(),
                    }
                }
            }
        }
        Ok(())
    }

    fn evaluate_recurse(
        &mut self,
        patch: &NodePatch<'a>,
        mut node: ConfigNode<'a>,
    ) -> Result<ConfigNode<'a>> {
        for node_patch in &patch.node_patches {
            let mut searcher = make_searcher(node_patch);
            while let Some(mut target) = searcher.search(&mut node.value.nodes)? {
                match &node_patch.operation {
                    Op::Insert => {
                        // TODO: recurse??
                        let child = evaluate_node_as_pure_data(self.file_path.clone(), node_patch)?;
                        // TODO: insertion order.
                        target.value.nodes.push(Some(child));
                        searcher.replace(&mut node.value.nodes, target)?;
                    }
                    Op::Copy => {}
                    Op::CopyFrom { .. } => {}
                    Op::Edit => {
                        self.parents.push(node);
                        target = self.evaluate_recurse(node_patch, target)?;
                        node = self.parents.pop().unwrap();
                        searcher.replace(&mut node.value.nodes, target)?;
                    }
                    Op::EditOrCreate => {}
                    Op::DefaultValue => {}
                    Op::Delete => {}
                    Op::Rename => {}
                }
            }
        }
        for key_patch in &patch.key_patches {
            match &key_patch.operation {
                Op::Insert => {
                    node.value
                        .keys
                        .push(ConfigKey::new(key_patch.ident, key_patch.value.into()));
                }
                Op::Copy => {}
                Op::CopyFrom { .. } => {}
                Op::Edit => {}
                Op::EditOrCreate => {}
                Op::DefaultValue => {}
                Op::Delete => {}
                Op::Rename => node.ident = key_patch.value,
            }
        }
        Ok(node)
    }
}

fn make_searcher<'a, 'b>(
    patch: &'b NodePatch<'a>,
) -> Searcher<'a, impl FnMut(&ConfigNode<'a>) -> bool + 'b> {
    Searcher::new(|node| {
        // TODO: indexing
        // TODO: name wildcard
        patch.ident == node.ident
            && patch
                .target_name
                .map(|name| Some(name) == node.value.name_key())
                .unwrap_or(true)
            && operator::has::is_satisfied(node, patch)
    })
}

fn evaluate_node_as_pure_data<'a>(path: Rc<Path>, patch: &NodePatch<'a>) -> Result<ConfigNode<'a>> {
    if patch.operation != Op::Insert {
        rt_error!(PatchInNonPatchNode @ path)?;
    }

    let mut node = ConfigNode {
        ident: patch.ident,
        ..Default::default()
    };

    for key in &patch.key_patches {
        if key.operation != Op::Insert {
            rt_error!(PatchInNonPatchNode @ path)?;
        }
        node.value
            .keys
            .push(ConfigKey::new(key.ident, key.value.into()));
    }

    for child_node_patch in &patch.node_patches {
        let child_node = evaluate_node_as_pure_data(path.clone(), child_node_patch)?;
        node.value.nodes.push(Some(child_node));
    }

    Ok(node)
}
