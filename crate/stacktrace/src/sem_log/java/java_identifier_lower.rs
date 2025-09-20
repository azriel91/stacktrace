use std::ops::{Deref, DerefMut};

/// A Java identifier which must start with a letter or underscore, and
/// subsequently may contain digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaIdentifierLower<'s> {
    /// The raw parsed `JavaIdentifierLower`.
    pub inner: crate::log::java::JavaIdentifierLower<'s>,
}

impl<'s> Deref for JavaIdentifierLower<'s> {
    type Target = crate::log::java::JavaIdentifierLower<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaIdentifierLower<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
