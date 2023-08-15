use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Debug, Default)]

pub struct ConfigNode<'a> {
    pub file_path: Option<Rc<Path>>,
    pub ident: &'a str,
    pub nodes: NodeList<'a>,
    pub keys: Vec<ConfigKey<'a>>,
}

pub type NodeList<'a> = Vec<Option<ConfigNode<'a>>>;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ConfigKey<'a> {
    pub ident: &'a str,
    pub value: Cow<'a, str>,
}

impl<'a> ConfigNode<'a> {
    pub fn is_top_level(&self) -> bool {
        self.file_path.is_some()
    }

    pub fn name_key(&self) -> Option<&str> {
        self.keys
            .iter()
            .find(|item| item.ident == "name")
            .map(|item| item.value.as_ref())
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
        for key in &self.keys {
            key.fmt_into(f, indent + 1, indent_size)?;
        }
        for node in &self.nodes {
            node.as_ref()
                .unwrap()
                .fmt_into(f, indent + 1, indent_size)?;
        }
        writeln!(f, "{0:1$}}}", "", indent_size * indent)?;
        Ok(())
    }
}

impl<'a> Display for ConfigNode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_into(f, 0, 4)
    }
}

impl<'a> ConfigKey<'a> {
    pub fn new(ident: &'a str, value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ident,
            value: value.into(),
        }
    }

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

impl<'a> Display for ConfigKey<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_into(f, 0, 4)
    }
}
