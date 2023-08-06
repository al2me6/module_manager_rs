use std::path::Path;
use std::rc::Rc;

use crate::config_node::{ConfigNode, ConfigNodeOrHole};
use crate::file::File;

#[derive(Clone, Debug, Default)]
pub struct Database<'a>(pub Vec<File<ConfigNodeOrHole<'a>>>);

impl<'a> Database<'a> {
    pub fn insert(&mut self, path: Rc<Path>, top_level_node: ConfigNode<'a>) {
        self.0.push(File::new(path, Some(top_level_node)));
    }
}
