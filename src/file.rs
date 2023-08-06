use std::path::Path;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Files<T>(pub Vec<FileContents<T>>);

/// Otherwise known as `UrlConfig`.
#[derive(Clone, Debug)]
pub struct FileContents<T> {
    pub path: Rc<Path>,
    pub contents: T,
}

impl<T> Default for Files<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Files<T> {
    pub fn sort(&mut self) {
        // FIXME: compare properly.
        self.0.sort_by(|a, b| a.path.cmp(&b.path));
    }
}

impl<T> IntoIterator for Files<T> {
    type IntoIter = <Vec<FileContents<T>> as IntoIterator>::IntoIter;
    type Item = FileContents<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Files<T> {
    type IntoIter = <&'a Vec<FileContents<T>> as IntoIterator>::IntoIter;
    type Item = &'a FileContents<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Files<T> {
    type IntoIter = <&'a mut Vec<FileContents<T>> as IntoIterator>::IntoIter;
    type Item = &'a mut FileContents<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}