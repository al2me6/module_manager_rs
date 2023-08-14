use std::marker::PhantomData;

use crate::config_node::{ConfigNode, NodeList};
use crate::{internal_error, Result};

#[derive(Debug)]
pub struct Searcher<'a, F> {
    needle: F,
    active: bool,
    next_idx: usize,
    _phantom: PhantomData<&'a mut ()>,
}

impl<'a, F> Searcher<'a, F>
where
    F: FnMut(&ConfigNode<'a>) -> bool,
{
    pub fn new(needle: F) -> Self {
        Self {
            needle,
            active: false,
            next_idx: 0,
            _phantom: PhantomData,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    fn idx(&self) -> usize {
        self.next_idx - 1
    }

    pub fn active_index(&self) -> Option<usize> {
        self.active.then_some(self.idx())
    }

    // TODO: return a mutably borrowed `SearchResult` for a safer API.
    pub fn search(&mut self, nodes: &mut NodeList<'a>) -> Result<Option<ConfigNode<'a>>> {
        if self.is_active() {
            internal_error("node is already active")?;
        }
        for node in &mut nodes[self.next_idx..] {
            self.next_idx += 1;
            if (self.needle)(node.as_ref().expect("node is inactive")) {
                self.active = true;
                return Ok(Some(node.take().unwrap()));
            }
        }
        Ok(None)
    }

    pub fn replace(&mut self, nodes: &mut NodeList<'a>, node: ConfigNode<'a>) -> Result {
        if !self.is_active() {
            internal_error("cannot return node to inactive parent")?
        };
        if nodes[self.idx()].replace(node).is_some() {
            internal_error("element marked active is not active")?;
        }
        self.active = false;
        Ok(())
    }

    pub fn delete_active(&mut self, nodes: &mut NodeList<'a>) -> Result {
        if !self.is_active() {
            internal_error("cannot delete node from to inactive parent")?
        };
        if nodes.remove(self.idx()).is_some() {
            internal_error("element marked active is not active")?;
        }
        self.active = false;
        Ok(())
    }

    pub fn insert(&mut self, idx: usize, nodes: &mut NodeList<'a>, node: ConfigNode<'a>) -> Result {
        if self.is_active() {
            internal_error("cannot insert into active parent")?;
        }
        nodes.insert(idx, Some(node));
        if idx <= self.next_idx {
            self.next_idx += 1;
        }
        Ok(())
    }

    pub fn push(&mut self, nodes: &mut NodeList<'a>, node: ConfigNode<'a>) -> Result {
        if self.is_active() {
            internal_error("cannot insert into active parent")?;
        }
        nodes.push(Some(node));
        Ok(())
    }
}
