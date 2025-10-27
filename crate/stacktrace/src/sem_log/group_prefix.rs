use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

/// A prefix used to group [`LogBlock`] semantically.
///
/// [`LogBlock`]: crate::sem_log::LogBlock
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GroupPrefix<'s>(Vec<Cow<'s, str>>);

impl<'s> GroupPrefix<'s> {
    /// Creates a new empty [`GroupPrefix`].
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new [`GroupPrefix`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Returns the underlying `Vec<Cow<'s, str>>`.
    pub fn into_inner(self) -> Vec<Cow<'s, str>> {
        self.0
    }
}

impl<'s> From<Vec<Cow<'s, str>>> for GroupPrefix<'s> {
    fn from(prefix: Vec<Cow<'s, str>>) -> Self {
        Self(prefix)
    }
}

impl<'s> Deref for GroupPrefix<'s> {
    type Target = Vec<Cow<'s, str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> DerefMut for GroupPrefix<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
