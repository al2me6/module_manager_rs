use std::borrow::Cow;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Item<'a, T> {
    pub name: &'a str,
    pub value: T,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NodeContents<'a> {
    pub file_path: Option<Rc<Path>>,
    pub nodes: Vec<Option<ConfigNode<'a>>>,
    pub keys: Vec<ConfigKey<'a>>,
}

pub type ConfigNode<'a> = Item<'a, NodeContents<'a>>;
pub type ConfigKey<'a> = Item<'a, Cow<'a, str>>;

impl<'a, T> Item<'a, T> {
    pub fn new(name: &'a str, value: T) -> Self {
        Self { name, value }
    }
}

impl<'a> ConfigNode<'a> {
    pub fn is_top_level(&self) -> bool {
        self.value.is_top_level()
    }
}

impl<'a> NodeContents<'a> {
    pub fn is_top_level(&self) -> bool {
        self.file_path.is_some()
    }

    pub fn name_key(&self) -> Option<&str> {
        self.keys
            .iter()
            .find(|item| item.name == "name")
            .map(|item| item.value.as_ref())
    }
}
