use std::ops::{Deref, DerefMut};

/// A Java identifier which must start with a letter or underscore, and
/// subsequently may contain digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaIdentifier<'s> {
    /// The raw parsed `JavaIdentifier`.
    pub inner: crate::log::java::JavaIdentifier<'s>,
}

impl<'s> Deref for JavaIdentifier<'s> {
    type Target = crate::log::java::JavaIdentifier<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaIdentifier<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
