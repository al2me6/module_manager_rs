use std::fmt::{Display, Formatter};

use crate::config_node::{ConfigKey, ConfigNode, NodeContents, NodeList};
use crate::{internal_error, Result};

#[derive(Clone, Debug, Default)]
pub struct Database<'a>(pub NodeList<'a>);

impl<'a> Database<'a> {
    pub fn insert(&mut self, top_level_node: ConfigNode<'a>) -> Result {
        if !top_level_node.is_top_level() {
            internal_error("attempted to insert top-level-node nor marked as such")?;
        }
        self.0.push(Some(top_level_node));
        Ok(())
    }
}

impl<'a> Display for Database<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in &self.0 {
            let node = node.as_ref().unwrap();
            let wrapper = ConfigNode::new(
                "URL_CONFIG",
                NodeContents {
                    file_path: None,
                    nodes: vec![Some(node.clone())],
                    keys: vec![ConfigKey::new(
                        "parentUrl",
                        node.value
                            .file_path
                            .as_ref()
                            .unwrap()
                            .to_string_lossy()
                            .rsplit_once("GameData")
                            .unwrap()
                            .1
                            .to_owned()
                            .into(),
                    )],
                },
            );
            wrapper.fmt_into(f, 0, 4)?;
        }
        Ok(())
    }
}
