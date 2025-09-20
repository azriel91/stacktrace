use std::ops::{Deref, DerefMut};

/// The method name excluding the `()` in a Java stacktrace.
///
/// e.g. `"fail"` in `com.example.stacktrace.Example.fail(Example.java:11)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaMethodName<'s> {
    /// The raw parsed `JavaMethodName`.
    pub inner: crate::log::java::JavaMethodName<'s>,
}

impl<'s> Deref for JavaMethodName<'s> {
    type Target = crate::log::java::JavaMethodName<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaMethodName<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
