use itertools::Itertools;

use crate::file::{FileContents, Files};
use crate::node_patch::NodePatch;
use crate::pass::Pass;
use crate::raw_patch::WorkingPatchSet;

pub type PatchesInPass<'a> = (Pass<'a>, Files<Vec<NodePatch<'a>>>);

#[derive(Clone, Debug, Default)]
pub struct PatchSet<'a>(pub Vec<PatchesInPass<'a>>);

impl<'a> PatchSet<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &PatchesInPass<'a>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PatchesInPass<'a>> {
        self.0.iter_mut()
    }
}

impl<'a> From<WorkingPatchSet<'a>> for PatchSet<'a> {
    fn from(value: WorkingPatchSet<'a>) -> Self {
        let mut passes = value
            .into_iter()
            .map(|(pass, file_contents)| {
                let files = Files(
                    file_contents
                        .into_iter()
                        .map(|(path, contents)| FileContents { path, contents })
                        .collect(),
                );
                (pass, files)
            })
            .collect_vec();
        passes.sort_by(|(a, _), (b, _)| a.cmp(b));
        for (_, files) in &mut passes {
            files.sort();
        }
        Self(passes)
    }
}
