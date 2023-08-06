use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use ksp_cfg_formatter::parser::{Document, NodeItem};

use crate::file::Files;
use crate::node_patch::NodePatch;
use crate::pass::Pass;
use crate::patch_set::PatchSet;
use crate::{PatchingError, Result};

#[derive(Debug, Default)]
pub struct RawPatches<'a> {
    pub files: Files<Document<'a>>,
}

pub type WorkingPatchSet<'a> = HashMap<Pass<'a>, HashMap<Rc<std::path::Path>, Vec<NodePatch<'a>>>>;

impl<'a> RawPatches<'a> {
    pub fn extract(self) -> Result<PatchSet<'a>> {
        let mut referenced_passes = self.extract_passes()?;
        referenced_passes.extend([Pass::Default]);

        let mut patches: HashMap<Pass, HashMap<Rc<std::path::Path>, Vec<NodePatch>>> =
            referenced_passes
                .into_iter()
                .map(|pass| (pass, HashMap::new()))
                .collect();

        for file in self.files {
            for top_level_item in file.contents.statements {
                match top_level_item {
                    NodeItem::Node(node) => {
                        patches
                            .entry(node.pass.into())
                            .or_default()
                            .entry(Rc::clone(&file.path))
                            .or_default()
                            .push(NodePatch::from_cst(node, true)?);
                    }
                    NodeItem::KeyVal(_) => {
                        Err(PatchingError::Internal("top-level keys are illegal".into()))?
                    }
                    NodeItem::Comment(_) | NodeItem::EmptyLine => {}
                }
            }
        }

        Ok(patches.into())
    }

    fn extract_passes(&self) -> Result<HashSet<Pass<'a>>> {
        let mut passes = HashSet::new();
        for file in &self.files {
            for top_level_item in &file.contents.statements {
                match top_level_item {
                    NodeItem::Node(node) => {
                        passes.insert(node.pass.into());
                    }
                    NodeItem::KeyVal(_) => {
                        Err(PatchingError::Internal("top-level keys are illegal".into()))?
                    }
                    NodeItem::Comment(_) | NodeItem::EmptyLine => {}
                }
            }
        }
        Ok(passes)
    }
}
