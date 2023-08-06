use std::path::Path;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Files<T>(pub Vec<File<T>>);

/// Otherwise known as `UrlConfig`.
#[derive(Clone, Debug)]
pub struct File<T> {
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
    type IntoIter = <Vec<File<T>> as IntoIterator>::IntoIter;
    type Item = File<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Files<T> {
    type IntoIter = <&'a Vec<File<T>> as IntoIterator>::IntoIter;
    type Item = &'a File<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Files<T> {
    type IntoIter = <&'a mut Vec<File<T>> as IntoIterator>::IntoIter;
    type Item = &'a mut File<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> File<T> {
    pub fn new(path: Rc<Path>, contents: T) -> Self {
        Self { path, contents }
    }
}
