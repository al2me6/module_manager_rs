use std::borrow::Cow;

#[macro_use]
pub mod pass;
pub mod config_node;
pub mod database;
pub mod file;
pub mod key_patch;
pub mod node_patch;
pub mod operator;
pub mod patch_set;
pub mod patcher;
pub mod raw_patch;

#[derive(Clone, PartialEq, Debug, thiserror::Error)]
pub enum PatchingError {
    #[error("The parser encountered an internal error: `{0}`")]
    Internal(Cow<'static, str>),
    #[error("runtime error")]
    Runtime,
}

pub type Result<T = ()> = std::result::Result<T, PatchingError>;
