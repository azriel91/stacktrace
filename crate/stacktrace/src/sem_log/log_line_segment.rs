use std::borrow::Cow;

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
