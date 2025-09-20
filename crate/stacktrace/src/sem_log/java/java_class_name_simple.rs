use std::ops::{Deref, DerefMut};

/// The simple class name, e.g. `Example$Exception$0` in
/// `com.example.stacktrace.Example$Exception$0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaClassNameSimple<'s> {
    /// The raw parsed `JavaClassNameSimple`.
    pub inner: crate::log::java::JavaClassNameSimple<'s>,
}

impl<'s> Deref for JavaClassNameSimple<'s> {
    type Target = crate::log::java::JavaClassNameSimple<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaClassNameSimple<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
