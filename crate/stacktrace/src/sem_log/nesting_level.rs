use std::ops::{Add, Deref, DerefMut, Sub};

/// The depth at which a [`LogBlock`] is nested within its parent.
///
/// `0` represents the root level, while higher values indicate deeper nesting.
///
/// [`LogBlock`]: crate::sem_log::LogBlock
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NestingLevel(u8);

impl NestingLevel {
    /// Returns a new [`NestingLevel`] with the given value.
    pub const fn new(value: u8) -> Self {
        NestingLevel(value)
    }

    /// Returns the underlying value of the [`NestingLevel`].
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl From<u8> for NestingLevel {
    fn from(value: u8) -> Self {
        NestingLevel(value)
    }
}

impl From<NestingLevel> for u8 {
    fn from(group_number: NestingLevel) -> Self {
        group_number.0
    }
}

impl Deref for NestingLevel {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NestingLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add<u8> for NestingLevel {
    type Output = NestingLevel;

    fn add(self, rhs: u8) -> Self::Output {
        NestingLevel(self.0 + rhs)
    }
}

impl Sub<u8> for NestingLevel {
    type Output = NestingLevel;

    fn sub(self, rhs: u8) -> Self::Output {
        NestingLevel(self.0 - rhs)
    }
}
