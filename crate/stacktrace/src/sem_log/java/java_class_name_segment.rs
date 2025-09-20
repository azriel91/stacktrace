use std::ops::{Deref, DerefMut};

/// One class name segment, e.g. `Example`, `Exception`, or `$0` in
/// `com.example.stacktrace.Example$Exception$0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaClassNameSegment<'s> {
    /// The raw parsed `JavaClassNameSegment`.
    pub inner: crate::log::java::JavaClassNameSegment<'s>,
}

impl<'s> Deref for JavaClassNameSegment<'s> {
    type Target = crate::log::java::JavaClassNameSegment<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaClassNameSegment<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
