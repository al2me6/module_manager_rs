pub mod needs;

use ksp_cfg_formatter::parser::{self, Path};

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Op<'a> {
    #[default]
    Insert,
    Copy,
    /// Otherwise known as 'paste'.
    CopyFrom {
        path: Path<'a>,
        target: &'a str,
    },
    Edit,
    EditOrCreate,
    DefaultValue,
    Delete,
    Rename,
}

impl<'a> Op<'a> {
    pub fn new(operator: Option<parser::Operator>, copy_from: Option<(Path<'a>, &'a str)>) -> Self {
        match operator.unwrap_or_default() {
            parser::Operator::None => {
                if let Some((path, target)) = copy_from {
                    Self::CopyFrom { path, target }
                } else {
                    Self::Insert
                }
            }
            parser::Operator::Edit => Self::Edit,
            parser::Operator::EditOrCreate => Self::EditOrCreate,
            parser::Operator::CreateIfNotFound => Self::DefaultValue,
            parser::Operator::Copy => Self::Copy,
            parser::Operator::Delete | parser::Operator::DeleteAlt => Self::Delete,
            parser::Operator::Rename => Self::Rename,
        }
    }
}
