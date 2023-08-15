pub mod operator;
pub mod patcher;
pub mod searcher;

use std::collections::HashSet;

use crate::database::Database;
use crate::module_manager::patcher::Patcher;
use crate::pass::Pass;
use crate::patch_set::PatchSet;
use crate::raw_patch::RawPatches;
use crate::Result;

pub struct ModuleManager<'a> {
    dll_passes: HashSet<String>,
    patches: PatchSet<'a>,
    database: Database<'a>,
}

impl<'a> ModuleManager<'a> {
    pub fn new(
        raw_patches: RawPatches<'a>,
        dll_names: impl Iterator<Item = &'a str>,
    ) -> Result<Self> {
        Ok(Self {
            dll_passes: dll_names.into_iter().map(ToOwned::to_owned).collect(),
            patches: raw_patches.extract()?,
            database: Database::default(),
        })
    }

    pub fn execute(mut self) -> Result<Database<'a>> {
        let declared_passes = self.scan_declared_passes();
        let all_existing_passes: HashSet<String> = self
            .dll_passes
            .iter()
            .map(AsRef::as_ref)
            .chain(declared_passes)
            .map(ToOwned::to_owned)
            .collect();
        self.prune_before_after(&all_existing_passes);
        self.prune_needs(&all_existing_passes);
        for (pass, files) in self.patches.iter() {
            log::info!("running pass {pass}");
            for file in files {
                for patch in &file.contents {
                    Patcher::new(&mut self.database, file.path.clone(), patch).evaluate()?;
                }
            }
        }
        Ok(self.database)
    }

    fn scan_declared_passes(&'a self) -> impl Iterator<Item = &'a str> {
        log::info!("scanning declared passes");
        self.patches.iter().filter_map(|(pass, _)| {
            if let Pass::For(ident) = pass {
                Some(&*ident.0)
            } else {
                None
            }
        })
    }

    fn prune_before_after(&mut self, declared_passes: &HashSet<String>) {
        self.patches.0.retain(|(pass, _)| match pass {
            // N.B.: the :LAST[ident] passes are not anchored to declared passes, but are merely
            // naked identifiers used for sorting.
            Pass::Default | Pass::First | Pass::For(_) | Pass::Last(_) | Pass::Final => true,
            pass @ (Pass::Before(ident) | Pass::After(ident)) => {
                let exists = declared_passes.contains(&*ident.0);
                if !exists {
                    log::info!("pruning pass {pass}, as :FOR[{ident}] does not exist");
                }
                exists
            }
        })
    }

    fn prune_needs(&mut self, declared_passes: &HashSet<String>) {
        log::info!("evaluating :NEEDS");
        for (_, files) in self.patches.iter_mut() {
            for file in files {
                file.contents
                    .retain_mut(|node| operator::needs::prune_node_recurse(node, declared_passes))
            }
        }
    }
}
