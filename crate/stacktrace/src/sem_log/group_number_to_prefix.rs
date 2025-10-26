use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::sem_log::{GroupNumber, GroupPrefix};

/// Map of [`GroupNumber`] to [`GroupPrefix`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GroupNumberToPrefix<'s>(BTreeMap<GroupNumber, GroupPrefix<'s>>);

impl<'s> GroupNumberToPrefix<'s> {
    /// Returns a new empty [`GroupNumberToPrefix`] map.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Returns the underlying map of [`GroupNumber`] to [`GroupPrefix`].
    pub fn into_inner(self) -> BTreeMap<GroupNumber, GroupPrefix<'s>> {
        self.0
    }
}

impl<'s> From<BTreeMap<GroupNumber, GroupPrefix<'s>>> for GroupNumberToPrefix<'s> {
    fn from(map: BTreeMap<GroupNumber, GroupPrefix<'s>>) -> Self {
        Self(map)
    }
}

impl<'s> Deref for GroupNumberToPrefix<'s> {
    type Target = BTreeMap<GroupNumber, GroupPrefix<'s>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> DerefMut for GroupNumberToPrefix<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
