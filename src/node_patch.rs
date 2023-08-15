use ksp_cfg_formatter::parser::{self, HasPredicate, Index, OrClause};

use crate::key_patch::KeyPatch;
use crate::operation::Op;
use crate::Result;

#[derive(Clone, Debug)]
pub struct NodePatch<'a> {
    pub is_top_level: bool,
    pub operation: Op<'a>,
    pub ident: &'a str,
    pub target_name: Option<&'a str>,
    pub has: Vec<HasPredicate<'a>>,
    pub needs: Vec<OrClause<'a>>,
    pub index: Option<Index>,
    pub node_patches: Vec<NodePatch<'a>>,
    pub key_patches: Vec<KeyPatch<'a>>,
}

impl<'a> NodePatch<'a> {
    pub fn from_cst(node: parser::Node<'a>, is_top_level: bool) -> Result<Self> {
        let mut node_patches = vec![];
        let mut key_patches = vec![];
        for item in node.block {
            match item {
                parser::NodeItem::Node(node) => node_patches.push(Self::from_cst(node, false)?),
                parser::NodeItem::KeyVal(key) => key_patches.push(KeyPatch::from_cst(key)?),
                parser::NodeItem::Comment(_) | parser::NodeItem::EmptyLine => {}
            }
        }
        Ok(Self {
            is_top_level,
            operation: Op::new(node.operator, node.path.map(|path| (path, node.identifier))),
            ident: node.identifier,
            target_name: node.name,
            has: node.has.map_or_else(Vec::new, |has| has.predicates),
            needs: node.needs.map_or_else(Vec::new, |needs| needs.or_clauses),
            index: node.index,
            node_patches,
            key_patches,
        })
    }
}
