use std::marker::PhantomData;

use crate::config_node::{ConfigNode, NodeList};
use crate::{internal_error, PatchingError, Result};

#[derive(Debug)]
pub struct Searcher<'a, F> {
    needle: F,
    next_idx: usize,
    _phantom: PhantomData<&'a mut ()>,
}

#[derive(Debug)]
#[must_use = "it is a logic error to create an active searcher without resolving it"]
pub struct ActiveSearcher<'a, F>(Searcher<'a, F>);

impl<'a, F> Searcher<'a, F>
where
    F: FnMut(&ConfigNode<'a>) -> bool,
{
    pub fn new(needle: F) -> Self {
        Self {
            needle,
            next_idx: 0,
            _phantom: PhantomData,
        }
    }

    fn idx(&self) -> usize {
        self.next_idx - 1
    }

    pub fn search(
        mut self,
        nodes: &mut NodeList<'a>,
    ) -> Result<Option<(ActiveSearcher<'a, F>, ConfigNode<'a>)>> {
        for node in &mut nodes[self.next_idx..] {
            self.next_idx += 1;
            if (self.needle)(
                node.as_ref().ok_or_else(|| {
                    PatchingError::Internal("tried to search active patcher".into())
                })?,
            ) {
                return Ok(Some((ActiveSearcher(self), node.take().unwrap())));
            }
        }
        Ok(None)
    }

    pub fn insert(&mut self, idx: usize, nodes: &mut NodeList<'a>, node: ConfigNode<'a>) -> Result {
        nodes.insert(idx, Some(node));
        if idx <= self.next_idx {
            self.next_idx += 1;
        }
        Ok(())
    }

    pub fn push(&mut self, nodes: &mut NodeList<'a>, node: ConfigNode<'a>) -> Result {
        nodes.push(Some(node));
        Ok(())
    }
}

impl<'a, F> ActiveSearcher<'a, F>
where
    F: FnMut(&ConfigNode<'a>) -> bool,
{
    pub fn replace(
        self,
        nodes: &mut NodeList<'a>,
        node: ConfigNode<'a>,
    ) -> Result<Searcher<'a, F>> {
        if nodes[self.0.idx()].replace(node).is_some() {
            internal_error("element marked active is not active")?;
        }
        Ok(self.0)
    }

    pub fn delete(self, nodes: &mut NodeList<'a>) -> Result<Searcher<'a, F>> {
        if nodes.remove(self.0.idx()).is_some() {
            internal_error("element marked active is not active")?;
        }
        Ok(self.0)
    }
}
