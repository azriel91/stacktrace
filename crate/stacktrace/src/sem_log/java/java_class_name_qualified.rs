use std::ops::{Deref, DerefMut};

/// The qualified class name, e.g.
/// `com.example.stacktrace.Example$Exception$0` in
/// `com.example.stacktrace.Example$Exception$0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaClassNameQualified<'s> {
    /// The raw parsed `JavaClassNameQualified`.
    pub inner: crate::log::java::JavaClassNameQualified<'s>,
}

impl<'s> Deref for JavaClassNameQualified<'s> {
    type Target = crate::log::java::JavaClassNameQualified<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaClassNameQualified<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
