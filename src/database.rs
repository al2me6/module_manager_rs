use std::collections::LinkedList;

use crate::config_node::ConfigNode;
use crate::file::Files;

#[derive(Clone, Debug, Default)]
pub struct Database<'a>(pub Files<LinkedList<ConfigNode<'a>>>);
