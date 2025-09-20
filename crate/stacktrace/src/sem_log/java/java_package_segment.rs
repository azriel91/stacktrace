use std::ops::{Deref, DerefMut};

/// One package segment, e.g. `example` in `com.example.stacktrace`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaPackageSegment<'s> {
    /// The raw parsed `JavaPackageSegment`.
    pub inner: crate::log::java::JavaPackageSegment<'s>,
}

impl<'s> Deref for JavaPackageSegment<'s> {
    type Target = crate::log::java::JavaPackageSegment<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaPackageSegment<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
