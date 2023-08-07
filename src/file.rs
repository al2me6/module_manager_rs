use std::path::Path;
use std::rc::Rc;

/// Otherwise known as `UrlConfig`.
#[derive(Clone, Debug)]
pub struct File<T> {
    pub path: Rc<Path>,
    pub contents: T,
}

impl<T> File<T> {
    pub fn new(path: Rc<Path>, contents: T) -> Self {
        Self { path, contents }
    }
}
