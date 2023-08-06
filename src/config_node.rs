use std::borrow::Cow;
use std::collections::LinkedList;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item<'a, T> {
    pub name: &'a str,
    pub value: T,
}

pub type ConfigNode<'a> = Item<'a, NodeContents<'a>>;
pub type ConfigKey<'a> = Item<'a, Cow<'a, str>>;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NodeContents<'a> {
    pub nodes: LinkedList<ConfigNode<'a>>,
    pub keys: LinkedList<ConfigKey<'a>>,
}

impl<'a> NodeContents<'a> {
    pub fn name_key(&self) -> Option<&str> {
        self.keys
            .iter()
            .find(|item| item.name == "name")
            .map(|item| item.value.as_ref())
    }
}
