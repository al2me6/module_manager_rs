use crate::config_node::ConfigNode;
use crate::{internal_error, Result};

#[derive(Clone, Debug, Default)]
pub struct Database<'a>(pub Vec<Option<ConfigNode<'a>>>);

impl<'a> Database<'a> {
    pub fn insert(&mut self, top_level_node: ConfigNode<'a>) -> Result {
        if !top_level_node.is_top_level() {
            internal_error("attempted to insert top-level-node nor marked as such")?;
        }
        self.0.push(Some(top_level_node));
        Ok(())
    }
}
