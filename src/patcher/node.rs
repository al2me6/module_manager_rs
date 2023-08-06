use std::path::Path;
use std::rc::Rc;

use crate::config_node::{ConfigKey, ConfigNode};
use crate::database::Database;
use crate::node_patch::NodePatch;
use crate::operator::Op;
use crate::{PatchingError, Result};

impl<'a> super::Patcher<'a> {
    pub(super) fn evaluate_top_level_patch(
        path: Rc<Path>,
        patch: &NodePatch<'a>,
        database: &mut Database<'a>,
    ) -> Result {
        if !patch.is_top_level {
            Err(PatchingError::Internal(
                "top-level node not marked as top-level".into(),
            ))?;
        }

        match &patch.operation {
            Op::Insert => {
                let mut node = ConfigNode::default();
                Self::evaluate_node_as_pure_data(path.clone(), patch, &mut node)?;
                database.insert(path, node);
            }
            Op::Copy => {}
            Op::CopyFrom { .. } => rt_error!(CannotCopyFromTopLevel @ path)?,
            Op::Edit => {}
            Op::EditOrCreate => {}
            Op::DefaultValue => {}
            Op::Delete => {}
            Op::Rename => Err(PatchingError::Internal(
                "rename operator cannot be applied to node".into(),
            ))?,
        }

        Ok(())
    }

    pub(super) fn evaluate_node_as_pure_data(
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
            Self::evaluate_node_as_pure_data(path.clone(), child_node_patch, &mut child_node)?;
            node.value.nodes.push(Some(child_node));
        }

        Ok(())
    }
}
