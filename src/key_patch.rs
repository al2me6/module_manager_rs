use ksp_cfg_formatter::parser::{self, ArrayIndex, AssignmentOperator, Index, OrClause};

use crate::operation::Op;
use crate::Result;

#[derive(Clone, Debug)]
pub struct KeyPatch<'a> {
    pub operation: Op<'a>,
    pub ident: &'a str,
    pub needs: Vec<OrClause<'a>>,
    pub edit: Option<AssignmentOperator>,
    pub index: Option<Index>,
    pub array_index: Option<ArrayIndex>,
    pub value: &'a str,
}

impl<'a> KeyPatch<'a> {
    pub fn from_cst(key: parser::KeyVal<'a>) -> Result<Self> {
        let operation = Op::new(key.operator, key.path.map(|path| (path, key.key)));
        let edit = (operation == Op::Edit).then_some(key.assignment_operator);
        Ok(Self {
            operation,
            ident: key.key,
            needs: key.needs.map_or_else(Vec::new, |needs| needs.or_clauses),
            edit,
            index: key.index,
            array_index: key.array_index,
            value: key.val,
        })
    }
}
