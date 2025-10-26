use std::ops::{Deref, DerefMut};

/// A number assigned to different [`LogBlock`]s that are semantically related,
/// e.g. having the same package prefix / crate name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupNumber(u32);

impl GroupNumber {
    /// Returns a new [`GroupNumber`] with the given value.
    pub fn new(value: u32) -> Self {
        GroupNumber(value)
    }

    /// Returns the underlying value of the [`GroupNumber`].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl From<u32> for GroupNumber {
    fn from(value: u32) -> Self {
        GroupNumber(value)
    }
}

impl From<GroupNumber> for u32 {
    fn from(group_number: GroupNumber) -> Self {
        group_number.0
    }
}

impl Deref for GroupNumber {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GroupNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
