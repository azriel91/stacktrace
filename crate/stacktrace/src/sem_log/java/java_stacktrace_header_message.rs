use std::ops::{Deref, DerefMut};

/// A Java identifier which must start with a letter or underscore, and
/// subsequently may contain digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaStacktraceHeaderMessage<'s> {
    /// The raw parsed `JavaStacktraceHeaderMessage`.
    pub inner: crate::log::java::JavaStacktraceHeaderMessage<'s>,
}

impl<'s> Deref for JavaStacktraceHeaderMessage<'s> {
    type Target = crate::log::java::JavaStacktraceHeaderMessage<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaStacktraceHeaderMessage<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
