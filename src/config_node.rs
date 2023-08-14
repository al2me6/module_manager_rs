use std::borrow::Cow;
use std::fmt::Formatter;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Item<'a, T> {
    pub ident: &'a str,
    pub value: T,
}

pub type NodeList<'a> = Vec<Option<ConfigNode<'a>>>;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NodeContents<'a> {
    pub file_path: Option<Rc<Path>>,
    pub nodes: NodeList<'a>,
    pub keys: Vec<ConfigKey<'a>>,
}

pub type ConfigNode<'a> = Item<'a, NodeContents<'a>>;
pub type ConfigKey<'a> = Item<'a, Cow<'a, str>>;

impl<'a, T> Item<'a, T> {
    pub fn new(ident: &'a str, value: T) -> Self {
        Self { ident, value }
    }
}

impl<'a> ConfigNode<'a> {
    pub fn is_top_level(&self) -> bool {
        self.value.is_top_level()
    }

    pub fn fmt_into(
        &self,
        f: &mut Formatter<'_>,
        indent: usize,
        indent_size: usize,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{0:1$}{ident}",
            "",
            indent_size * indent,
            ident = self.ident
        )?;
        writeln!(f, "{0:1$}{{", "", indent_size * indent)?;
        for key in &self.value.keys {
            key.fmt_into(f, indent + 1, indent_size)?;
        }
        for node in &self.value.nodes {
            node.as_ref()
                .unwrap()
                .fmt_into(f, indent + 1, indent_size)?;
        }
        writeln!(f, "{0:1$}}}", "", indent_size * indent)?;
        Ok(())
    }
}

impl<'a> ConfigKey<'a> {
    pub fn fmt_into(
        &self,
        f: &mut Formatter<'_>,
        indent: usize,
        indent_size: usize,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{0:1$}{ident} = {value}",
            "",
            indent_size * indent,
            ident = self.ident,
            value = self.value
        )?;
        Ok(())
    }
}

impl<'a> NodeContents<'a> {
    pub fn is_top_level(&self) -> bool {
        self.file_path.is_some()
    }

    pub fn name_key(&self) -> Option<&str> {
        self.keys
            .iter()
            .find(|item| item.ident == "name")
            .map(|item| item.value.as_ref())
    }
}
