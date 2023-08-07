use crate::config_node::ConfigNode;
use crate::{internal_error, Result};

#[derive(Debug)]
pub struct Searcher<'a, 'b, F> {
    nodes: &'b mut Vec<Option<ConfigNode<'a>>>,
    needle: F,
    active: bool,
    idx: usize,
}

impl<'a, 'b, F> Searcher<'a, 'b, F>
where
    'a: 'b,
    F: FnMut(&ConfigNode<'a>) -> bool + 'b,
{
    pub fn new(nodes: &'b mut Vec<Option<ConfigNode<'a>>>, needle: F) -> Self {
        Self {
            nodes,
            needle,
            active: false,
            idx: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn active_index(&self) -> Option<usize> {
        self.active.then_some(self.idx)
    }

    pub fn search(&mut self) -> Result<Option<ConfigNode<'a>>> {
        if self.is_active() {
            internal_error("node is already active")?;
        }
        for node in &mut self.nodes[self.idx..] {
            self.idx += 1;
            if (self.needle)(node.as_ref().expect("node is inactive")) {
                return Ok(Some(node.take().unwrap()));
            }
        }
        Ok(None)
    }

    pub fn replace(&mut self, node: ConfigNode<'a>) -> Result {
        if !self.is_active() {
            internal_error("cannot return node to inactive parent")?
        };
        if self.nodes[self.idx].replace(node).is_some() {
            internal_error("element marked active is not active")?;
        }
        self.active = false;
        Ok(())
    }

    pub fn delete_active(&mut self) -> Result {
        if !self.is_active() {
            internal_error("cannot delete node from to inactive parent")?
        };
        if self.nodes.remove(self.idx).is_some() {
            internal_error("element marked active is not active")?;
        }
        self.active = false;
        Ok(())
    }

    pub fn insert(&mut self, idx: usize, node: ConfigNode<'a>) -> Result {
        if self.is_active() {
            internal_error("cannot insert into active parent")?;
        }
        self.nodes.insert(idx, Some(node));
        if idx <= self.idx {
            self.idx += 1;
        }
        Ok(())
    }

    pub fn push(&mut self, node: ConfigNode<'a>) -> Result {
        if self.is_active() {
            internal_error("cannot insert into active parent")?;
        }
        self.nodes.push(Some(node));
        Ok(())
    }
}
