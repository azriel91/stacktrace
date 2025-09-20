use std::ops::{Deref, DerefMut};

/// A qualified Java package, e.g. `com.example.stacktrace`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaPackage<'s> {
    /// The raw parsed `JavaPackage`.
    pub inner: crate::log::java::JavaPackage<'s>,
}

impl<'s> Deref for JavaPackage<'s> {
    type Target = crate::log::java::JavaPackage<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaPackage<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
