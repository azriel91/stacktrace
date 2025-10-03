use std::{
    borrow::Cow,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::sem_log::LogLineSegmentKind;

/// Represents a segment of a log line.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogLineSegment<'s> {
    /// The text of the log line segment.
    pub text: Cow<'s, str>,
    /// The separator between this segment and the next.
    ///
    /// Note that this may be empty if there is no separator.
    pub separator: Cow<'s, str>,
    /// Whether this segment is unique to this log line or is common with a
    /// parent [`LogBlock`].
    ///
    /// [`LogBlock`]: crate::sem_log::LogBlock
    pub kind: LogLineSegmentKind,
}

impl<'s> LogLineSegment<'s> {
    /// Returns a fully owned version of this [`LogLineSegment`].
    pub fn into_static(&self) -> LogLineSegment<'static> {
        LogLineSegment {
            text: Cow::Owned(self.text.clone().into_owned()),
            separator: Cow::Owned(self.separator.clone().into_owned()),
            kind: self.kind,
        }
    }

    /// Returns a hash of this [`LogBlock`] using the default hasher.
    pub fn hash_with_default_hasher(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
