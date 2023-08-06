use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Item<'a, T> {
    pub name: &'a str,
    pub value: T,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NodeContents<'a> {
    pub nodes: Vec<ConfigNodeOrHole<'a>>,
    pub keys: Vec<ConfigKey<'a>>,
}

pub type ConfigNode<'a> = Item<'a, NodeContents<'a>>;
pub type ConfigNodeOrHole<'a> = Option<ConfigNode<'a>>;
pub type ConfigKey<'a> = Item<'a, Cow<'a, str>>;

impl<'a, T> Item<'a, T> {
    pub fn new(name: &'a str, value: T) -> Self {
        Self { name, value }
    }
}

impl<'a> NodeContents<'a> {
    pub fn name_key(&self) -> Option<&str> {
        self.keys
            .iter()
            .find(|item| item.name == "name")
            .map(|item| item.value.as_ref())
    }
}
