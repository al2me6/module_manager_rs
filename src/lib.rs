macro_rules! rt_error {
    ($variant:ident$(($($arg:expr),+))? @ $path:expr) => {
        std::result::Result::Err($crate::PatchingError::Runtime {
            path: $path.clone(),
            kind: $crate::RuntimeError::$variant$(($($arg),+))?
        })
    };
}

#[macro_use]
pub mod pass;
pub mod config_node;
pub mod database;
pub mod file;
pub mod key_patch;
pub mod module_manager;
pub mod node_patch;
pub mod operator;
pub mod patch_set;
pub mod raw_patch;
pub mod searcher;

use std::borrow::Cow;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug, thiserror::Error)]
pub enum PatchingError {
    #[error("the parser encountered an internal error: {0}")]
    Internal(Cow<'static, str>),
    #[error("error when evaluating `{path}`: {kind}")]
    Runtime { path: Rc<Path>, kind: RuntimeError },
}

#[derive(Clone, PartialEq, Debug)]
pub enum RuntimeError {
    CannotCopyFromTopLevel,
    PatchInNonPatchNode,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotCopyFromTopLevel => {
                f.write_str("the copy-from operator `#` cannot be used at the top-level")
            }
            Self::PatchInNonPatchNode => {
                f.write_str("a top-level insertion node cannot contain patches")
            }
        }
    }
}

pub(crate) fn internal_error<T>(msg: impl Into<Cow<'static, str>>) -> Result<T> {
    Err(PatchingError::Internal(msg.into()))
}

pub type Result<T = ()> = std::result::Result<T, PatchingError>;
