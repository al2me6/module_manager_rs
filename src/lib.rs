macro_rules! rt_error {
    ($variant:ident$(($($arg:expr),+))? @ $path:expr) => {
        ::std::result::Result::Err($crate::PatchingError::Runtime {
            path: ::std::sync::Arc::from(&*$path),
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
pub mod operation;
pub mod patch_set;
pub mod raw_patch;

use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, PartialEq, Debug, thiserror::Error)]
pub enum PatchingError {
    #[error("the parser encountered an internal error: {0}")]
    Internal(Cow<'static, str>),
    #[error("error when evaluating `{path}`: {kind}")]
    Runtime { path: Arc<Path>, kind: RuntimeError },
}

#[derive(Clone, PartialEq, Debug)]
pub enum RuntimeError {
    CannotRenameNode,
    CannotCopyFromTopLevel,
    PatchInNonPatchNode,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotRenameNode => {
                f.write_str("the rename operator `|` cannot be applied to a node")
            }
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
